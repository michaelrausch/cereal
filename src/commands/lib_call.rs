use crate::command::Command;
use crate::command::ExecutionContext;
use crate::libraries::LibraryExecutor;

pub struct LibCallCommand {
    name: String,
}

impl LibCallCommand {
    pub fn new(name: String) -> Self {
        LibCallCommand { name }
    }
}

impl Command for LibCallCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let _ = LibraryExecutor::new().execute(&self.name, context);
        Ok(())
    }

    fn name(&self) -> &str {
        "LIBCALL"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(Self {
            name: self.name.clone(),
        })
    }
} 