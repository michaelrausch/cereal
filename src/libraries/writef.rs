use std::fs::OpenOptions;
use std::io::Write;

use crate::command::ExecutionContext;
use crate::consts::Registers;


pub struct WriteF {}

impl WriteF {
    pub fn new() -> Self {
        WriteF {}
    }
}

impl WriteF {
    pub fn execute(&self, context: &mut ExecutionContext) -> Result<(), String> {
        let filename = context.variables.get(Registers::R0).unwrap();
        let data = context.variables.get(Registers::R1).unwrap();

        if filename.is_empty() || data.is_empty() {
            return Err("Filename or data cannot be empty".to_string());
        }

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename) 
        {
            writeln!(file, "{}", data).unwrap();
        }

        Ok(())
    }
}

