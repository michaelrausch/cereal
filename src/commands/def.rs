use crate::command::{Command, ExecutionContext};

pub struct DefCommand {
    name: String,    // Name of the variable
    value: String,   // Value to assign
}

impl DefCommand {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

impl Command for DefCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        // Expand any variables in the value before assigning
        let expanded_value = context.expand_variables(&self.value);
        context.variables.insert(self.name.clone(), expanded_value);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "DEF"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(DefCommand::new(self.name.clone(), self.value.clone()))
    }
} 