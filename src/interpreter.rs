use std::collections::HashMap;
use log::{error, info, warn};
use crate::parser::{FunctionAST, LetBindingAST, ProgramAST, StatementAST};
use crate::parser::AST;
use crate::debug_output::{build_statement_debug_strings};
use crate::expr::ExprAST;


use crate::builtin::perform_write;

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
                    local: HashMap<String, ExprAST>,
                    exec: &FunctionAST, allow_io: bool) -> ExprAST{
    let mut local = local;
    for s in &exec.statements {
        match &s {
            StatementAST::Bind(lb) => {
                warn!("Try to process {:?}", lb.debug_strings());
                let var = &lb.variable;
                if global.contains_key(var) || local.contains_key(var) {
                    panic!("{} is already in env! No shadowing allowed!", var);
                }
                let expr_ast: &ExprAST = &lb.expr;
                let expr = lazy_solve(&global, &local, expr_ast);
            },
            StatementAST::Write(write) => {
                if !allow_io { panic!("IO is not allowed in this scope") }
                // TODO assume writeline AND STDOUT
                info!("Trying to process write");
                let expr = eager_solve(&global, &local, &write.expr);
                perform_write(&write.impure_procedure_name, &write.file_handler, &expr);
            }
            _ => panic!("Not supported other statements!"),
        }
    }
    ExprAST::Error
}

fn eager_solve(global: &HashMap<String, FunctionClojure>, local: &HashMap<String, ExprAST>,
               ast: &ExprAST) -> ExprAST {
    match ast {
        ExprAST::Int(x) => ExprAST::Int(*x),
        ExprAST::Bool(x) => ExprAST::Bool(*x),
        _ => todo!()
    }
}

fn lazy_solve(global: &HashMap<String, FunctionClojure>, local: &HashMap<String, ExprAST>,
              ast: &ExprAST) -> IroncamelExpression {
    match ast {
        _ => {

            error!("Not supported ast yet");
            IroncamelExpression::StubExpr
        }
    }
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
// At interpreter level, everything is almost expr

// What's the different between ExprAST and IroncamelExpression?
// At interpreter level, I'd like to remove all variables names
enum IroncamelExpression {
    FunctionClojure(FunctionClojure),
    StubExpr
}

struct FunctionClojure {

}