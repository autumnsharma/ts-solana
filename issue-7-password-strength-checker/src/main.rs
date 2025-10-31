use std::io;

fn main() {
    println!("Enter a password to check its strength:");

    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let score = calculate_strength(password);
    match score {
        0..=2 => println!("Weak 🔴"),
        3..=4 => println!("Moderate 🟡"),
        _ => println!("Strong 🟢"),
    }
}

fn calculate_strength(password: &str) -> u8 {
    let mut score = 0;
    if password.len() >= 8 { score += 1; }
    if password.chars().any(|c| c.is_uppercase()) { score += 1; }
    if password.chars().any(|c| c.is_lowercase()) { score += 1; }
    if password.chars().any(|c| c.is_digit(10)) { score += 1; }
    if password.chars().any(|c| "!@#$%^&*()_+-=[]{}".contains(c)) { score += 1; }
    score
}
