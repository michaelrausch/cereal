use crate::command::ExecutionContext;


pub struct HttpGet {}

impl HttpGet {
    pub fn new() -> Self {
        HttpGet {}
    }
}

impl HttpGet {
    pub fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let url = context.variables.get("r0").unwrap();

        // Ensure both command and args are not empty
        if url.is_empty() {
            return Err("Git command or arguments cannot be empty".to_string());
        }

        let output = reqwest::blocking::get(url).unwrap();
        let body = output.text().unwrap();

        context.set_variable("http_get_body".to_string(), body);

        Ok(())
    }
}

