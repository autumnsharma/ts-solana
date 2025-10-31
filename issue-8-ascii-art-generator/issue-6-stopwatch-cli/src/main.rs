use std::io;
use std::time::{Instant, Duration};

fn main() {
    println!("⏱️  Simple Stopwatch CLI");
    println!("Press ENTER to start, then ENTER again to stop.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // start

    let start = Instant::now();

    input.clear();
    io::stdin().read_line(&mut input).unwrap(); // stop

    let elapsed = start.elapsed();
    print_duration(elapsed);
}

fn print_duration(d: Duration) {
    let secs = d.as_secs();
    let millis = d.subsec_millis();
    println!("Elapsed time: {}.{} seconds", secs, millis);
}
