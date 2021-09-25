pub mod command;
pub mod config;
pub mod loader;
pub mod optimizer;
pub mod translator;
pub mod vm;

// Copyright Informations
pub const NAME: &'static str = "Brain Bytecode Runtime (Rust Implementation)";
pub const VERSION: &'static str = "1.21.9 Resilience";
pub const AUTHOR: &'static str = "Cylix <cylix.lee@foxmail.com>";
pub const DESCRIPTION: &'static str = "This is free software; Anyone can use it for any purpose.";

// VM Error Codes
pub const NO_INPUTS: i32 = 1;
pub const MULTIPLE_INPUTS: i32 = 2;
pub const OPEN_FILE_FAILURE: i32 = 3;
pub const CREATE_FILE_FAILURE: i32 = 4;
pub const LOADING_FAILURE: i32 = 5;
pub const TOO_MANY_ARGUMENTS: i32 = 6;

// Translator Error Codes
pub const UNEXPECTED_CHARACTER: i32 = 101;

// VM Defaults
pub const INITIAL_CHUNK: usize = 16;
pub const ALLOCATE_THRESHOLD: usize = 1024;

// Loader Defaults
pub const MAX_ARGUMENT_COUNT: usize = 1;

// Exception Codes
pub const MOVE_OUT_OF_BOUND: i32 = -1;
