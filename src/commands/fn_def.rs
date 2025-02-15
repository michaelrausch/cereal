use crate::command::Command;
use crate::command::ExecutionContext;

pub struct FnDefCommand {
    name: String,
    body: Vec<String>,
}

impl FnDefCommand {
    pub fn new(name: String, body: Vec<String>) -> Self {
        FnDefCommand { name, body }
    }
}

impl Command for FnDefCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        println!("Defining function: {}", self.name);
        context.get_vm().define_function(&self.name, self.body.clone())
    }

    fn name(&self) -> &str {
        "FN"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(Self {
            name: self.name.clone(),
            body: self.body.clone(),
        })
    }
} 