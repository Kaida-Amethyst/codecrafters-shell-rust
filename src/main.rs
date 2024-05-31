#[allow(unused_imports)]
use std::io::{self, Write};

mod builtin;
use builtin::get_builtin_commands;

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

        let mut input_command = input.splitn(2, char::is_whitespace);
        // get command and args, check if they are none
        let (command, args) = (input_command.next(), input_command.next());

        // Firstlt, check if command is none
        // if command is none, check if it is valid by "builtin_commands.get(command)"
        // if args is none, pass empty string to the function, else pass the args
        if let Some(command) = command {
            match (builtin_commands.get(command), args) {
                (Some(command), Some(args)) => command(args),
                (Some(command), None) => command(""),
                (None, _) => println!("{}: command not found", command),
            }
        }
        io::stdout().flush().unwrap();
    }
}
