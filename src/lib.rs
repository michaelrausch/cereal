#[allow(dead_code)]
pub mod parser;
pub mod lexer;
pub mod command;
pub mod commands;
pub mod libraries;
pub mod vm;
pub mod consts;

#[cfg(test)]
mod parser_test;

#[cfg(test)]
mod vm_test;

#[cfg(test)]
mod lexer_test; 
