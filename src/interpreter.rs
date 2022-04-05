use std::collections::HashMap;
use log::{debug, error, info, warn};
use crate::builtin;
use crate::parser::{BlockAST, function2block, FunctionAST, LetBindingAST, ProgramAST, StatementAST};
use crate::parser::AST;
use crate::debug_output::{build_statement_debug_strings,build_expr_debug_strings};
use crate::expr::ExprAST;


use crate::builtin::perform_write;


struct GlobalState {
    pub global_scope: HashMap<String,FunctionAST>
}


impl GlobalState {
    pub fn is_defined_in_global(&self, func_name: &str) -> bool {
        self.find_builtin_function(func_name) || self.global_scope.contains_key(func_name)
    }
    pub(crate) fn find_builtin_function(&self, func_name: &str) -> bool {
        builtin::ARITHMETIC_OPERATORS.contains(&func_name) ||
            builtin::LIST_BUILTIN_FUNCTIONS.contains(&func_name)
    }
    pub(crate) fn find_function(&self, func_name: &String) -> Option<&FunctionAST> {
        self.global_scope.get(func_name)
    }
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
    let global_scope = build_global_state(ast);
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
fn execute_block_with_consumable_env(global: &GlobalState,
                                     mut local: HashMap<String, ExprAST>,
                                     exec: &BlockAST, allow_io: bool) -> ExprAST{
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

fn execute_block(global: &GlobalState,
                 local: &HashMap<String, ExprAST>,
                 exec: &BlockAST, allow_io: bool) -> ExprAST{
    if exec.statements.len() == 0 {
        return lazy_solve(global, local, &exec.return_expr)
    }
    execute_block_with_consumable_env(global, local.clone(), exec, allow_io)
}

fn eager_solve(global: &GlobalState, local: &HashMap<String, ExprAST>,
               ast: &ExprAST) -> ExprAST {
    let ast = lazy_solve(global, local, ast);
    debug!("Eager solving {:?}", build_expr_debug_strings(&ast));
    let result = match ast {
        ExprAST::Int(_) |  ExprAST::Bool(_) => ast,
        ExprAST::CallBuiltinFunction(func_name, params) => {
            let mut solved_params = Vec::with_capacity(params.len());
            for p in params {
                let rp = eager_solve(global, local, &p);
                solved_params.push(rp);
            }
            builtin::call_builtin_function(&func_name, solved_params)
        }
        _ => todo!()
    };
    debug!("Eager solving result {:?}", build_expr_debug_strings(&result));
    result
}

// Lazy solve would remove variable name -> No.
// So what's the purpose of lazy solve for me?
// If's condition is eager solved
fn lazy_solve(global: &GlobalState, local: &HashMap<String, ExprAST>,
              ast: &ExprAST) -> ExprAST {
    debug!("Lazy solving {:?}", build_expr_debug_strings(ast));
    match ast {
        ExprAST::Int(_) |  ExprAST::Bool(_) => ast.clone(),
        // TODO the implementation for lookup is not correct
        ExprAST::Variable(v) => {
            match local.get(v) {
                Some(x) => { x.clone() }
                None => { info!("Not found variable ({}) in this scope", v)}
            }
            if !global.is_defined_in_global(v) {
                panic!("This variable exists in neither global nor local ({})", v);
            }
            todo!()
        }
        ExprAST::CallCallableObject(func_name, params) => {
            // Is this a local function?
            match local.get(func_name) {
                Some(x) => {info!("found variable ({}) in local scope", func_name); todo!() }
                None => { info!("Not found variable ({}) in local scope", func_name)}
            }
            match global.find_builtin_function(func_name) {
                true => {
                    let mut lazy_solved_params = Vec::with_capacity(params.len());
                    for p in params {
                        let rp = lazy_solve(global, local, p);
                        lazy_solved_params.push(Box::new(rp));
                    }
                    return ExprAST::CallBuiltinFunction(func_name.to_owned(),
                                                        lazy_solved_params);
                },
                false =>  { info!("Not a builtin function ({}) ", func_name)}
            }
            match global.find_function(func_name) {
                Some(fun) => {
                    assert_eq!(fun.arguments.len(), params.len());
                    let mut new_env = HashMap::new();
                    for i in 0..fun.arguments.len() {
                        let var_name = &fun.arguments[i];
                        let param = lazy_solve(global, local, &params[i]);
                        new_env.insert(var_name.to_owned(), param);
                    }
                    return execute_block_with_consumable_env(
                        global, new_env,
                        &function2block(fun.clone()), false);
                }
                None  => { info!("Not found variable ({}) in local scope", func_name)}
            }
            panic!("Can't find a callable object called ({})", func_name)
        }
        ExprAST::If(if_expr) => {
            let cond = eager_solve(global, local, &if_expr.condition);
            let cond = match cond {
                ExprAST::Bool(x) => x,
                _ => panic!("Expect a boolean value, got {:?}", build_expr_debug_strings(&cond))
            };
            let selected = if cond { &if_expr.then_case} else { &if_expr.else_case};
            execute_block(global, local, selected, false)
        },
        ExprAST::CallBuiltinFunction(_func_name, _params) => {
            debug!("Skip builtin when lazy eval {:?}", build_expr_debug_strings(ast));
            ast.clone()
        }
        _ => {

            error!("Not supported ast yet : {:?}", build_expr_debug_strings(ast));
            ExprAST::Error
        }
    }
}



fn process_global_functions(prog: &ProgramAST) -> HashMap<String,FunctionAST> {
    let mut result = HashMap::new();

    for func in &prog.functions {
        let name = func.function_name.to_owned();
        if name == "main" {
            continue;
            // We don't process main function here
        }
        warn!("interpreting {}", &name);
        result.insert(name, func.clone());
    }

    result
}


// I think using enum in rust is better than using Java-like interfaces
// At interpreter level, everything is almost expr
