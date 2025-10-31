use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <markdown-file>");
        return;
    }

    let path = &args[1];
    let content = fs::read_to_string(path)
        .expect("Unable to read file");

    // very minimal Markdown → HTML
    let html = content
        .replace("# ", "<h1>")
        .replace("## ", "<h2>")
        .replace("**", "<b>")
        .replace("*", "<i>")
        .replace("\n", "<br>\n");

    println!("{}", html);
}
git add .
git commit -m "Added Markdown to HTML converter (fixes #3)"
git push origin issue-3-markdown-to-html
