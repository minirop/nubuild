use dirs::config_dir;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug, PartialEq, Deserialize)]
struct Command {
    name: String,
    file: String,
    commands: HashMap<String, Vec<String>>,
    #[serde(default = "String::new")]
    separator: String,
    default: String,
}

fn main() {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();

    // if the first arg starts with a dash, it's specific to nubuild
    if args.len() > 0 && args[0].starts_with("-") {
        match args[0].as_str() {
            "-v" | "-V" | "--version" => {
                println!("nubuild {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {}
        }
    }

    let mut paths = vec![PathBuf::from("./")];
    if let Some(mut homepath) = config_dir() {
        homepath.push("nubuild");
        paths.push(homepath);
    }

    for mut path in paths {
        path.push("nubuild.yml");
        if let Some(commands) = load_config_file(path.as_path()) {
            for command in &commands {
                if let Some(named) = args.first() {
                    if command.name == *named {
                        args.remove(0);
                        execute_command(command, args);
                        return;
                    }
                }
            }

            for command in &commands {
                if Path::new(&command.file).exists() {
                    execute_command(command, args);
                    return;
                }
            }
        }
    }

    eprintln!("No command to run.");
}

fn load_config_file(path: &Path) -> Option<Vec<Command>> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            return None;
        }
    };

    let ret: Result<Vec<Command>, serde_yaml::Error> = serde_yaml::from_reader(f);
    match ret {
        Ok(commands) => Some(commands),
        Err(err) => {
            println!("{err:?}");
            None
        }
    }
}

fn execute_command(command: &Command, mut args: Vec<String>) {
    let subcommand = if args.len() > 0 {
        if command.commands.contains_key(&args[0]) {
            args.remove(0)
        } else {
            command.default.clone()
        }
    } else {
        command.default.clone()
    };

    let mut command_data = command.commands[&subcommand].clone();
    let executable = command_data.remove(0);

    let mut arguments = vec![];
    if command_data.len() > 0 {
        arguments.extend(command_data);
    }

    if args.len() > 0 {
        if command.separator.len() > 0 {
            arguments.push(command.separator.clone());
        }
        arguments.extend(args);
    }

    let mut script_command = process::Command::new(executable);
    script_command.args(arguments);
    let output = script_command.output().expect("Can't run command.");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
