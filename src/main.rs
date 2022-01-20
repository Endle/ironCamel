use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = args.last().unwrap();

    let source_code = fs::read_to_string(source)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", source_code);
}
