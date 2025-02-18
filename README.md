# Cereal 🥣

Cereal is an experimental scripting language interpreter written in Rust. This project serves as a learning exercise to understand language implementation and Rust.

## Installation

You can download the latest binary from the [releases page](https://github.com/michaelrausch/cereal/releases).

## Usage

Run a Cereal script:
```bash
./cereal script.cereal
```

Enter REPL mode:
```bash
./cereal
```

## Language Features

Cereal is a simple scripting language (currently in development) that supports:
- Basic command execution
- Library function calls
- Input/output operations

#### Current Modules:
- `httpget` - HTTP GET request
- `git` - Git commands

## Sample Program
```
-- Constants Section
DEF website "https://mkl.gg"
DEF filename "output.html"

-- Function Section
FN search_website DO
    !httpget $website
    IF $http_get_body NOTCONTAINS $search_term
        ABORT "Website does not contain $search_term"
    ENDIF
    PRINT "Website contains $search_term"
    PRINT "Writing to file..."
    !writef $filename $http_get_body
ENDFN

-- Main Section
PRINT "Enter the search term to check at $website:"
INPUT search_term
CALL search_website
```

### 'Compiled' output:
```
DEF website "https://mkl.gg"
FN search_website DO
    MOV r0 $website
    LIBCALL httpget
    IF $http_get_body NOTCONTAINS $search_term
        ABORT "Website does not contain $search_term"
    ENDIF
    PRINT "Website contains $search_term"
    PRINT "Writing to file..."
    MOV r0 $filename
    MOV r1 $http_get_body
    LIBCALL writef
ENDFN
PRINT "Enter the search term to check at $website:"
INPUT search_term
CALL search_website
```

### Parsed output:
```
Line 1: DEF website "https://mkl.gg" -> Tokens: [Token { token_type: Command, value: "DEF" }, Token { token_type: Identifier, value: "website" }, Token { token_type: String, value: "https://mkl.gg" }]
Line 2: DEF filename "output.html" -> Tokens: [Token { token_type: Command, value: "DEF" }, Token { token_type: Identifier, value: "filename" }, Token { token_type: String, value: "output.html" }]
Line 3: FN search_website DO -> Tokens: [Token { token_type: Command, value: "FN" }, Token { token_type: Identifier, value: "search_website" }, Token { token_type: Identifier, value: "DO" }]
Line 4: !httpget $website -> Tokens: [Token { token_type: Macro, value: "!" }, Token { token_type: Identifier, value: "httpget" }, Token { token_type: Variable, value: "$website" }]
Line 5: MOV r0 $website -> Tokens: [Token { token_type: Command, value: "MOV" }, Token { token_type: Identifier, value: "r0" }, Token { token_type: Variable, value: "$website" }]
Line 6: LIBCALL httpget -> Tokens: [Token { token_type: Command, value: "LIBCALL" }, Token { token_type: Identifier, value: "httpget" }]
Line 7: IF $http_get_body NOTCONTAINS $search_term -> Tokens: [Token { token_type: Command, value: "IF" }, Token { token_type: Variable, value: "$http_get_body" }, Token { token_type: Identifier, value: "NOTCONTAINS" }, Token { token_type: Variable, value: "$search_term" }]
Line 8: ABORT "Website does not contain $search_term" -> Tokens: [Token { token_type: Command, value: "ABORT" }, Token { token_type: String, value: "Website does not contain $search_term" }]
Line 9: ENDIF -> Tokens: [Token { token_type: Command, value: "ENDIF" }]
Line 10: PRINT "Website contains $search_term" -> Tokens: [Token { token_type: Command, value: "PRINT" }, Token { token_type: String, value: "Website contains $search_term" }]
Line 11: PRINT "Writing to file..." -> Tokens: [Token { token_type: Command, value: "PRINT" }, Token { token_type: String, value: "Writing to file..." }]
Line 12: !writef $filename $http_get_body -> Tokens: [Token { token_type: Macro, value: "!" }, Token { token_type: Identifier, value: "writef" }, Token { token_type: Variable, value: "$filename" }, Token { token_type: Variable, value: "$http_get_body" }]
Line 13: MOV r0 $filename -> Tokens: [Token { token_type: Command, value: "MOV" }, Token { token_type: Identifier, value: "r0" }, Token { token_type: Variable, value: "$filename" }]
Line 14: MOV r1 $http_get_body -> Tokens: [Token { token_type: Command, value: "MOV" }, Token { token_type: Identifier, value: "r1" }, Token { token_type: Variable, value: "$http_get_body" }]
Line 15: LIBCALL writef -> Tokens: [Token { token_type: Command, value: "LIBCALL" }, Token { token_type: Identifier, value: "writef" }]
Line 16: ENDFN -> Tokens: [Token { token_type: Command, value: "ENDFN" }]
Line 17: PRINT "Enter the search term to check at $website:" -> Tokens: [Token { token_type: Command, value: "PRINT" }, Token { token_type: String, value: "Enter the search term to check at $website:" }]
Line 18: INPUT search_term -> Tokens: [Token { token_type: Command, value: "INPUT" }, Token { token_type: Identifier, value: "search_term" }]
Line 19: CALL search_website -> Tokens: [Token { token_type: Command, value: "CALL" }, Token { token_type: Identifier, value: "search_website" }]
```

## Language Syntax

#### Constants
```
DEF <name> <value>
```
Constants can be used in expressions by prefixing them with `$`

#### Functions 
```
FN <name> DO
    ...
ENDFN

CALL <name>
```

#### IF
```
IF <a> IS/NOT/CONTAINS/NOTCONTAINS <b>
    ...
ENDIF
```

#### INPUT
```
INPUT <variable>
```
Reads user input and stores it in the specified variable.

#### ABORT
```
ABORT <message>
```
Aborts the program with the specified message.

#### PRINT
```
PRINT <value>
```
Prints a value to the console.

#### EXEC
``` 
EXEC <command>
```
Executes a command on the host machine.


## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## License

This project is open-sourced under the MIT License - see the LICENSE file for details.

## Project Status

⚠️ **Educational Project**: This is primarily a learning experiment and should not be used in production environments.

## Contributing

While this is primarily a learning project, feel free to:
- Open issues for bugs you find
- Suggest improvements
- Submit pull requests
