#[allow(unused_imports)]
use std::io::{self, Write};

fn get_valid_commands() -> Vec<&'static str> {
    vec![]
}

fn main() {
    // valid commands
    let valid_commands = get_valid_commands();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Check if the input is a valid command
        let input_command = input.split_whitespace().next().unwrap();
        if let Some(command) = valid_commands.iter().find(|&&c| c == input_command) {
            println!("Command found: {}", command);
        } else {
            println!("{}: command not found", input_command);
        }
        io::stdout().flush().unwrap();
    }
}
