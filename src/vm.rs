#![allow(dead_code)]

use crate::command::Command;
use crate::parser::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::command::ExecutionContext;
use std::collections::HashMap;

pub struct VM {
    commands: Vec<Box<dyn Command>>,
    context: ExecutionContext<'static>,
    parser: Parser,
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

    #[allow(dead_code)]
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = io::BufReader::new(file);
        let mut parser = Parser::new();

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            if let Some(command) = parser.parse_line(&line)? {
                self.add_command(command);
            }
        }
        Ok(())
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    #[allow(dead_code)]
    pub fn execute_instruction(&mut self, instruction: &str) -> Result<(), String> {
        let mut parser = Parser::new();
        if let Some(command) = parser.parse_line(instruction)? {
            command.execute(&mut self.context)?;
        }
        Ok(())
    }

    pub fn load_string(&mut self, script: &str) -> Result<(), String> {
        let mut parser = Parser::new();
        self.parser = parser.clone();
        
        for line in script.lines() {
            let line = line.trim();
            if !line.is_empty() {
                if let Some(command) = parser.parse_line(line)? {
                    match command.name() {
                        "FN" => {
                            // Start collecting function body
                            let name = parser.get_last_args().unwrap_or_default()[1].clone();
                            self.current_fn = Some((name, Vec::new()));
                        }
                        "ENDFN" => {
                            // End function definition and store it
                            if let Some((name, body)) = self.current_fn.take() {
                                self.define_function(&name, body)?;
                            } else {
                                return Err("ENDFN without matching FN".to_string());
                            }
                        }
                        _ => {
                            if let Some((_, ref mut body)) = self.current_fn {
                                // If we're in a function definition, add to body
                                body.push(line.to_string());
                            } else {
                                // Otherwise add to normal commands
                                self.add_command(command);
                            }
                        }
                    }
                }
            }
        }

        if self.current_fn.is_some() {
            return Err("Unclosed function definition".to_string());
        }

        Ok(())
    }

    pub fn define_function(&mut self, name: &str, body: Vec<String>) -> Result<(), String> {
        self.functions.insert(name.to_string(), body);
        Ok(())
    }

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