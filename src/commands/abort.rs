use crate::command::{Command, ExecutionContext};

pub struct AbortCommand {
    error: String,
}

impl AbortCommand {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

impl Command for AbortCommand {
    fn execute(&self, _context: &mut ExecutionContext) -> Result<(), String> {
        eprintln!("ABORT: {}", self.error);
        std::process::exit(0);
    }
    
    fn name(&self) -> &'static str {
        "ABORT"
    }

    fn is_control_flow(&self) -> bool {
        false
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(AbortCommand::new(self.error.clone()))
    }
}

pub struct EndIfCommand;

impl Command for EndIfCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        // Clear any skip flags
        context.clear_skip();
        Ok(())
    }

    fn name(&self) -> &'static str {
        "ENDIF"
    }

    fn is_control_flow(&self) -> bool {
        true
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(EndIfCommand)
    }
} 