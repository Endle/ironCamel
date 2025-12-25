use clap::Parser;
use ironcamel::pipeline;
use log::{debug, error, info};
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The source file to be executed
    #[clap(long)]
    run: String,

    /// Libaraies to be included
    #[clap(short, long)]
    include: Vec<String>,
}

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} [{}] - {}",
                record
                    .file()
                    .unwrap_or("unknown")
                    .replace("src", "")
                    .replace("\\", "")
                    .replace("/", ""),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .init();

    let args = Args::parse();

    info!("Args {:?}", &args);

    let mut source_vec = Vec::with_capacity(args.include.len() + 1);
    for lib_path in args.include {
        match fs::read_to_string(&lib_path) {
            Ok(s) => {
                source_vec.push(s);
            }
            Err(e) => {
                error!("Read lib {} failed: {}, skipping\n", lib_path, e)
            }
        }
    }
    let main_code = fs::read_to_string(&args.run).expect("Something went wrong reading the file");
    source_vec.push(main_code);
    let source_code = source_vec.join("\n");

    debug!("Source code:\n{}", &source_code);

    let token_stream = ironcamel::tokenizer::convert_source_to_tokens(&source_code);
    info!("{:?}", &token_stream);

    let ast = ironcamel::parser::build_ast(&token_stream);
    info!("{:?}", &ast);
    let ast = pipeline::tree_transform(ast);
    debug!("{:?}", &ast);

    ironcamel::interpreter::eval(&ast);
    // println!("With text:\n{}", source_code);
}
