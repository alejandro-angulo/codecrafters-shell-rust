use std::env;
use std::env::current_dir;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

pub enum Builtins {
    Exit,
    Echo,
    Type,
    Pwd,
}

impl FromStr for Builtins {
    type Err = ();

    fn from_str(input: &str) -> Result<Builtins, Self::Err> {
        match input {
            "exit" => Ok(Builtins::Exit),
            "echo" => Ok(Builtins::Echo),
            "type" => Ok(Builtins::Type),
            "pwd" => Ok(Builtins::Pwd),
            // TODO: Is this the "right" thing to do for errors?
            _ => Err(()),
        }
    }
}
pub fn die(arguments: &[&str]) {
    if arguments.len() > 1 {
        eprintln!("exit: too many arguments");
        return;
    }
    let mut status_code: i32 = 0;
    if arguments.len() == 1 {
        status_code = arguments[0].parse::<i32>().unwrap();
    }
    exit(status_code);
}

pub fn find_executable(command: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();
    for directory in path.split(':') {
        let entries = read_dir(directory);
        if entries.is_err() {
            // TODO: Call this out somehow?
            continue;
        }
        for entry in entries.unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == command {
                return Some(entry.path());
            }
        }
    }
    None
}

pub fn type_builtin(command: &str) -> String {
    match Builtins::from_str(command) {
        Ok(_) => format!("{} is a shell builtin", command),
        Err(_) => {
            let path = find_executable(command);
            if path.is_some() {
                let path = path.unwrap();
                return format!("{} is {}", command, path.display());
            }
            format!("{} not found", command)
        }
    }
}
pub fn pwd() -> String {
    let current_dir = current_dir().unwrap();
    current_dir.display().to_string()
}
