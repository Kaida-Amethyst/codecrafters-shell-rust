#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;
use builtin::find_command_in_env;
use builtin::get_builtin_commands;
use std::process::Command;

fn execute_program(command: &str, args: &str) {
    let output = Command::new(command).args(args.split_whitespace()).output();

    match output {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn execute_program_in_path(command: &str, args: &str) {
    // Search command in env
    if let Some(path) = find_command_in_env(command) {
        let command = format!("{}/{}", path, command);
        execute_program(&command, args);
    } else {
        println!("Command not found: {}", command);
    }
}

fn main() {
    // valid commands
    let builtin_commands = get_builtin_commands();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // if stdin is empty, continue the loop directly
        if input.trim().is_empty() {
            continue;
        }

        let mut input_command = input.splitn(2, char::is_whitespace);
        // get command and args, check if they are none
        let (command, args) = (input_command.next(), input_command.next());

        // Firstlt, check if command is none
        // if command is none, check if it has '/' in it, if so, execute it directly
        // if not, check if it is valid by "builtin_commands.get(command)"
        // if args is none, pass empty string to the function, else pass the args
        if let Some(command) = command {
            if command.contains("/") {
                execute_program(&command.trim(), args.unwrap_or(""));
            } else {
                match (builtin_commands.get(command), args) {
                    (Some(command), Some(args)) => command(args),
                    (Some(command), None) => command(""),
                    (None, Some(args)) => execute_program_in_path(command, args),
                    (None, None) => println!(""),
                }
            }
        }
        io::stdout().flush().unwrap();
    }
}
