use std::env;
use std::fs;
use log::{debug, info};
use ironcamel::pipeline;
use std::io::Write;


struct ArgConfig {
    source_code_path: String,
}

fn main() {
    // env_logger::builder()
    //     .format_timestamp(None)
    //     .format_target(false)
    //     .init();
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] - {}",
                record.file().unwrap_or("unknown")
                    .replace("src","").replace("\\","").replace("/",""),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .init();

    let config = parse_env_args();

    // TODO error check
    let source_code = fs::read_to_string(&config.source_code_path)
        .expect("Something went wrong reading the file");

    let token_stream = ironcamel::tokenizer::convert_source_to_tokens(&source_code);
    info!("{:?}", &token_stream);

    let ast = ironcamel::parser::build_ast(&token_stream);
    info!("{:?}", &ast);
    let ast = pipeline::tree_transform(ast);
    info!("{:?}", &ast);

    ironcamel::interpreter::eval(&ast);
    // println!("With text:\n{}", source_code);
}

fn parse_env_args() -> ArgConfig {
    let args: Vec<String> = env::args().collect();
    // TODO error check
    let source = args.last().unwrap();
    ArgConfig {
        source_code_path:source.clone()
    }
}
