use std::process::Command as ProcessCommand;
use crate::command::{Command, ExecutionContext};

pub struct NpmCommand {
    cmd: String,    // NPM command to execute
}

impl NpmCommand {
    pub fn new(cmd: String) -> Self {
        Self { cmd }
    }
}

impl Command for NpmCommand {
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let expanded_cmd = context.expand_variables(&self.cmd);
        let npm_cmd = if cfg!(windows) { "npm.cmd" } else { "npm" };
        ProcessCommand::new(npm_cmd)
            .arg(&expanded_cmd)
            .status()
            .map_err(|e| format!("Failed to execute NPM command: {}", e))?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "NPM"
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(NpmCommand::new(self.cmd.clone()))
    }
} 