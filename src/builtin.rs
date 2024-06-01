use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

fn echo(args: &str) {
    print!("{}", args);
}

fn pwd(args: &str) {
    if args.is_empty() {
        if let Ok(path) = std::env::current_dir() {
            println!("{}", path.display());
        }
    } else {
        println!("pwd: too many arguments");
    }
}

fn cd(args: &str) {
    let args = args.trim();

    match args {
        "" => {
            if let Ok(home) = std::env::var("HOME") {
                if let Err(e) = std::env::set_current_dir(home) {
                    println!("cd: {}", e);
                }
            }
        }
        _ => {
            let new_dir = Path::new(args);
            if let Err(_) = std::env::set_current_dir(&new_dir) {
                println!("{}: No such file or directory", new_dir.display());
            }
        }
    }
}

fn check_command_in_path(command: &str, path: &str) -> bool {
    let path = Path::new(path);
    if let Ok(entries) = fs::read_dir(path) {
        return entries
            .filter_map(Result::ok)
            .any(|entry| entry.file_name() == command);
    };
    false
}

// Find command in env
// Return the path of the command
pub fn find_command_in_env(command: &str) -> Option<String> {
    let env_path = if let Ok(path) = std::env::var("PATH") {
        path
    } else {
        String::new()
    };
    let env_paths: Vec<&str> = env_path.split(':').collect();
    env_paths.iter().find_map(|path| {
        if check_command_in_path(command, path) {
            Some(path.to_string())
        } else {
            None
        }
    })
}

fn command_type(args: &str) {
    let builtin_commands = get_builtin_commands();
    let input_command = args.split_whitespace().next();
    if let Some(command) = input_command {
        match builtin_commands.get(command) {
            Some(_) => println!("{} is a shell builtin", command),
            None => match find_command_in_env(command) {
                Some(path) => println!("{} is {}/{}", command, path, command),
                None => println!("{} not found", command),
            },
        }
    }
}

pub fn get_builtin_commands() -> HashMap<&'static str, fn(&str)> {
    let mut fn_map: HashMap<&'static str, fn(&str)> = HashMap::new();
    fn_map.insert("exit", exit);
    fn_map.insert("echo", echo);
    fn_map.insert("type", command_type);
    fn_map.insert("pwd", pwd);
    fn_map.insert("cd", cd);
    fn_map
}
