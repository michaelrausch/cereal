use std::collections::HashMap;
use crate::command::Command;
use crate::commands::{DefCommand, ExecCommand, NpmCommand, IfCommand, EndIfCommand, PrintCommand, AbortCommand};


// Create a wrapper struct that implements Clone
#[derive(Clone)]
struct CloneableFactory {
    #[allow(dead_code)]
    name: String,
    create_fn: fn(Vec<&str>) -> Result<Box<dyn Command>, String>,
}

#[derive(Clone)]
pub struct CommandRegistry {
    factories: HashMap<String, CloneableFactory>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
        };
        
        // Register built-in commands
        registry.register("DEF", "DEF", |args| {
            if args.len() < 2 {
                return Err("DEF requires variable name and value".to_string());
            }
            Ok(Box::new(DefCommand::new(
                args[0].to_string(),
                args[1..].join(" "),
            )))
        });

        registry.register("EXEC", "EXEC", |args| {
            if args.is_empty() {
                return Err("EXEC requires a command".to_string());
            }
            let cmd = args.join(" ");
            println!("Creating EXEC command with: {}", cmd);
            Ok(Box::new(ExecCommand::new(cmd)))
        });

        registry.register("NPM", "NPM", |args| {
            if args.is_empty() {
                return Err("NPM requires a command".to_string());
            }
            Ok(Box::new(NpmCommand::new(args.join(" "))))
        });

        registry.register("IF", "IF", |args| {
            if args.len() != 3 || (args[1] != "IS" && args[1] != "NOT" && args[1] != "CONTAINS" && args[1] != "NOTCONTAINS") {
                return Err("IF requires a condition variable".to_string());
            }
            Ok(Box::new(IfCommand::new(args[2].to_string(), args[0].to_string(), args[1].to_string())))
        });

        registry.register("ENDIF", "ENDIF", |_| {
            Ok(Box::new(EndIfCommand))
        });

        registry.register("ABORT", "ABORT", |args| {
            Ok(Box::new(AbortCommand::new(args.join(" "))))
        });

        registry.register("PRINT", "PRINT", |args| {
            Ok(Box::new(PrintCommand::new(args.join(" "))))
        });
        
        registry
    }

    pub fn register(&mut self, name: &str, factory_name: &str, factory: fn(Vec<&str>) -> Result<Box<dyn Command>, String>) {
        self.factories.insert(
            name.to_uppercase(),
            CloneableFactory {
                name: factory_name.to_string(),
                create_fn: factory,
            },
        );
    }

    pub fn create_command(&self, name: &str, args: Vec<&str>) -> Result<Box<dyn Command>, String> {
        if let Some(factory) = self.factories.get(&name.to_uppercase()) {
            let command_args = args[1..].to_vec();
            (factory.create_fn)(command_args)
        } else {
            Err(format!("Unknown command: {}", name))
        }
    }
} 