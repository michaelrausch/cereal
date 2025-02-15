use crate::vm::VM;

#[test]
fn test_basic_function_definition() {
    let mut vm = VM::new();
    let script = r#"
        FN test_func DO
            PRINT "Hello from function"
        ENDFN
    "#;
    
    assert!(vm.load_string(script).is_ok());
    assert!(vm.functions.contains_key("test_func"));
}

#[test]
fn test_nested_function_error() {
    let mut vm = VM::new();
    let script = r#"
        FN outer
            FN inner
            ENDFN
        ENDFN
    "#;
    
    assert!(vm.load_string(script).is_err());
}

#[test]
fn test_unclosed_function() {
    let mut vm = VM::new();
    let script = r#"
        FN test_func
            PRINT "This function never ends"
    "#;
    
    assert!(vm.load_string(script).is_err());
}

#[test]
fn test_endfn_without_fn() {
    let mut vm = VM::new();
    let script = "ENDFN";
    
    assert!(vm.load_string(script).is_err());
}

#[test]
fn test_function_body_capture() {
    let mut vm = VM::new();
    let script = r#"
        FN test_func DO
            PRINT Line 1
            PRINT Line 2
        ENDFN
    "#;
    
    assert!(vm.load_string(script).is_ok());
    
    if let Some(body) = vm.functions.get("test_func") {
        assert_eq!(body.len(), 2);
        assert!(body[0].contains("Line 1"));
        assert!(body[1].contains("Line 2"));
    } else {
        panic!("Function not found");
    }
}

#[test]
fn test_register_operations() {
    let mut vm = VM::new();
    
    // Test setting and getting registers
    vm.set_register("test", "value".to_string());
    assert_eq!(vm.get_register("test"), Some(&"value".to_string()));
    
    // Test clearing single register
    vm.clear_register("test");
    assert_eq!(vm.get_register("test"), None);
    
    // Test clearing all registers
    vm.set_register("test1", "value1".to_string());
    vm.set_register("test2", "value2".to_string());
    vm.clear_all_registers();
    assert_eq!(vm.get_register("test1"), None);
    assert_eq!(vm.get_register("test2"), None);
}

#[test]
fn test_empty_script() {
    let mut vm = VM::new();
    let script = "";
    
    assert!(vm.load_string(script).is_ok());
}

#[test]
fn test_whitespace_only_script() {
    let mut vm = VM::new();
    let script = "    \n    \t    \n";
    
    assert!(vm.load_string(script).is_ok());
}

#[test]
fn test_execute_line() {
    let mut vm = VM::new();
    
    // Test valid command
    assert!(vm.execute_line("PRINT \"test\"").is_ok());
    
    // Test empty line
    assert!(vm.execute_line("").is_ok());
} 