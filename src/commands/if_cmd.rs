use crate::command::{Command, ExecutionContext};

pub struct IfCommand {
    condition_var: String,  // Name of the variable to check
}

impl IfCommand {
    pub fn new(condition_var: String) -> Self {
        // Remove the $ if it exists at the start
        let var_name = if condition_var.starts_with('$') {
            condition_var[1..].to_string()
        } else {
            condition_var
        };
        Self { condition_var: var_name }
    }
}

impl Command for IfCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        // Get the value of the condition variable
        let condition_value = context.variables
            .get(&self.condition_var)
            .cloned()
            .unwrap_or_default();

        // Set the skip flag in the context based on the condition
        if condition_value != "TRUE" {
            context.set_skip_until("ENDIF");
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "IF"
    }

    fn is_control_flow(&self) -> bool {
        true
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(IfCommand::new(self.condition_var.clone()))
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