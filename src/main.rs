use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod ast;
mod runtime;
mod sqlgen;
mod tui;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut db_path = None;
    let mut script_path = None;
    let mut verbose = false;

    for arg in args.iter().skip(1) {
        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else if db_path.is_none() {
            db_path = Some(arg);
        } else if script_path.is_none() {
            script_path = Some(arg);
        }
    }
    
    if db_path.is_none() || script_path.is_none() {
        eprintln!("Usage: liteabl <database.db> <script.p> [--verbose]");
        process::exit(1);
    }
    
    let db_path = db_path.unwrap();
    let script_path = script_path.unwrap();
    
    let source = fs::read_to_string(script_path)
        .unwrap_or_else(|err| {
            eprintln!("Error reading script: {}", err);
            process::exit(1);
        });
    
    match runtime::execute(db_path, &source, verbose) {
        Ok(_) => println!("Script executed successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}