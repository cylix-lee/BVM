use std::{process, time::Instant};

use brt::{
    config::Config, loader::Loader, optimizer, translator, vm::VM, AUTHOR, DESCRIPTION, NAME,
    NO_INPUTS, VERSION,
};

fn main() {
    let config = Config::new();
    if config.print_version() {
        println!("{}", NAME);
        println!("Version {}, Created by {}", VERSION, AUTHOR);
        println!("{}", DESCRIPTION);
    }
    if !config.filename().is_empty() {
        if config.print_version() {
            println!();
        }
        if config.is_translate() {
            translator::translate(config.filename());
        } else if config.is_optimize() {
            optimizer::optimize(config.filename());
        } else {
            let loader = Loader::new(config.filename());
            let mut vm = VM::new(&config);
            let mut timer = None;
            if config.is_speedtest() {
                timer = Some(Instant::now());
            }
            vm.run(&loader);
            if config.is_speedtest() {
                println!();
                println!("----------Speedtest Result----------");
                println!("Execution Time: {}s", timer.unwrap().elapsed().as_secs_f64());
            }
        }
    } else {
        if config.is_translate() || config.is_optimize() || config.is_speedtest() {
            eprintln!("[Error] No inputs.");
            process::exit(NO_INPUTS);
        }
    }
}
