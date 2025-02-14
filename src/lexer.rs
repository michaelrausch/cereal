/// A token representing a single lexical unit in the language
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

/// The different types of tokens that can be recognized
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Command,     // Built-in commands like DEF, MOV, etc.
    Identifier,  // Names/identifiers
    String,      // String literals
    Variable,    // Variables starting with $
    Macro,       // Macros starting with !
    Symbol(char),// Single characters like (, ), etc.
    EOL,         // End of line
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new Lexer instance for the given input string
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the entire input string into a vector of tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        
        Ok(tokens)
    }

    /// Returns the next character without advancing the position
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// Advances the position and returns the next character
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.position += 1;
        self.column += 1;
        Some(c)
    }

    /// Skips over whitespace characters, updating line/column numbers
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
    }

    /// Reads a string literal, handling escape sequences
    fn read_string(&mut self) -> Result<Token, String> {
        let mut value = String::new();
        self.advance(); // Skip opening quote

        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Ok(Token {
                        token_type: TokenType::String,
                        value,
                    });
                }
                '\\' => {
                    self.advance(); // Skip backslash
                    if let Some(next) = self.advance() {
                        value.push(match next {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '"' => '"',
                            '\\' => '\\',
                            _ => return Err(format!("Invalid escape sequence: \\{}", next)),
                        });
                    }
                }
                _ => {
                    value.push(c);
                    self.advance();
                }
            }
        }
        Err("Unterminated string literal".to_string())
    }

    /// Reads an identifier or command token
    fn read_identifier_or_command(&mut self) -> Token {
        let mut value = String::new();
        
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            value.push(c);
            self.advance();
        }

        let token_type = if self.is_command(&value) {
            TokenType::Command
        } else {
            TokenType::Identifier
        };

        Token {
            token_type,
            value,
        }
    }

    /// Checks if a string is a valid command
    fn is_command(&self, value: &str) -> bool {
        // Add all valid commands here
        let commands = [
            "DEF", "MOV", "EXEC", "FN", 
            "CALL", "ENDFN", "INPUT", "LIBCALL", "IF", "ENDIF",
            "PRINT", "ABORT"
        ];
        commands.contains(&value)
    }

    /// Reads a variable token (starting with $)
    fn read_variable(&mut self) -> Token {
        self.advance(); // Skip $
        let mut value = String::new();
        
        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            value.push(c);
            self.advance();
        }

        Token {
            token_type: TokenType::Variable,
            value: "$".to_string() + &value,
        }
    }

    /// Returns the next token from the input
    /// Handles comments, strings, variables, macros, and other tokens
    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        let c = match self.peek() {
            None => return Ok(None),
            Some(c) => c,
        };

        let token = match c {
            '/' if self.input.get(self.position + 1) == Some(&'/') => {
                // Skip comment line
                while let Some(c) = self.peek() {
                    if c == '\n' { break; }
                    self.advance();
                }
                return self.next_token();
            }
            '-' if self.input.get(self.position + 1) == Some(&'-') => {
                // Skip comment line
                while let Some(c) = self.peek() {
                    if c == '\n' { break; }
                    self.advance();
                }
                return self.next_token();
            }
            '"' => self.read_string()?,
            '$' => self.read_variable(),
            '!' => {
                self.advance();
                Token {
                    token_type: TokenType::Macro,
                    value: "!".to_string(),
                }
            }
            c if c.is_alphabetic() => self.read_identifier_or_command(),
            '\n' => {
                self.advance();
                self.line += 1;
                self.column = 1;
                Token {
                    token_type: TokenType::EOL,
                    value: "\n".to_string(),
                }
            }
            c => {
                self.advance();
                Token {
                    token_type: TokenType::Symbol(c),
                    value: c.to_string(),
                }
            }
        };

        Ok(Some(token))
    }
} 