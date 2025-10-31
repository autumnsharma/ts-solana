use std::env;
use figlet_rs::FIGfont;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <text>");
        return;
    }

    let input = &args[1];
    let standard_font = FIGfont::standand().unwrap_or_else(|_| FIGfont::default().unwrap());
    let figure = standard_font.convert(input);
    if let Some(figure) = figure {
        println!("{}", figure);
    }
}
