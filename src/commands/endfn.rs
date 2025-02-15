use crate::command::Command;
use crate::command::ExecutionContext;

pub struct EndFnCommand;

impl EndFnCommand {
    pub fn new() -> Self {
        EndFnCommand
    }
}

impl Command for EndFnCommand {
    fn execute(&self, _context: &mut ExecutionContext) -> Result<(), String> {
        // The actual handling of ENDFN is done in the VM during parsing
        Ok(())
    }

    fn name(&self) -> &str {
        "ENDFN"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(Self {})
    }
} 