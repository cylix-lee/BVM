use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    process,
    rc::Rc,
};

use crate::{
    command::{command_factory, Command},
    LOADING_FAILURE, MAX_ARGUMENT_COUNT, OPEN_FILE_FAILURE, TOO_MANY_ARGUMENTS,
};

pub struct Loader {
    commands: VecDeque<Rc<dyn Command>>,
}

impl Loader {
    pub fn new(filename: &String) -> Self {
        let file = File::open(filename);
        if file.is_err() {
            let error = file.unwrap_err();
            eprintln!("[Error] Cannot open file {}", filename);
            eprintln!("{}", error.to_string());
            process::exit(OPEN_FILE_FAILURE);
        }
        let file = file.unwrap();
        let mut commands = VecDeque::new();
        let mut reader = BufReader::new(file);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("[Error] Unknown Error while loading bytecode.");
                    eprintln!("{}", e.to_string());
                    process::exit(LOADING_FAILURE);
                }
            }
            let code: Vec<String> = line
                .trim()
                .to_string()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            if code.len() > MAX_ARGUMENT_COUNT + 1 {
                eprintln!("[Error] Too many arguments in {}", code[0]);
                process::exit(TOO_MANY_ARGUMENTS);
            }
            if !code.is_empty() {
                commands.push_back(command_factory(&code));
            }
        }
        Self { commands }
    }

    pub fn commands(&self) -> &VecDeque<Rc<dyn Command>> {
        &self.commands
    }
}
