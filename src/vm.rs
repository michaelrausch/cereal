#![allow(dead_code)]

use crate::command::Command;
use crate::parser::Parser;
use crate::command::ExecutionContext;
use std::collections::HashMap;

pub struct VM {
    commands: Vec<Box<dyn Command>>,
    context: ExecutionContext<'static>,
    parser: Parser,
    #[cfg(test)]
    pub functions: HashMap<String, Vec<String>>,  // Make public for tests
    #[cfg(not(test))]
    functions: HashMap<String, Vec<String>>,
    call_stack: Vec<usize>,
    current_line: usize,
    current_fn: Option<(String, Vec<String>)>,
    registers: HashMap<String, String>,
}

impl VM {
    pub fn new() -> Self {
        // Display boot screen
        Self::display_boot_screen();
        
        VM {
            commands: Vec::new(),
            context: ExecutionContext::new(),
            parser: Parser::new(),
            functions: HashMap::new(),
            call_stack: Vec::new(),
            current_line: 0,
            current_fn: None,
            registers: HashMap::new(),
        }
    }

    fn display_boot_screen() {
        println!(r"
      o8Oo./
   ._o8o8o8Oo_.
    \========/
     `------'  CEREAL VM v0.1.0
     
     ");

     println!("[VM] The VM is ready to go!");
    }

    pub fn execute(&mut self) -> Result<(), String> {
        println!("[VM] Executing loaded instructions\n");

        // First, collect all the commands and their args
        let mut command_data: Vec<(Box<dyn Command>, Vec<String>)> = Vec::new();
        for command in &self.commands {
            let args = self.parser.get_last_args().unwrap_or_default();
            command_data.push((command.box_clone(), args));
        }

        // Now process each command
        for (command, args) in command_data {
            // Clone the state we need
            let vars = self.context.variables.clone();
            let skip = self.context.skip_until.clone();
            
            // Execute the command and get the resulting state
            let (new_vars, new_skip) = {
                let mut context = ExecutionContext::with_vm(self);
                context.variables = vars;
                context.skip_until = skip;
                context.set_args(args);
                
                // Only execute if we're not skipping or if it's a control flow command
                if !context.should_skip(command.name()) || command.is_control_flow() {
                    command.execute(&mut context)?;
                }
                
                (context.variables, context.skip_until)
            }; // context is dropped here, releasing the borrow on self
            
            // Now update VM state
            self.context.variables = new_vars;
            self.context.skip_until = new_skip;
        }
        Ok(())
    }

    /// Adds a command to the VM's command list for later execution.
    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    /// Executes a single instruction line immediately.
    /// Useful for direct command execution outside of normal program flow.
    #[allow(dead_code)]
    pub fn execute_instruction(&mut self, instruction: &str) -> Result<(), String> {
        let mut parser = Parser::new();
        if let Some(command) = parser.parse_line(instruction)? {
            command.execute(&mut self.context)?;
        }
        Ok(())
    }

    /// Loads and parses a script from a string, processing each line.
    /// Returns an error if there are any parsing issues or unclosed function definitions.
    pub fn load_string(&mut self, script: &str) -> Result<(), String> {
        let parser = Parser::new();
        self.parser = parser.clone();
        
        for line in script.lines() {
            let line = line.trim();
            if !line.is_empty() {
                self.process_line(line)?;
            }
        }

        if self.current_fn.is_some() {
            return Err("Unclosed function definition".to_string());
        }

        Ok(())
    }

    /// Processes a single line of code, parsing it into a command and handling it.
    /// Returns an error if parsing or command handling fails.
    fn process_line(&mut self, line: &str) -> Result<(), String> {
        if let Some(command) = self.parser.parse_line(line)? {
            self.handle_command(command, line)?;
        }
        Ok(())
    }

    /// Routes a command to its appropriate handler based on the command name.
    /// Special handling for FN and ENDFN commands, with all others treated as regular commands.
    fn handle_command(&mut self, command: Box<dyn Command>, line: &str) -> Result<(), String> {
        match command.name() {
            "FN" => self.handle_fn_start(),
            "ENDFN" => self.handle_fn_end(),
            _ => self.handle_regular_command(command, line),
        }
    }

    /// Handles the start of a function definition (FN command).
    /// Creates a new function context with the given name and empty body.
    fn handle_fn_start(&mut self) -> Result<(), String> {
        let name = self.parser.get_last_args().unwrap_or_default()[1].clone();
        self.current_fn = Some((name, Vec::new()));
        Ok(())
    }

    /// Handles the end of a function definition (ENDFN command).
    /// Stores the collected function body if there is a matching FN,
    /// otherwise returns an error.
    fn handle_fn_end(&mut self) -> Result<(), String> {
        if let Some((name, body)) = self.current_fn.take() {
            self.define_function(&name, body)
        } else {
            Err("ENDFN without matching FN".to_string())
        }
    }

    /// Handles regular commands (non-FN/ENDFN).
    /// If inside a function definition, adds the command to the function body.
    /// Otherwise, adds it to the main command list for execution.
    fn handle_regular_command(&mut self, command: Box<dyn Command>, line: &str) -> Result<(), String> {
        if let Some((_, ref mut body)) = self.current_fn {
            // If we're in a function definition, add to body
            body.push(line.to_string());
        } else {
            // Otherwise add to normal commands
            self.add_command(command);
        }
        Ok(())
    }

    /// Defines a new function with the given name and body.
    /// Stores the function in the VM's function map for later execution.
    pub fn define_function(&mut self, name: &str, body: Vec<String>) -> Result<(), String> {
        self.functions.insert(name.to_string(), body);
        Ok(())
    }

    /// Calls a previously defined function by name.
    /// Executes each line in the function's body, maintaining the call stack for proper returns.
    /// Returns an error if the function is not found.
    pub fn call_function(&mut self, name: &str) -> Result<(), String> {
        let body = self.functions.get(name).ok_or_else(|| {
            format!("Function '{}' not found", name)
        })?.clone();

        // Save current position
        self.call_stack.push(self.current_line);
        
        // Execute function body
        for line in body {
            if let Some(command) = self.parser.parse_line(&line)? {
                // Check if we should skip this command
                if !self.context.should_skip(command.name()) || command.is_control_flow() {
                    command.execute(&mut self.context)?;
                }
            }
        }

        // Restore position
        if let Some(return_line) = self.call_stack.pop() {
            self.current_line = return_line;
        }

        Ok(())
    }

    pub fn execute_line(&mut self, line: &str) -> Result<(), String> {
        if let Some(command) = self.parser.parse_line(line)? {
            command.execute(&mut self.context)
        } else {
            Ok(())
        }
    }

    pub fn set_register(&mut self, name: &str, value: String) {
        self.registers.insert(name.to_string(), value);
    }

    pub fn get_register(&self, name: &str) -> Option<&String> {
        self.registers.get(name)
    }

    pub fn clear_register(&mut self, name: &str) {
        self.registers.remove(name);
    }

    pub fn clear_all_registers(&mut self) {
        self.registers.clear();
    }
}