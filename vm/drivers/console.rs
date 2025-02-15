use crate::vm::driver::Driver;

pub struct ConsoleDriver {
    // Console-specific state
}

impl ConsoleDriver {
    pub fn new() -> Self {
        Self {}
    }
}

impl Driver for ConsoleDriver {
    fn name(&self) -> &str {
        "console"
    }

    fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn handle_command(&mut self, command: &str, args: &[String]) -> Result<Option<String>, String> {
        match command {
            "print" => {
                println!("{}", args.join(" "));
                Ok(None)
            }
            // Add other console commands
            _ => Ok(None),
        }
    }
} 