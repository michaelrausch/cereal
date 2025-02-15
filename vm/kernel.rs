use crate::command::{Command, ExecutionContext};
use crate::vm::driver::Driver;
use std::collections::HashMap;

pub struct Kernel {
    // Kernel-specific state
    interrupt_handlers: HashMap<u32, Box<dyn Fn(&mut ExecutionContext) -> Result<(), String>>>,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            interrupt_handlers: HashMap::new(),
        }
    }

    pub fn execute_commands(
        &mut self,
        commands: &[Box<dyn Command>],
        context: &mut ExecutionContext,
        drivers: &mut HashMap<String, Box<dyn Driver>>,
    ) -> Result<(), String> {
        for command in commands {
            self.execute_single_command(command, context, drivers)?;
        }
        Ok(())
    }

    fn execute_single_command(
        &mut self,
        command: &Box<dyn Command>,
        context: &mut ExecutionContext,
        drivers: &mut HashMap<String, Box<dyn Driver>>,
    ) -> Result<(), String> {
        // Handle command execution with proper driver access
        if !context.should_skip(command.name()) || command.is_control_flow() {
            // Give commands access to drivers if needed
            context.set_drivers(drivers);
            command.execute(context)?;
        }
        Ok(())
    }

    pub fn register_interrupt(&mut self, interrupt_number: u32, handler: Box<dyn Fn(&mut ExecutionContext) -> Result<(), String>>) {
        self.interrupt_handlers.insert(interrupt_number, handler);
    }
} 