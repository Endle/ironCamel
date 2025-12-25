

use crate::parser::{ProgramAST, FunctionAST, BlockAST};
use log::{debug, info};

use inkwell::context::Context;


fn compile_fn(fnast: &FunctionAST){
    let context = Context::create();
    let module = context.create_module("ret");
    let builder = context.create_builder();
    let i32_type = context.i32_type();
    let arg_types = [i32_type.into()];
    let fn_type = i32_type.fn_type(&arg_types, false);
    let fn_value = module.add_function("ret", fn_type, None);
    let entry = context.append_basic_block(fn_value, "entry");
    let i32_arg = fn_value.get_first_param().unwrap();

builder.position_at_end(entry);
builder.build_return(Some(&i32_arg)).unwrap();
}

pub fn compile(ast: &ProgramAST) -> String {
    info!("to compile to llvm IR");
    let mut str_builder: Vec<String> = vec!();

    for fnast in &ast.functions {
        compile_fn(&fnast);
    }

    return str_builder.join(" \n");
}

