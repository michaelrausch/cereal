use crate::command::{Command, ExecutionContext};

pub struct NeqCommand {
    left: String,
    right: String,
}

impl NeqCommand {
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

impl Command for NeqCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let var = "eq_result";

        let left_val = static_or_variable(self.left.clone(), context);
        let right_val = static_or_variable(self.right.clone(), context);

        // If left and right are equal, set output_var to TRUE, otherwise set to FALSE
        if left_val != right_val {
            context.set_variable(var.to_string(), "TRUE".to_string());
        } else {
            context.set_variable(var.to_string(), "FALSE".to_string());
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "NEQ"
    }

    fn is_control_flow(&self) -> bool {
        false
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(NeqCommand::new(self.left.clone(), self.right.clone()))
    }
}