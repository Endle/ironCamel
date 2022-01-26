use std::env;
use std::fs;

struct ArgConfig {
    source_code_path: String,
}

fn main() {
    let config = parse_env_args();

    // TODO error check
    let source_code = fs::read_to_string(&config.source_code_path)
        .expect("Something went wrong reading the file");

    let token_stream = ironcamel::tokenizer::convert_source_to_tokens(&source_code);
    println!("With text:\n{}", source_code);
}

fn parse_env_args() -> ArgConfig {
    let args: Vec<String> = env::args().collect();
    // TODO error check
    let source = args.last().unwrap();
    ArgConfig {
        source_code_path:source.clone()
    }
}