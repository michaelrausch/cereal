use crate::command::{Command, ExecutionContext};

pub struct PrintCommand {
    cmd: String,
}

impl PrintCommand {
    pub fn new(cmd: String) -> Self {
        Self { cmd }
    }
}

impl Command for PrintCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let expanded_cmd = context.expand_variables(&self.cmd);

        if expanded_cmd.contains("$") {
            let variable_name = expanded_cmd.split('$').collect::<Vec<&str>>();
            let variable_value = context.variables.get(&variable_name[1].to_string());

            if let Some(value) = variable_value {
                println!("{}", value);
            } else {
                return Err(format!("Variable {} not found", variable_name[1]));
            }
        } else {
            println!("{}", expanded_cmd);
        }
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "PRINT"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(PrintCommand::new(self.cmd.clone()))
    }
} 