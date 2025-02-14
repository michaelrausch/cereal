// Main entry point for the scripting language
mod command;
mod commands;
mod parser;
mod vm;
use vm::VM;
mod libraries;
mod lexer;

use std::env;
use std::fs;
use std::process;

fn main() {
    // Get the script file path from command line arguments
    let args: Vec<String> = env::args().collect();
    let script_path = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("script.cereal")
    };

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
