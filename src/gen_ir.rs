

use crate::parser::{ProgramAST, FunctionAST, BlockAST};
use log::{debug, info};



fn compile_fn(fnast: &FunctionAST){
}

pub fn compile(ast: &ProgramAST) -> String {
    info!("to compile to llvm IR");
    let mut str_builder: Vec<String> = vec!();

    for fnast in &ast.functions {
        compile_fn(&fnast);
    }

    return str_builder.join(" \n");
}

