use std::process::Command as ProcessCommand;
use crate::command::{Command, ExecutionContext};

pub struct ExecCommand {
    cmd: String,    // Command to execute
}

impl ExecCommand {
    pub fn new(cmd: String) -> Self {
        Self { cmd }
    }
}

impl Command for ExecCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        // Expand variables in the command
        let expanded_cmd = context.expand_variables(&self.cmd);

        // Execute the command using the appropriate shell
        let output = if cfg!(windows) {
            ProcessCommand::new("cmd")
                .arg("/C")
                .arg(&expanded_cmd)
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e))?
        } else {
            ProcessCommand::new("sh")
                .arg("-c")
                .arg(&expanded_cmd)
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e))?
        };

        // Handle command output
        let stdout_value = if !output.stdout.is_empty() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            print!("{}", stdout);
            stdout.to_string()
        } else {
            String::new()
        };
        
        // Always set stdout and status, even if empty
        context.set_variable("exec_stdout".to_string(), stdout_value);
        
        if !output.stderr.is_empty() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        if !output.status.success() {
            // Set the status before returning the error
            context.set_variable("exec_status".to_string(), output.status.to_string());
            return Err(format!("Command failed with exit code: {}", output.status));
        }

        context.set_variable("exec_status".to_string(), output.status.to_string());

        Ok(())
    }

    fn name(&self) -> &'static str {
        "EXEC"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(ExecCommand::new(self.cmd.clone()))
    }
} 