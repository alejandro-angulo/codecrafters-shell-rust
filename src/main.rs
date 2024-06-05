use std::io::{self, Write};
use std::str::FromStr;

mod commands;
use commands::die;
use commands::execute_command;
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
                Err(_) => execute_command(parts),
            },
        }
    }
}
