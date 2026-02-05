use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod ast;
mod runtime;
mod sqlgen;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: liteabl <database.db> <script.p>");
        process::exit(1);
    }
    
    let db_path = &args[1];
    let script_path = &args[2];
    
    let source = fs::read_to_string(script_path)
        .unwrap_or_else(|err| {
            eprintln!("Error reading script: {}", err);
            process::exit(1);
        });
    
    match runtime::execute(db_path, &source) {
        Ok(_) => println!("Script executed successfully"),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}