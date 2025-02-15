use crate::commands::registry::CommandRegistry;
use crate::commands::*;
use crate::command::{Command, MultiCommand};

#[derive(Clone)]
pub struct Parser {
    current_line: usize,
    registry: CommandRegistry,
    last_args: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        let mut registry = CommandRegistry::new();
        
        // Register the built-in commands
        registry.register("DEF", "DEF", |args| {
            if args.len() < 2 {
                return Err("DEF requires variable name and value".to_string());
            }
            Ok(Box::new(DefCommand::new(
                args[0].to_string(),
                args[1..].join(" "),
            )))
        });

        registry.register("MOV", "MOV", |args| {
            if args.len() < 2 {
                return Err("MOV requires two arguments".to_string());
            }
            Ok(Box::new(MovCommand::new(
                args[0].to_string(),
                args[1..].join(" "),
            )))
        });

        registry.register("EXEC", "EXEC", |args| {
            if args.is_empty() {
                return Err("EXEC requires a command".to_string());
            }
            Ok(Box::new(ExecCommand::new(args.join(" "))))
        });

        registry.register("EQ", "EQ", |args| {
            if args.len() != 2 {
                return Err("EQ requires two arguments".to_string());
            }
            Ok(Box::new(EqCommand::new(args[0].to_string(), args[1].to_string())))
        });

        registry.register("NEQ", "NEQ", |args| {
            if args.len() != 2 {
                return Err("NEQ requires two arguments".to_string());
            }
            Ok(Box::new(NeqCommand::new(args[0].to_string(), args[1].to_string())))
        });

        registry.register("NPM", "NPM", |args| {
            if args.is_empty() {
                return Err("NPM requires a command".to_string());
            }
            Ok(Box::new(NpmCommand::new(args.join(" "))))
        });

        registry.register("FN", "FN", |args| {
            if args.len() < 2 || args[1] != "DO" {
                return Err("Function definition must be in format: FN name DO".to_string());
            }
            Ok(Box::new(FnDefCommand::new(
                args[0].to_string(),
                Vec::new(), // Body will be filled by VM during execution
            )))
        });

        registry.register("CALL", "CALL", |args| {
            if args.is_empty() {
                return Err("CALL requires a function name".to_string());
            }
            Ok(Box::new(FnCallCommand::new(args[0].to_string())))
        });

        registry.register("ENDFN", "ENDFN", |_args| {
            Ok(Box::new(EndFnCommand::new()))
        });
        
        registry.register("INPUT", "INPUT", |args| {
            if args.is_empty() {
                return Err("INPUT requires a variable name".to_string());
            }
            Ok(Box::new(InputCommand::new(args[0].to_string())))
        });

        registry.register("LIBCALL", "LIBCALL", |args| {
            if args.is_empty() {
                return Err("LIBCALL requires a library name".to_string());
            }
            Ok(Box::new(LibCallCommand::new(args[0].to_string())))
        });

        Parser {
            current_line: 0,
            registry,
            last_args: Vec::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str) -> Result<Option<Box<dyn Command>>, String> {
        self.current_line += 1;
        let line = line.trim();
        
        if self.is_empty_or_comment(line) {
            return Ok(None);
        }

        if line.starts_with('!') {
            return self.parse_macro(line);
        }

        self.parse_regular_command(line)
    }

    fn is_empty_or_comment(&self, line: &str) -> bool {
        line.is_empty() || line.starts_with("//") || line.starts_with("--")
    }

    fn parse_macro(&mut self, line: &str) -> Result<Option<Box<dyn Command>>, String> {
        let tokens: Vec<&str> = line[1..].split_whitespace().collect();
        if tokens.is_empty() {
            return Err("Empty macro".to_string());
        }

        let macro_name = tokens[0];
        let args = &tokens[1..];
        
        let mut expanded_commands = Vec::new();
        self.generate_mov_commands(args, &mut expanded_commands)?;
        self.add_libcall_command(macro_name, &mut expanded_commands)?;

        Ok(Some(Box::new(MultiCommand::new(expanded_commands))))
    }

    fn generate_mov_commands(
        &mut self,
        args: &[&str],
        commands: &mut Vec<Box<dyn Command>>
    ) -> Result<(), String> {
        for (i, arg) in args.iter().enumerate() {
            let mov_cmd = format!("MOV r{} {}", i, arg);
            if let Some(command) = self.parse_line(&mov_cmd)? {
                commands.push(command);
            }
        }
        Ok(())
    }

    fn add_libcall_command(
        &mut self,
        macro_name: &str,
        commands: &mut Vec<Box<dyn Command>>
    ) -> Result<(), String> {
        let libcall_cmd = format!("LIBCALL {}", macro_name);
        if let Some(command) = self.parse_line(&libcall_cmd)? {
            commands.push(command);
        }
        Ok(())
    }

    fn parse_regular_command(&mut self, line: &str) -> Result<Option<Box<dyn Command>>, String> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            return Ok(None);
        }

        // Store the args for later use (including the command name)
        self.last_args = tokens.iter().map(|s| s.to_string()).collect();
        
        // Only pass the arguments (without the command name) to create_command
        self.registry
            .create_command(tokens[0], tokens.to_vec())
            .map(Some)
    }

    pub fn get_last_args(&self) -> Option<Vec<String>> {
        if self.last_args.is_empty() {
            None
        } else {
            Some(self.last_args.clone())
        }
    }
}
