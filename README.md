# Cereal 🥣

Cereal is an experimental scripting language interpreter written in Rust. This project serves as a learning exercise to understand language implementation and Rust.

## Installation

You can download the latest binary from the [releases page](https://github.com/michaelrausch/cereal/releases).

## Usage

Run a Cereal script:
```bash
./cereal script.cereal
```

If no script is specified, it will look for `script.cereal` in the current directory.

## Language Features

Cereal is a simple scripting language (currently in development) that supports:
- Basic command execution
- Library function calls
- Input/output operations

## Sample Program
```
-- Constants Section
DEF planet Earth
DEF yes y

-- Functions Section    
FN print_hello_world DO
    PRINT Name?
    INPUT name
    PRINT Hello, $name! Welcome to planet $planet!
ENDFN

FN ask_commit_changes DO 
    PRINT Commit changes? (y/n)
ENDFN

-- Main Section
CALL print_hello_world
CALL ask_commit_changes

-- Get user input
INPUT commit_changes

-- Check if the user said yes
IF $commit_changes IS $yes
    -- Library methods can (currently) be called
    -- by using the LIBCALL keyword. Arguments 
    -- are passed in registers.
    MOV r0 commit
    MOV r1 -m "Changes"
    LIBCALL git
ENDIF

-- End of program

```

## Language Syntax

### Constants
```
DEF <name> <value>
```
Constants can be used in expressions by prefixing them with `$`

### Functions 
```
FN <name> DO
    ...
ENDFN

CALL <name>
```

### IF
```
IF <a> IS/NOT/CONTAINS/NOTCONTAINS <b>
    ...
ENDIF
```

Condition must equal TRUE


### EQ / NEQ
```
EQ <left> <right>
NEQ <left> <right>
```

EQ and NEQ store the result in the `eq_result` register.

### MOV
```
MOV <register> <value>
```
Values stored in registers are used as arguments for library / VM calls.

### LIBCALL
```
LIBCALL <name>
```
Calls a library function. You can pass arguments in registers.

### INPUT
```
INPUT <variable>
```
Reads user input and stores it in the specified variable.

### PRINT
```
PRINT <value>
```
Prints a value to the console.

### EXEC
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
