use std::{process, rc::Rc};

use crate::vm::VM;

pub enum CommandKind {
    Move,
    Calculate,
    Console,
    While,
}

pub trait Command : ToString {
    fn run(&self, vm: &mut VM);
    fn kind(&self) -> CommandKind;
    fn value(&self) -> i32;
}

pub struct MoveCommand(i32);
pub struct CalculateCommand(i32);
pub enum ConsoleCommand {
    PutChar,
    GetChar,
    PutInt,
    GetInt,
}
pub enum WhileCommand {
    While,
    EndWhile,
}

pub fn command_factory(code: &Vec<String>) -> Rc<dyn Command> {
    let command_name = &code[0];
    if command_name == "leftmove" {
        Rc::new(MoveCommand(-code[1].parse::<i32>().unwrap()))
    } else if command_name == "rightmove" {
        Rc::new(MoveCommand(code[1].parse::<i32>().unwrap()))
    } else if command_name == "add" {
        Rc::new(CalculateCommand(code[1].parse::<i32>().unwrap()))
    } else if command_name == "sub" {
        Rc::new(CalculateCommand(-code[1].parse::<i32>().unwrap()))
    } else if command_name == "putchar" {
        Rc::new(ConsoleCommand::PutChar)
    } else if command_name == "getchar" {
        Rc::new(ConsoleCommand::GetChar)
    } else if command_name == "putint" {
        Rc::new(ConsoleCommand::PutInt)
    } else if command_name == "getint" {
        Rc::new(ConsoleCommand::GetInt)
    } else if command_name == "while" {
        Rc::new(WhileCommand::While)
    } else if command_name == "endwhile" {
        Rc::new(WhileCommand::EndWhile)
    } else {
        process::exit(1);
    }
}

impl ToString for MoveCommand {
    fn to_string(&self) -> String {
        if self.0 == 0 {
            String::new()
        } else {
            if self.0 < 0 {
                format!("leftmove {}", self.0.abs())
            } else {
                format!("rightmove {}", self.0)
            }
        }
    }
}

impl Command for MoveCommand {
    fn run(&self, vm: &mut VM) {
        vm.move_pointer(self.0);
    }

    fn kind(&self) -> CommandKind {
        CommandKind::Move
    }

    fn value(&self) -> i32 {
        self.0
    }
}

impl ToString for CalculateCommand {
    fn to_string(&self) -> String {
        if self.0 == 0 {
            String::new()
        } else {
            if self.0 < 0 {
                format!("sub {}", self.0.abs())
            } else {
                format!("add {}", self.0)
            }
        }
    }
}

impl Command for CalculateCommand {
    fn run(&self, vm: &mut VM) {
        vm.calculate(self.0);
    }

    fn kind(&self) -> CommandKind {
        CommandKind::Calculate
    }

    fn value(&self) -> i32 {
        self.0
    }
}

impl ToString for ConsoleCommand {
    fn to_string(&self) -> String {
        match self {
            ConsoleCommand::PutChar => String::from("putchar"),
            ConsoleCommand::GetChar => String::from("getchar"),
            ConsoleCommand::PutInt => String::from("putint"),
            ConsoleCommand::GetInt => String::from("getint"),
        }
    }
}

impl Command for ConsoleCommand {
    fn run(&self, vm: &mut VM) {
        match self {
            ConsoleCommand::PutChar => vm.putchar(),
            ConsoleCommand::GetChar => vm.getchar(),
            ConsoleCommand::PutInt => vm.putint(),
            ConsoleCommand::GetInt => vm.getint(),
        }
    }

    fn kind(&self) -> CommandKind {
        CommandKind::Console
    }

    fn value(&self) -> i32 {
        0
    }
}

impl ToString for WhileCommand {
    fn to_string(&self) -> String {
        match self {
            WhileCommand::While => String::from("while"),
            WhileCommand::EndWhile => String::from("endwhile"),
        }
    }
}

impl Command for WhileCommand {
    fn run(&self, vm: &mut VM) {
        match self {
            WhileCommand::While => vm.start_while(),
            WhileCommand::EndWhile => vm.end_while(),
        }
    }

    fn kind(&self) -> CommandKind {
        CommandKind::While
    }

    fn value(&self) -> i32 {
        0
    }
}