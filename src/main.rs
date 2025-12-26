use std::fs;
use log::{debug, error, info};
use ironcamel::pipeline;
use std::io::Write;
use clap::Parser;


enum RunMode {
    AdHoc,
    CompileToLLVMIR,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The source file to be executed
    #[clap(short, long)]
    run: Option<String>,

    #[clap(short,long)]
    compile: Option<String>,

    /// Libaraies to be included
    #[clap(short, long)]
    include: Vec<String>,
}

fn read_source_code(args: &Args)->(RunMode, String){
    if (!args.run.is_none())  &&  (!args.compile.is_none()) {
        panic!("We can't define both --run and --compile");
    }
    if !args.run.is_none() {
        let main_code = fs::read_to_string(args.run.as_ref().unwrap())
            .expect("Something went wrong reading the file");
        return (RunMode::AdHoc, main_code);
    }
    if !args.compile.is_none() {
        let main_code = fs::read_to_string(args.compile.as_ref().unwrap())
            .expect("Something went wrong reading the file");
        return (RunMode::CompileToLLVMIR, main_code);
    }
    panic!("No source code is provided");
}

fn main() {
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

    let args = Args::parse();

    info!("Args {:?}", &args);

    let mut source_vec = Vec::with_capacity(args.include.len() + 1);
    for lib_path in &args.include {
        match fs::read_to_string(&lib_path) {
            Ok(s) => { source_vec.push(s); }
            Err(e) => { error!("Read lib {} failed: {}, skipping\n", lib_path, e) }
        }
    }
    let (run_mode, main_code) = read_source_code(&args);
    source_vec.push(main_code);
    let source_code = source_vec.join("\n");

    debug!("Source code:\n{}", &source_code);

    let token_stream = ironcamel::tokenizer::convert_source_to_tokens(&source_code);
    info!("{:?}", &token_stream);

    let ast = ironcamel::parser::build_ast(&token_stream);
    info!("{:?}", &ast);
    let ast = pipeline::tree_transform(ast);
    debug!("{:?}", &ast);

    match run_mode {
        RunMode::AdHoc => {
            ironcamel::interpreter::eval(&ast);
        },
        RunMode::CompileToLLVMIR => {
            info!("to compile to llvm IR");
            let ir = ironcamel::gen_ir::compile(&ast);
            println!("{}", &ir);
        },
    }


}

