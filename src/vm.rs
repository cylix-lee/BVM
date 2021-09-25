use std::{
    io::{self, Read},
    process,
};

use crate::{config::Config, loader::Loader, MOVE_OUT_OF_BOUND};

struct WhileCondition {
    pub command_index: u32,
    pub condition_index: u32,
    pub ignoring: bool,
}

impl WhileCondition {
    fn new(command_index: u32, condition_index: u32) -> Self {
        Self {
            command_index: command_index,
            condition_index,
            ignoring: false,
        }
    }

    fn new_ignoring(command_index: u32, condition_index: u32) -> Self {
        Self {
            command_index,
            condition_index,
            ignoring: true,
        }
    }
}

pub struct VM {
    memory: Vec<i32>,
    while_stack: Vec<WhileCondition>,
    allocate_threshold: usize,
    memory_index: u32,
    command_index: u32,
}

impl VM {
    pub fn new(config: &Config) -> Self {
        let memory = vec![0; config.initial_chunk()];
        let allocate_threshold = config.allocate_threshold();
        Self {
            memory,
            while_stack: Vec::new(),
            allocate_threshold,
            memory_index: 0,
            command_index: 0,
        }
    }

    pub fn move_pointer(&mut self, offset: i32) {
        if offset < 0 && offset.abs() > self.memory_index as i32 {
            eprintln!("<Exception> in <MoveCommand>: MOVE_OUT_OF_BOUND");
            eprintln!(
                "current index: {}, wanted to move: {}",
                self.memory_index, offset
            );
            process::exit(MOVE_OUT_OF_BOUND);
        }
        self.memory_index = (self.memory_index as i32 + offset) as u32;
        while self.memory_index as usize >= self.memory.len() {
            if self.memory.len() > self.allocate_threshold {
                self.memory
                    .resize(self.memory.len() + self.allocate_threshold, 0);
            } else {
                self.memory.resize(self.memory.len() * 2, 0);
            }
        }
    }

    pub fn run(&mut self, loader: &Loader) {
        let commands = loader.commands();
        while (self.command_index as usize) < commands.len() {
            if self.while_stack.is_empty() {
                commands[self.command_index as usize].run(self);
            } else {
                if !self.while_stack.last().unwrap().ignoring {
                    commands[self.command_index as usize].run(self);
                }
            }
            self.command_index += 1;
        }
    }

    pub fn calculate(&mut self, value: i32) {
        self.memory[self.memory_index as usize] += value;
    }

    pub fn putchar(&self) {
        print!("{}", self.memory[self.memory_index as usize] as u8 as char);
    }

    pub fn getchar(&mut self) {
        let mut input = [0u8; 1];
        io::stdin().read(&mut input).unwrap();
        self.memory[self.memory_index as usize] = input[0] as i32;
    }

    pub fn putint(&self) {
        print!("{}", self.memory[self.memory_index as usize]);
    }

    pub fn getint(&mut self) {
        let mut input = [0u8; 1];
        io::stdin().read(&mut input).unwrap();
        self.memory[self.memory_index as usize] = (input[0] as char).to_digit(10).unwrap() as i32;
    }

    pub fn start_while(&mut self) {
        if self.memory[self.memory_index as usize] != 0 {
            self.while_stack
                .push(WhileCondition::new(self.command_index, self.memory_index));
        } else {
            self.while_stack.push(WhileCondition::new_ignoring(
                self.command_index,
                self.memory_index,
            ));
        }
    }

    pub fn end_while(&mut self) {
        let while_condition = self.while_stack.last().unwrap();
        if self.memory[while_condition.condition_index as usize] != 0 {
            self.command_index = while_condition.command_index;
        } else {
            self.while_stack.pop();
        }
    }
}
