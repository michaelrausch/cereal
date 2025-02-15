use std::process::Command;

use crate::command::ExecutionContext;


pub struct Git {
    name: String,
}

impl Git {
    pub fn new(name: String) -> Self {
        Git { name }
    }
}

impl Git {
    pub fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let command = context.variables.get("r0").unwrap();
        let default_args = String::new();

        // Get the git command from r0 and arguments from r1
        let args = context.variables.get("r1").unwrap_or(&default_args);

        // Ensure both command and args are not empty
        if command.is_empty() {
            return Err("Git command or arguments cannot be empty".to_string());
        }

        // Execute git command using process
        let output = if cfg!(windows) {
            Command::new("cmd")
                .arg("/C")
                .arg("git")
                .arg(command)
                .arg(args)
                .output()
                .map_err(|e| format!("Failed to execute git command: {}", e))?
        } else {
            Command::new("git")
                .arg(command)
                .arg(args) 
                .output()
                .map_err(|e| format!("Failed to execute git command: {}", e))?
        };

        // Handle command output
        if !output.stdout.is_empty() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        if !output.status.success() {
            return Err(format!("Git command failed with exit code: {}", output.status));
        }

        Ok(())
    }
}

