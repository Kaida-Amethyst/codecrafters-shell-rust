#[allow(unused_imports)]
use std::io::{self, Write};

use std::collections::HashMap;

fn exit(args: &str) {
    if args.is_empty() {
        std::process::exit(0);
    }
    // parse args to int
    if let Some(code) = args.parse::<i32>().ok() {
        std::process::exit(code);
    }
    std::process::exit(0);
}

fn get_builtin_commands() -> HashMap<&'static str, fn(&str)> {
    let mut fn_map: HashMap<&'static str, fn(&str)> = HashMap::new();
    fn_map.insert("exit", exit);
    fn_map
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

        let mut input_command = input.split_whitespace();
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
