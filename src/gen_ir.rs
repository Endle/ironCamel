

use crate::parser::{ProgramAST, FunctionAST};
use crate::expr::ExprAST;
use log::info;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::InstructionValue;
use inkwell::values::AnyValue;
use inkwell::support::LLVMString;

struct Compiler<'a> {
    context: &'a Context,
    module:  Module<'a>,
    builder: Builder<'a>,
}


fn compile_fn<'a>(compiler: &Compiler<'a>, fnast: &FunctionAST) -> LLVMString{
//InstructionValue<'a>{
    let context = &compiler.context;
    let module =  &compiler.module;
    let builder = &compiler.builder;

    info!("building function {:?}", &fnast);
    let return_type = match &*fnast.return_expr {
        ExprAST::Int(_) => context.i64_type(),
        _ => unimplemented!(),
    };
    let arg_types = []; // not supported yet
    let fn_type = return_type.fn_type(&arg_types, false);
    let fn_value = module.add_function(&fnast.function_name, fn_type, None);
    info!("{:?}", &fn_value);
    let entry = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry);

    let return_value = match &*fnast.return_expr {
        ExprAST::Int(x) =>
            context.i64_type().const_int( (*x) as u64, false),
        _ => unimplemented!(),
    };

    let inst = builder.build_return(Some(&return_value)).unwrap();
    info!("inst: {:?}", &inst);
    let code = fn_value.print_to_string();
    code
}

pub fn compile(ast: &ProgramAST) -> String {
    info!("to compile to llvm IR");
    let mut str_builder: Vec<String> = vec!();

    let context = Context::create();
    let module  = context.create_module("iron");
    let builder = context.create_builder();
    let compiler = Compiler {context:&context, module, builder};

    for fnast in &ast.functions {
        let code = compile_fn(&compiler, &fnast);
        let code = code.to_string();
        str_builder.push(code);
    }

    return str_builder.join(" \n");
}

