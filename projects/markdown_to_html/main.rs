use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <markdown-file>");
        return;
    }

    let input_path = &args[1];
    let content = fs::read_to_string(input_path)
        .expect("Failed to read file");

    let html = content
        .replace("# ", "<h1>")
        .replace("## ", "<h2>")
        .replace("**", "<b>")
        .replace("*", "<i>")
        .replace("\n", "<br>\n");

    println!("Converted HTML:\n{}", html);
}

