use std::{fs::File, io::{BufWriter, Write}, process};

use crate::{CREATE_FILE_FAILURE, command::CommandKind, loader::Loader};

enum OptimizedCommand {
    Move(i32),
    Calculate(i32),
    Empty,
}

impl OptimizedCommand {
    fn modify(&mut self, value: i32) {
        match self {
            OptimizedCommand::Move(n) => *n += value,
            OptimizedCommand::Calculate(n) => *n += value,
            OptimizedCommand::Empty => {}
        }
    }
}

impl ToString for OptimizedCommand {
    fn to_string(&self) -> String {
        let command_name: String;
        let number: u32;
        match self {
            OptimizedCommand::Move(n) => {
                if n == &0 {
                    command_name = String::new();
                } else {
                    if n < &0 {
                        command_name = String::from("leftmove");
                    } else {
                        command_name = String::from("rightmove");
                    }
                }
                number = n.abs() as u32;
            }
            OptimizedCommand::Calculate(n) => {
                if n == &0 {
                    command_name = String::new();
                } else {
                    if n < &0 {
                        command_name = String::from("sub");
                    } else {
                        command_name = String::from("add");
                    }
                }
                number = n.abs() as u32;
            }
            OptimizedCommand::Empty => {
                command_name = String::new();
                number = 0;
            }
        }
        if !command_name.is_empty() {
            format!("{} {}", command_name, number)
        } else {
            String::new()
        }
    }
}

pub fn optimize(filename: &String) {
    let loader = Loader::new(filename);
    let mut optimized_command = OptimizedCommand::Empty;
    let file = File::create(filename);
    if file.is_err() {
        eprintln!("[Error] Cannot generate file {}", filename);
        process::exit(CREATE_FILE_FAILURE);
    }
    let file = file.unwrap();
    let mut writer = BufWriter::new(file);

    for command in loader.commands() {
        match command.kind() {
            CommandKind::Move => {
                match &optimized_command {
                    OptimizedCommand::Move(_) => {
                        optimized_command.modify(command.value())
                    }
                    OptimizedCommand::Calculate(_) => {
                        writer.write(optimized_command.to_string().as_bytes()).unwrap();
                        writer.write("\n".as_bytes()).unwrap();
                        optimized_command = OptimizedCommand::Move(command.value());
                    }
                    OptimizedCommand::Empty => {
                        optimized_command = OptimizedCommand::Move(command.value());
                    }
                }
            }
            CommandKind::Calculate => {
                match &optimized_command {
                    OptimizedCommand::Move(_) => {
                        writer.write(optimized_command.to_string().as_bytes()).unwrap();
                        writer.write("\n".as_bytes()).unwrap();
                        optimized_command = OptimizedCommand::Calculate(command.value());
                    }
                    OptimizedCommand::Calculate(_) => {
                        optimized_command.modify(command.value());
                    }
                    OptimizedCommand::Empty => {
                        optimized_command = OptimizedCommand::Calculate(command.value());
                    }
                }
            }
            _ => {
                writer.write(optimized_command.to_string().as_bytes()).unwrap();
                writer.write("\n".as_bytes()).unwrap();
                optimized_command = OptimizedCommand::Empty;
                writer.write(command.to_string().as_bytes()).unwrap(); 
                writer.write("\n".as_bytes()).unwrap();
            }
        }
    }
    writer.write(optimized_command.to_string().as_bytes()).unwrap();
}
