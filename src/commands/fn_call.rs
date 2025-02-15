use crate::command::Command;
use crate::command::ExecutionContext;

pub struct FnCallCommand {
    name: String,
}

impl FnCallCommand {
    pub fn new(name: String) -> Self {
        FnCallCommand { name }
    }
}

impl Command for FnCallCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        context.get_vm().call_function(&self.name)
    }

    fn name(&self) -> &str {
        "CALL"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(Self {
            name: self.name.clone(),
        })
    }
} 