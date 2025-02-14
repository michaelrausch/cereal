use crate::commands::registry::CommandRegistry;
use crate::commands::*;
use crate::command::{Command, MultiCommand};
use crate::lexer::{Lexer, Token, TokenType};
use std::fs::OpenOptions;
use std::io::Write;

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
        
        if line.is_empty() {
            return Ok(None);
        }

        let mut lexer = Lexer::new(line);
        let tokens = lexer.tokenize()?;
        
        // Log tokens to file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("tokens.bin") 
        {
            if self.current_line == 0 {
                writeln!(file, "SOF")
                    .unwrap_or_else(|e| eprintln!("Failed to write SOF: {}", e));
            }
            // Write the value in each token
            writeln!(file, "Line {}: {} -> Tokens: {:?}", 
                self.current_line, line, tokens)
                .unwrap_or_else(|e| eprintln!("Failed to write to tokens.txt: {}", e));
        }

        if tokens.is_empty() {
            return Ok(None);
        }

        // Store tokens for later use
        self.last_args = tokens.iter().map(|t| t.value.clone()).collect();
 
        match &tokens[0].token_type {
            TokenType::Macro => self.parse_macro(&tokens),
            TokenType::Command => self.parse_command(&tokens),
            _ => Err(format!("Expected command or macro, got {:?}", tokens[0].token_type)),
        }
    }

    fn parse_command(&mut self, tokens: &[Token]) -> Result<Option<Box<dyn Command>>, String> {
        let command_name = &tokens[0].value;
        let args: Vec<&str> = tokens[1..]
            .iter()
            .map(|t| t.value.as_str())
            .collect();

        self.registry
            .create_command(command_name, args)
            .map(Some)
    }

    fn parse_macro(&mut self, tokens: &[Token]) -> Result<Option<Box<dyn Command>>, String> {
        if tokens.len() < 2 {
            return Err("Macro requires a name".to_string());
        }

        let macro_name = &tokens[1].value;
        let args: Vec<&str> = tokens[2..]
            .iter()
            .map(|t| t.value.as_str())
            .collect();

        let mut expanded_commands = Vec::new();
        self.generate_mov_commands(&args, &mut expanded_commands)?;
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

    pub fn get_last_args(&self) -> Option<Vec<String>> {
        if self.last_args.is_empty() {
            None
        } else {
            Some(self.last_args.clone())
        }
    }
}
