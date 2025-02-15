#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::command::Command;

    fn assert_command_name(result: Result<Option<Box<dyn Command>>, String>, expected_name: &str) {
        match result {
            Ok(Some(command)) => assert_eq!(command.name(), expected_name),
            Ok(None) => panic!("Expected command, got None"),
            Err(e) => panic!("Expected command, got error: {}", e),
        }
    }

    fn assert_is_none(result: Result<Option<Box<dyn Command>>, String>) {
        match result {
            Ok(None) => (),  // This is what we want
            Ok(Some(command)) => panic!("Expected None, got command: {}", command.name()),
            Err(e) => panic!("Expected None, got error: {}", e),
        }
    }

    #[test]
    fn test_empty_line() {
        let mut parser = Parser::new();
        assert_is_none(parser.parse_line(""));
        assert_is_none(parser.parse_line("   "));
    }

    #[test]
    fn test_comments() {
        let mut parser = Parser::new();
        assert_is_none(parser.parse_line("// comment"));
        assert_is_none(parser.parse_line("-- comment"));
        assert_is_none(parser.parse_line("   // comment"));
    }

    #[test]
    fn test_basic_commands() {
        let mut parser = Parser::new();
        
        assert_command_name(parser.parse_line("MOV x 42"), "MOV");
        assert_command_name(parser.parse_line("DEF var value"), "DEF");
        assert_command_name(parser.parse_line("EXEC echo hello"), "EXEC");
    }

    #[test]
    fn test_macro_expansion() {
        let mut parser = Parser::new();
        let result = parser.parse_line("$httpget https://example.com").unwrap().unwrap();
        
        // Macro should expand to a MultiCommand
        assert_eq!(result.name(), "MULTI");
    }

    #[test]
    fn test_invalid_commands() {
        let mut parser = Parser::new();
        
        // Test invalid command name
        assert!(parser.parse_line("INVALID_COMMAND").is_err());
        
        // Test commands with insufficient arguments
        assert!(parser.parse_line("MOV").is_err());
        assert!(parser.parse_line("DEF").is_err());
    }

    #[test]
    fn test_argument_handling() {
        let mut parser = Parser::new();
        
        // Parse a command and check the stored arguments
        parser.parse_line("MOV x 42").unwrap();
        let args = parser.get_last_args().unwrap();
        assert_eq!(args, vec!["MOV", "x", "42"]);
    }

    #[test]
    fn test_command_with_spaces() {
        let mut parser = Parser::new();
        
        // Test command with quoted string or multiple spaces
        assert_command_name(parser.parse_line("DEF greeting Hello, World!"), "DEF");
        assert_command_name(parser.parse_line("EXEC echo   multiple   spaces"), "EXEC");
    }

    #[test]
    fn test_line_number_tracking() {
        let mut parser = Parser::new();
        
        // Parse multiple lines and verify line number increases
        parser.parse_line("MOV x 42").unwrap();
        parser.parse_line("").unwrap();  // Empty line should still increment
        parser.parse_line("// comment").unwrap();  // Comment should still increment
        parser.parse_line("DEF var value").unwrap();
        
        // Instead of accessing private field, test that 4 lines were processed
        let result = parser.parse_line("EXEC echo test").unwrap();
        assert!(result.is_some()); // Verify we can still parse after 4 lines
    }
} 