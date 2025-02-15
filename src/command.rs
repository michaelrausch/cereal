use std::collections::HashMap;
use std::any::Any;
use crate::vm::VM;

// Base trait for all commands in the scripting language
pub trait Command: Any {
    // Execute the command with given context
    fn execute(&self, context: &mut ExecutionContext) -> Result<(), String>;
    
    // Get the name of the command
    fn name(&self) -> &str;
    
    // Whether this command affects control flow
    fn is_control_flow(&self) -> bool {
        false
    }
    
    // Create a clone of this command
    fn box_clone(&self) -> Box<dyn Command>;
}

// Execution context that holds the current state during command execution
pub struct ExecutionContext<'a> {
    // Variables defined in the current scope
    pub variables: HashMap<String, String>,
    // Current command arguments
    pub args: Vec<String>,
    // Command to skip until (for conditional execution)
    pub skip_until: Option<String>,  // Make this public
    // Value to be returned from current execution
    #[allow(dead_code)]
    return_value: Option<String>,
    // Reference to the VM for advanced operations
    #[allow(dead_code)]
    vm: Option<&'a mut VM>,
}

impl<'a> ExecutionContext<'a> {
    // Create a new empty context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            args: Vec::new(),
            return_value: None,
            vm: None,
            skip_until: None,
        }
    }

    // Create a context with VM reference
    pub fn with_vm(vm: &'a mut VM) -> Self {
        Self {
            variables: HashMap::new(),
            args: Vec::new(),
            return_value: None,
            vm: Some(vm),
            skip_until: None,
        }
    }

    // Set a variable in the current scope
    #[allow(dead_code)]
    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    // Get the current command arguments
    #[allow(dead_code)]
    pub fn get_args(&self) -> Result<Vec<String>, String> {
        Ok(self.args.clone())
    }

    // Set the current command arguments
    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }

    // Set a value to be returned
    #[allow(dead_code)]
    pub fn set_return_value(&mut self, value: String) {
        self.return_value = Some(value);
    }

    // Take and clear the return value
    #[allow(dead_code)]
    pub fn take_return_value(&mut self) -> Option<String> {
        self.return_value.take()
    }

    // Expand variables in a string (e.g., $var becomes the value of var)
    pub fn expand_variables(&self, input: &str) -> String {
        let mut result = input.to_string();

        for (key, value) in &self.variables {
            result = result.replace(&format!("${}", key), value);
        }
        result
    }

    // Get a clone of the current arguments
    pub fn get_current_args(&self) -> Vec<String> {
        self.args.clone()
    }

    // Set which command to skip until
    pub fn set_skip_until(&mut self, command: &str) {
        self.skip_until = Some(command.to_string());
    }

    // Clear the skip flag
    pub fn clear_skip(&mut self) {
        self.skip_until = None;
    }

    // Check if we should skip the current command
    pub fn should_skip(&self, command_name: &str) -> bool {
        if let Some(skip_until) = &self.skip_until {
            if command_name == skip_until {
                return false;  // Don't skip the ending command
            }
            true
        } else {
            false
        }
    }

    // Add these methods to access the VM
    pub fn get_vm(&mut self) -> &mut VM {
        self.vm.as_mut().expect("VM not initialized")
    }
} 