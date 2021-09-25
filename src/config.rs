use std::{env, process};

use crate::{ALLOCATE_THRESHOLD, INITIAL_CHUNK, MULTIPLE_INPUTS, NO_INPUTS};

pub struct Config {
    filename: String,
    initial_chunk: usize,
    allocate_threshold: usize,
    print_version: bool,
    is_translate: bool,
    is_optimize: bool,
    is_speedtest: bool,
}

impl Config {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            eprintln!("[Error] No inputs.");
            process::exit(NO_INPUTS);
        }
        let args = &args[1..];
        let mut filename = String::new();
        let mut initial_chunk: usize = INITIAL_CHUNK;
        let mut allocate_threshold: usize = ALLOCATE_THRESHOLD;
        let mut print_version = false;
        let mut is_translate = false;
        let mut is_optimize = false;
        let mut is_speedtest = false;

        let mut specify_chunk = false;
        let mut specify_threshold = false;
        for arg in args {
            if specify_chunk {
                initial_chunk = arg.parse().unwrap();
                specify_chunk = false;
            } else if specify_threshold {
                allocate_threshold = arg.parse().unwrap();
                specify_threshold = false;
            } else {
                if arg == "-initial" {
                    specify_chunk = true;
                } else if arg == "-threshold" {
                    specify_threshold = true;
                } else if arg == "-version" {
                    print_version = true;
                } else if arg == "-translate" {
                    is_translate = true;
                } else if arg == "-optimize" {
                    is_optimize = true;
                } else if arg == "-speedtest" {
                    is_speedtest = true;
                } else {
                    if filename.is_empty() {
                        filename = arg.clone();
                    } else {
                        eprintln!("[Error] Multiple inputs.");
                        eprintln!("has: {}, meet: {}", filename, arg);
                        process::exit(MULTIPLE_INPUTS);
                    }
                }
            }
        }
        Self {
            filename,
            initial_chunk,
            allocate_threshold,
            print_version,
            is_translate,
            is_optimize,
            is_speedtest,
        }
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }
    pub fn initial_chunk(&self) -> usize {
        self.initial_chunk
    }
    pub fn allocate_threshold(&self) -> usize {
        self.allocate_threshold
    }
    pub fn print_version(&self) -> bool {
        self.print_version
    }
    pub fn is_translate(&self) -> bool {
        self.is_translate
    }
    pub fn is_optimize(&self) -> bool {
        self.is_optimize
    }
    pub fn is_speedtest(&self) -> bool {
        self.is_speedtest
    }
}
