use std::collections::HashMap;
use log::{error, info, warn};
use crate::parser::{BlockAST, function2block, FunctionAST, LetBindingAST, ProgramAST, StatementAST};
use crate::parser::AST;
use crate::debug_output::{build_statement_debug_strings,build_expr_debug_strings};
use crate::expr::ExprAST;


use crate::builtin::perform_write;

struct GlobalState {
    pub global_scope: HashMap<String,FunctionClojure>
}

impl GlobalState {
    pub(crate) fn has_identifier(&self, var: &String) -> bool {
        if self.global_scope.contains_key(var) {
            return true;
        }
        false
    }
}

pub fn eval(ast: &ProgramAST) -> i64 {
    let mut global_scope = build_global_state(ast);
    let main_ast = ast.functions.iter().find(
        |&x| x.function_name == "main");
    match main_ast {
        None => panic!("function main not found!"),
        _ => ()
    }
    let main_ast = main_ast.unwrap();
    warn!("main ast {:?}", main_ast.debug_strings());
    execute_block(&global_scope, &HashMap::new(),
                  &function2block((*main_ast).clone()), true);
    0
}

fn build_global_state(ast: &ProgramAST) -> GlobalState {
    let global_functions = process_global_functions(ast);
    GlobalState {
        global_scope: global_functions
    }
}

fn execute_block(global: &GlobalState,
                 local: &HashMap<String, ExprAST>,
                 exec: &BlockAST, allow_io: bool) -> ExprAST{
    if exec.statements.len() == 0 {
        return lazy_solve(global, local, &exec.return_expr)
    }
    let mut local = local.clone();
    for s in &exec.statements {
        match &s {
            StatementAST::Bind(lb) => {
                warn!("Try to process {:?}", lb.debug_strings());
                let var = &lb.variable;
                if global.has_identifier(var) || local.contains_key(var) {
                    panic!("{} is already in env! No shadowing allowed!", var);
                }
                let expr_ast: &ExprAST = &lb.expr;
                let expr = lazy_solve(&global, &local, expr_ast);
                local.insert(var.to_owned(), expr);
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
    lazy_solve(global, &local, &exec.return_expr)
}

fn eager_solve(global: &GlobalState, local: &HashMap<String, ExprAST>,
               ast: &ExprAST) -> ExprAST {
    let ast = lazy_solve(global, local, ast);
    match ast {
        ExprAST::Int(_) |  ExprAST::Bool(_) => ast,
        _ => todo!()
    }
}

// Lazy solve would remove variable name
fn lazy_solve(global: &GlobalState, local: &HashMap<String, ExprAST>,
              ast: &ExprAST) -> ExprAST {
    info!("Lazy solving {:?}", build_expr_debug_strings(ast));
    match ast {
        ExprAST::Int(_) |  ExprAST::Bool(_) => ast.clone(),
        // TODO the implementation for lookup is not correct
        ExprAST::Variable(v) => {
            match local.get(v) {
                Some(x) => { x.clone() }
                //TODO this is not correct. It also could be a global function's name
                None => { panic!("Not found variable ({}) in this scope", v)}
            }
        }
        ExprAST::CallCallableObject(func_name, params) => {
            // Is this a local function?
            match local.get(func_name) {
                Some(x) => {info!("found variable ({}) in local scope", func_name)   }
                None => { info!("Not found variable ({}) in local scope", func_name)}
            }
            todo!()
        }
        _ => {

            error!("Not supported ast yet : {:?}", build_expr_debug_strings(ast));
            ExprAST::Error
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