mod kernel;
mod driver;
pub mod drivers;

use kernel::Kernel;
use driver::Driver;
use std::collections::HashMap;
// ... existing imports ...

pub struct VM {
    kernel: Kernel,
    drivers: HashMap<String, Box<dyn Driver>>,
    commands: Vec<Box<dyn Command>>,
    context: ExecutionContext<'static>,
    parser: Parser,
}

impl VM {
    pub fn new() -> Self {
        Self::display_boot_screen();
        
        let mut vm = VM {
            kernel: Kernel::new(),
            drivers: HashMap::new(),
            commands: Vec::new(),
            context: ExecutionContext::new(),
            parser: Parser::new(),
        };
        
        // Register default drivers
        vm.register_default_drivers();
        vm
    }

    fn register_default_drivers(&mut self) {
        use drivers::{console::ConsoleDriver, filesystem::FilesystemDriver};
        
        self.register_driver("console", Box::new(ConsoleDriver::new()));
        self.register_driver("fs", Box::new(FilesystemDriver::new()));
    }

    pub fn register_driver(&mut self, name: &str, driver: Box<dyn Driver>) {
        self.drivers.insert(name.to_string(), driver);
    }

    // ... rest of existing code ...

    pub fn execute(&mut self) -> Result<(), String> {
        // Update execute to work through kernel
        self.kernel.execute_commands(&self.commands, &mut self.context, &mut self.drivers)
    }
} 