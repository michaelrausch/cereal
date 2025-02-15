use std::any::Any;

pub trait Driver: Any + Send + Sync {
    fn name(&self) -> &str;
    fn initialize(&mut self) -> Result<(), String>;
    fn shutdown(&mut self) -> Result<(), String>;
    fn handle_command(&mut self, command: &str, args: &[String]) -> Result<Option<String>, String>;
} 