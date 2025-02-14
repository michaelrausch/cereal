#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, TokenType};

    fn assert_tokens(input: &str, expected: Vec<(TokenType, &str)>) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), expected.len(), 
            "Expected {} tokens, got {}", expected.len(), tokens.len());
        
        for (token, (expected_type, expected_value)) in tokens.iter().zip(expected.iter()) {
            assert_eq!(token.token_type, *expected_type,
                "Expected token type {:?}, got {:?}", expected_type, token.token_type);
            assert_eq!(token.value, *expected_value,
                "Expected token value {}, got {}", expected_value, token.value);
        }
    }

    #[test]
    fn test_empty_input() {
        assert_tokens("", vec![]);
        assert_tokens("    ", vec![]);
        assert_tokens("\n\t\r", vec![]);
    }

    #[test]
    fn test_commands() {
        assert_tokens("DEF", vec![(TokenType::Command, "DEF")]);
        assert_tokens("MOV", vec![(TokenType::Command, "MOV")]);
        assert_tokens("EXEC", vec![(TokenType::Command, "EXEC")]);
    }

    #[test]
    fn test_identifiers() {
        assert_tokens("x", vec![(TokenType::Identifier, "x")]);
        assert_tokens("variable_name", vec![(TokenType::Identifier, "variable_name")]);
        assert_tokens("abc123", vec![(TokenType::Identifier, "abc123")]);
    }

    #[test]
    fn test_strings() {
        assert_tokens("\"Hello\"", vec![(TokenType::String, "Hello")]);
        assert_tokens("\"Hello, World!\"", vec![(TokenType::String, "Hello, World!")]);
        assert_tokens("\"Escaped \\\"quote\\\"\"", vec![(TokenType::String, "Escaped \"quote\"")]);
    }

    #[test]
    fn test_variables() {
        assert_tokens("$x", vec![(TokenType::Variable, "$x")]);
        assert_tokens("$var_name", vec![(TokenType::Variable, "$var_name")]);
        assert_tokens("$abc123", vec![(TokenType::Variable, "$abc123")]);
    }

    #[test]
    fn test_macros() {
        assert_tokens("!macro", vec![
            (TokenType::Macro, "!"),
            (TokenType::Identifier, "macro")
        ]);
    }

    #[test]
    fn test_comments() {
        assert_tokens("// This is a comment", vec![]);
        assert_tokens("-- This is also a comment", vec![]);
        assert_tokens("DEF x // with comment", vec![
            (TokenType::Command, "DEF"),
            (TokenType::Identifier, "x")
        ]);
    }

    #[test]
    fn test_complex_line() {
        assert_tokens(
            "DEF greeting \"Hello, $name!\"", 
            vec![
                (TokenType::Command, "DEF"),
                (TokenType::Identifier, "greeting"),
                (TokenType::String, "Hello, $name!")
            ]
        );
    }

    #[test]
    fn test_invalid_string() {
        let mut lexer = Lexer::new("\"Unterminated string");
        assert!(lexer.tokenize().is_err());

        let mut lexer = Lexer::new("\"Invalid \\z escape\"");
        assert!(lexer.tokenize().is_err());
    }

    #[test]
    fn test_multiple_tokens_per_line() {
        assert_tokens(
            "MOV x $value",
            vec![
                (TokenType::Command, "MOV"),
                (TokenType::Identifier, "x"),
                (TokenType::Variable, "$value")
            ]
        );
    }
} 