use std::env;
use std::env::current_dir;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::str::FromStr;

pub enum Builtins {
    Exit,
    Echo,
    Type,
    Pwd,
    Cwd,
}

impl FromStr for Builtins {
    type Err = ();

    fn from_str(input: &str) -> Result<Builtins, Self::Err> {
        match input {
            "exit" => Ok(Builtins::Exit),
            "echo" => Ok(Builtins::Echo),
            "type" => Ok(Builtins::Type),
            "pwd" => Ok(Builtins::Pwd),
            "cd" => Ok(Builtins::Cwd),
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

fn find_executable(command: &str) -> Option<PathBuf> {
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

pub fn cwd(input: Vec<&str>) {
    let arg = input.get(1).unwrap_or(&"~");
    let home = env::var("HOME").unwrap_or_default();
    let path = if arg == &"~" { home.as_str() } else { arg };
    if env::set_current_dir(path).is_err() {
        println!("cd: {}: No such file or directory", path);
    }
}

pub fn execute_command(input: Vec<&str>) {
    let command = input.first();
    if command.is_none() {
        return;
    }

    let command = command.unwrap();
    let executable_path = match Path::new(command).exists() {
        true => Some(PathBuf::from(command)),
        false => find_executable(command),
    };

    if executable_path.is_some() {
        let executable_path = executable_path.unwrap();
        Command::new(executable_path)
            .args(&input[1..])
            .status()
            .expect("Failed to execute process");
    } else {
        println!("{}: command not found", command);
    }
}
