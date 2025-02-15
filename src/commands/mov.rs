use crate::command::{Command, ExecutionContext};

pub struct MovCommand {
    name: String,    // Name of the variable
    value: String,   // Value to assign
}

impl MovCommand {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

impl Command for MovCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        // Expand any variables in the value before assigning
        let expanded_value = context.expand_variables(&self.value);
        context.set_variable(self.name.clone(), expanded_value);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "MOV"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(MovCommand::new(self.name.clone(), self.value.clone()))
    }
} 