use crate::command::{Command, ExecutionContext};
use std::io::{self, Write};

pub struct InputCommand {
    var: String
}

impl InputCommand {
    pub fn new(var: String) -> Self {
        InputCommand { var }
    }
}

impl Command for InputCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        // Flush stdout to ensure prompt is displayed before input
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        
        // Remove trailing newline
        input = input.trim().to_string();
        
        // Store the input in the VM's variables
        context.set_variable(self.var.to_string(), input);
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "INPUT"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(InputCommand { var: self.var.clone() })
    }
} 