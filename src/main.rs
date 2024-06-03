use std::env;
use std::fs::read_dir;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
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
        let parts: Vec<&str> = input.split(' ').filter(|s| !s.is_empty()).collect();
        match parts.first() {
            None => {
                continue;
            }
            Some(command) => match Builtins::from_str(command) {
                Ok(command) => match command {
                    Builtins::Exit => {
                        if let Some(status_code) = die(&parts[1..]) {
                            exit(status_code)
                        }
                    }
                    Builtins::Echo => println!("{}", parts[1..].join(" ")),
                    Builtins::Type => println!("{}", type_builtin(parts[1])),
                },
                Err(_) => {
                    let executable_path = match Path::new(command).exists() {
                        true => Some(PathBuf::from(command)),
                        false => find_in_path(command),
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

    ExitCode::from(0)
}

fn find_in_path(command: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();
    for directory in path.split(":") {
        for entry in read_dir(directory)
            .context(format!("Failed to read from {}", directory))
            .unwrap()
        {
            let entry = entry.unwrap();
            if entry.file_name() == command {
                return Some(entry.path());
            }
        }
    }
    None
}

fn type_builtin(command: &str) -> String {
    match Builtins::from_str(command) {
        Ok(_) => format!("{} is a shell builtin", command),
        Err(_) => {
            let path = find_in_path(command);
            if path.is_some() {
                let path = path.unwrap();
                return format!("{} is {}", command, path.display());
            }
            format!("{} not found", command)
        }
    }
}
