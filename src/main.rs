use std::{env, process};

use mg::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::build(&args).unwrap_or_else(|e| {
        println!("problem parsing argument: {e}");
        process::exit(-1);
    });
    if let Err(e) = run(cfg) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
