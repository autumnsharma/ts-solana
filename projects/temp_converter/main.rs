use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run <value> <toC|toF>");
        return;
    }

    let value: f64 = args[1].parse().unwrap();
    let result = match args[2].as_str() {
        "toC" => (value - 32.0) * 5.0 / 9.0,
        "toF" => (value * 9.0 / 5.0) + 32.0,
        _ => {
            println!("Use toC or toF");
            return;
        }
    };

    println!("Converted: {:.2}", result);
}

