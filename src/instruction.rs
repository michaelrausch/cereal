#[derive(Debug, Clone)]
pub enum Instruction {
    Def(String, String),  // variable name, value
    Exec(String),         // command to execute
    If(String),          // condition
    Else,
    Npm(String),         // npm command
} 