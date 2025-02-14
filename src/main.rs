// Main entry point for the scripting language
mod command;
mod commands;
mod parser;
mod vm;
use vm::VM;
mod libraries;
mod consts;
mod lexer;

use std::env;
use std::fs;
use std::process;
use std::io::{self, Write};

fn run_repl() {
    let mut vm = VM::new();
    println!("Cereal REPL (Press Ctrl+C to exit, type 'RUN' to execute, 'LOAD <filename>' to load a file or 'EXIT' to exit)\n");
    
    let mut buffer = String::new();
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Check if input is "RUN" (ignoring whitespace)
                if input.trim() == "RUN" {
                    // Skip empty accumulated buffer
                    if buffer.trim().is_empty() {
                        buffer.clear();
                        continue;
                    }
                    
                    // Execute accumulated buffer
                    if let Err(e) = vm.load_string(&buffer) {
                        eprintln!("Error: {}", e);
                        buffer.clear();
                        continue;
                    }
                    
                    if let Err(e) = vm.execute() {
                        eprintln!("Error: {}", e);
                        buffer.clear();
                        continue;
                    }
                    
                    // Clear buffer after successful execution
                    buffer.clear();
                } else if input.trim() == "EXIT" {
                    break;
                } else if input.starts_with("LOAD") {
                    let filename = input.split_off(4);
                    let filename = filename.trim();

                    if let Ok(content) = fs::read_to_string(filename) {
                        buffer.push_str(&content);
                        println!("Loaded file '{}'", filename);
                    } else {
                        eprintln!("Error: Failed to load file '{}'", filename);
                    }
                } else {
                    // Accumulate input
                    buffer.push_str(&input);
                }
            }
            Err(_) => continue,
        }
    }
}

fn main() {
    // Get the script file path from command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        // No script file provided, enter REPL mode
        run_repl();
        return;
    }
    
    // ... existing code for script file execution ...
    let script_path = args[1].clone();

    // Read the script file
    let script_content = match fs::read_to_string(&script_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading script file '{}': {}", script_path, e);
            process::exit(1);
        }
    };

    // Create a new Virtual Machine instance
    let mut vm = VM::new();
    
    // Load and execute the script
    if let Err(e) = vm.load_string(&script_content) {
        eprintln!("Error loading script: {}", e);
        process::exit(1);
    }

    // Execute all commands in the VM
    if let Err(e) = vm.execute() {
        eprintln!("Error executing program: {}", e);
        process::exit(1);
    }
}
