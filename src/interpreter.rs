use std::collections::HashMap;
use log::warn;
use crate::parser::{build_statement_debug_strings, FunctionAST, LetBindingAST, ProgramAST};
use crate::parser::AST;
use crate::parser::StatementAST::Bind;


pub fn eval(ast: &ProgramAST) -> i64{
    let mut global_scope = process_global_functions(ast);
    let main_ast = ast.functions.iter().find(
        |&x| x.function_name == "main");
    match main_ast {
        None => panic!("function main not found!"),
        _ => ()
    }
    let main_ast = main_ast.unwrap();
    warn!("main ast {:?}", main_ast.debug_strings());
    execute_function(&global_scope, HashMap::new(),
                     main_ast, true);
    0
}

fn execute_function(global: &HashMap<String, FunctionClojure>,
                    local: HashMap<String, IroncamelExpression>,
                    exec: &FunctionAST, allow_io: bool) -> IroncamelExpression{
    let mut local = local;
    for s in &exec.statements {
        match s {
            Bind(lb) => {
                warn!("Try to process {:?}", lb.debug_strings())
            },
            _ => panic!("Not supported other statements!"),
        }
    }
    IroncamelExpression::StubExpr
}

fn process_global_functions(prog: &ProgramAST) -> HashMap<String,FunctionClojure> {
    let mut result = HashMap::new();

    for func in &prog.functions {
        let name = func.function_name.to_owned();
        if name == "main" {
            continue;
            // We don't process main function here
        }
        warn!("interpreting {}", &name);
        let cloj = FunctionClojure{};
        result.insert(name, cloj);
    }

    result
}


// I think using enum in rust is better than using Java-like interfaces
enum IroncamelExpression {
    FunctionClojure(FunctionClojure),
    StubExpr
}

struct FunctionClojure {

}