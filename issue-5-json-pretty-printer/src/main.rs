use std::env;
use std::fs;
use serde_json::{Value, from_str, to_string_pretty};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <json-file>");
        return;
    }

    let input_path = &args[1];
    let data = fs::read_to_string(input_path)
        .expect("Failed to read file");

    let json_value: Value = from_str(&data)
        .expect("Invalid JSON content");

    let pretty_json = to_string_pretty(&json_value)
        .expect("Failed to format JSON");

    println!("{}", pretty_json);
}
