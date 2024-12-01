use aoc_2024::get_solution;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <day> <part>");
        process::exit(1);
    }

    match get_solution(&args[1], &args[2]) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1)
        }
    }
}
