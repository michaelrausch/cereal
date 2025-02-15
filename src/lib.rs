#[allow(dead_code)]
pub mod parser;
pub mod command;
pub mod commands;
pub mod libraries;
pub mod vm;

#[cfg(test)]
mod parser_test;

#[cfg(test)]
mod vm_test; 
