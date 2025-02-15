use crate::command::{Command, ExecutionContext};

pub struct EqCommand {
    left: String,
    right: String,
}

impl EqCommand {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

fn static_or_variable(value: String, context: &mut ExecutionContext) -> String {
    if value.starts_with('$') {
        let var_name = value[1..].to_string();
        context.variables.get(&var_name).cloned().unwrap_or_default()
    } else {
        value
    }
}

impl Command for EqCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let var = "eq_result";

        let left_val = static_or_variable(self.left.clone(), context);
        let right_val = static_or_variable(self.right.clone(), context);

        // If left and right are equal, set output_var to TRUE, otherwise set to FALSE
        if left_val == right_val {
            context.set_variable(var.to_string(), "TRUE".to_string());
        } else {
            context.set_variable(var.to_string(), "FALSE".to_string());
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "EQ"
    }

    fn is_control_flow(&self) -> bool {
        false
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(EqCommand::new(self.left.clone(), self.right.clone()))
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