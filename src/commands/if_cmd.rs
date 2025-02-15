use crate::command::{Command, ExecutionContext};

pub struct IfCommand {
    expected_value: String,
    condition_var: String,  // Name of the variable to check
    operator: String,
}

impl IfCommand {
    pub fn new(expected_value: String, condition_var: String, operator: String) -> Self {
        Self { expected_value, condition_var, operator }
    }
}

impl Command for IfCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let condition_value = context.expand_variables(&self.condition_var);
        let expected_value = context.expand_variables(&self.expected_value);

        // Clear any existing skip state before evaluating new condition
        context.clear_skip();

        // Set the skip flag in the context based on the condition
        let should_skip = match self.operator.as_str() {
            "IS" => condition_value != expected_value,
            "NOT" => condition_value == expected_value,
            "CONTAINS" => !condition_value.contains(&expected_value),
            "NOTCONTAINS" => condition_value.contains(&expected_value),
            _ => return Err(format!("Unknown operator: {}", self.operator))
        };

        if should_skip {
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
        Box::new(IfCommand::new(self.expected_value.clone(), self.condition_var.clone(), self.operator.clone()))
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