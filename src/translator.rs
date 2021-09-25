use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    process,
};

use crate::{CREATE_FILE_FAILURE, LOADING_FAILURE, OPEN_FILE_FAILURE, UNEXPECTED_CHARACTER};
use lazy_static::lazy_static;

macro_rules! hashmap {
    ($($key: expr => $value: expr), *) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

lazy_static! {
    static ref CODE_TABLE: HashMap<char, String> = hashmap!(
        '<' => String::from("leftmove 1"),
        '>' => String::from("rightmove 1"),
        '+' => String::from("add 1"),
        '-' => String::from("sub 1"),
        '.' => String::from("putchar"),
        ',' => String::from("getchar"),
        '[' => String::from("while"),
        ']' => String::from("endwhile"),
        ' ' => String::new(),
        '\t' => String::new(),
        '\n' => String::new(),
        '\r' => String::new()
    );
}

fn purify_name(filename: &String) -> String {
    filename
        .split(".")
        .map(|e| e.to_string())
        .collect::<Vec<String>>()[0]
        .clone()
}

pub fn translate(filename: &String) {
    let source = File::open(filename);
    if source.is_err() {
        eprintln!("[Error] Cannot open file {}", filename);
        process::exit(OPEN_FILE_FAILURE);
    }
    let source = source.unwrap();
    let destination_name = purify_name(filename);
    let destination = File::create(destination_name.clone() + ".brain");
    if destination.is_err() {
        eprintln!("[Error] Cannot generate file {}", destination_name);
        process::exit(CREATE_FILE_FAILURE);
    }
    let destination = destination.unwrap();

    let mut reader = BufReader::new(source);
    let mut writer = BufWriter::new(destination);
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("[Error] Unknown Error while loading brainf*ck.");
                eprintln!("{}", e.to_string());
                process::exit(LOADING_FAILURE);
            }
        }
        for code in line.as_bytes() {
            let code = *code as char;
            if !CODE_TABLE.contains_key(&code) {
                eprintln!("[Error] Unexpected character {}", code);
                process::exit(UNEXPECTED_CHARACTER);
            }
            writer
                .write(CODE_TABLE.get(&code).unwrap().as_bytes())
                .unwrap();
            writer.write("\n".as_bytes()).unwrap();
        }
    }
}
