use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: cargo run <num1> <op> <num2>");
        return;
    }

    let a: f64 = args[1].parse().unwrap();
    let op = &args[2];
    let b: f64 = args[3].parse().unwrap();

    let result = match op.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => {
            println!("Unknown operator {}", op);
            return;
        }
    };

    println!("Result: {}", result);
}
