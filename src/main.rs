use std::io::{self, Write};
#[allow(unused_imports)]
use std::str::FromStr;

enum Builtins {
    Exit,
    Echo,
    Type,
}

impl FromStr for Builtins {
    type Err = ();

    fn from_str(input: &str) -> Result<Builtins, Self::Err> {
        match input {
            "exit" => Ok(Builtins::Exit),
            "echo" => Ok(Builtins::Echo),
            "type" => Ok(Builtins::Type),
            // TODO: Is this the "right" thing to do for errors?
            _ => Err(()),
        }
    }
}

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
        let parts: Vec<&str> = input.split(" ").filter(|s| !s.is_empty()).collect();
        match parts.first() {
            None => {
                continue;
            }
            Some(command) => match Builtins::from_str(command) {
                Ok(command) => match command {
                    Builtins::Exit => break,
                    Builtins::Echo => println!("{}", parts[1..].join(" ")),
                    Builtins::Type => println!("{}", type_builtin(parts[1])),
                },
                Err(_) => {
                    println!("{}: command not found", command);
                }
            },
        }
    }
}

fn type_builtin(command: &str) -> String {
    match Builtins::from_str(command) {
        Ok(_) => format!("{} is a shell builtin", command),
        Err(_) => format!("{} not found", command),
    }
}
