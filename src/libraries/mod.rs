use crate::command::ExecutionContext;
pub mod git;
pub mod httpget;
pub struct LibraryExecutor {}

impl LibraryExecutor {
    pub fn new() -> Self {
        LibraryExecutor {}
    }

    pub fn execute(&self, name: &str, context: &mut ExecutionContext) -> Result<(), String> {
        match name {
            "git" => git::Git::new().execute(context),
            "httpget" => httpget::HttpGet::new().execute(context),
            _ => Err(format!("Library '{}' not found", name)),
        }
    }
}

