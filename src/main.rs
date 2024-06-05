use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
#[allow(unused_imports)]
use std::str::FromStr;

mod commands;
use commands::die;
use commands::find_executable;
use commands::pwd;
use commands::type_builtin;
use commands::Builtins;

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Remove trailing newline
        input.pop();
        let parts: Vec<&str> = input.split(' ').filter(|s| !s.is_empty()).collect();
        match parts.first() {
            None => {
                continue;
            }
            Some(command) => match Builtins::from_str(command) {
                Ok(command) => match command {
                    Builtins::Exit => die(&parts[1..]),
                    Builtins::Echo => println!("{}", parts[1..].join(" ")),
                    Builtins::Type => println!("{}", type_builtin(parts[1])),
                    Builtins::Pwd => println!("{}", pwd()),
                },
                Err(_) => {
                    let executable_path = match Path::new(command).exists() {
                        true => Some(PathBuf::from(command)),
                        false => find_executable(command),
                    };

                    if executable_path.is_some() {
                        let executable_path = executable_path.unwrap();
                        Command::new(executable_path)
                            .args(&parts[1..])
                            .status()
                            .expect("Failed to execute process");
                    } else {
                        println!("{}: command not found", command);
                    }
                }
            },
        }
    }
}
