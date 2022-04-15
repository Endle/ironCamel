use std::collections::HashMap;
use std::rc::Rc;
use log::{debug, error, info, warn};
use crate::builtin;
use crate::parser::{BlockAST, function2block, FunctionAST, LetBindingAST, ProgramAST, StatementAST};
use crate::parser::AST;
use crate::debug_output::{build_statement_debug_strings,build_expr_debug_strings};
use crate::expr::{ClosureAST, ExprAST};


use crate::builtin::{IroncamelLinkedList, perform_write};
use crate::interpreter::CallableObject::Closure;


struct GlobalState {
    pub global_scope: HashMap<String,FunctionAST>
}

impl GlobalState {
    pub fn is_defined_in_global(&self, func_name: &str) -> bool {
        self.has_builtin_function(func_name) || self.global_scope.contains_key(func_name)
    }
    pub(crate) fn has_builtin_function(&self, func_name: &str) -> bool {
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

#[derive(Clone)]
pub enum CallableObject {
    GlobalFunction(String),
    BuiltinFunction(String),
    Closure(Rc<ClosureAST>, Rc<HashMap<String,ExprAST>>),
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
    execute_function(&global_scope, &main_ast,&Vec::new(),true);
    // execute_block(&global_scope, &HashMap::new(),
    //               &function2block((*main_ast).clone()), true);
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
                let expr = solve(&global, &mut local, expr_ast);
                local.insert(var.to_owned(), expr);
            },
            StatementAST::Write(write) => {
                if !allow_io { panic!("IO is not allowed in this scope") }
                // TODO assume writeline AND STDOUT
                info!("Trying to process write");
                let expr = solve(&global, &mut local, &write.expr);
                perform_write(&write.impure_procedure_name, &write.file_handler, &expr);
            }
            _ => panic!("Not supported other statements!"),
        }
    }
    solve(global, &mut local, &exec.return_expr)
}

fn execute_block(global: &GlobalState,
                 local: &HashMap<String, ExprAST>,
                 exec: &BlockAST, allow_io: bool) -> ExprAST{
    if exec.statements.len() == 0 {
        // info!("Fast solve block {:?}", exec.return_expr);
        // return lazy_solve_no_update(global, local, &exec.return_expr)
        info!("TODO: Avoid unnecessary local env copy");
    }
    execute_block_with_consumable_env(global, local.clone(), exec, allow_io)
}


fn execute_function(global: &GlobalState, fun: &FunctionAST, params: &Vec<ExprAST>,
                    allow_io: bool) -> ExprAST{
    assert_eq!(fun.arguments.len(), params.len());
    let mut new_env = HashMap::new();
    for i in 0..fun.arguments.len() {
        let var_name = &fun.arguments[i];
        new_env.insert(var_name.to_owned(), params[i].to_owned());
    }
    execute_block_with_consumable_env(
        global, new_env,
        &function2block(fun.clone()), allow_io)
}


/*
fn eager_solve(global: &GlobalState, local: &mut HashMap<String, ExprAST>,
               ast: &ExprAST) -> ExprAST {
    let ast = solve(global, local, ast);
    debug!("Eager solving {:?}", build_expr_debug_strings(&ast));
    let result = match ast {
        ExprAST::Int(_) |  ExprAST::Bool(_) => ast,
        ExprAST::CallBuiltinFunction(func_name, params) => {
            let mut solved_params = Vec::with_capacity(params.len());
            for p in params {
                let rp = solve(global, local, &p);
                solved_params.push(rp);
            }
            builtin::call_builtin_function(&func_name, solved_params)
        },
        ExprAST::CallCallableObjectByname(func_name, params) => {
            let mut solved_params = Vec::with_capacity(params.len());
            for p in params {
                let rp = solve(global, local, &p);
                solved_params.push(rp);
            }
            match global.find_function(&func_name) {
                Some(func) => {
                    execute_function(global, func, &solved_params, false)
                },
                None => panic!("Global function ({}) not found!", func_name)
            }
        }
        _ => todo!()
    };
    debug!("Eager solving result {:?}", build_expr_debug_strings(&result));
    result
}


 */

fn solve(global: &GlobalState, local: &mut HashMap<String, ExprAST>,
         ast: &ExprAST) -> ExprAST {

    info!("Eager solving {:?} with env {:?}", build_expr_debug_strings(&ast), local.keys());

    // info!("Local env {:?}", local.keys());
    let result = match ast {
        ExprAST::Int(_) |  ExprAST::Bool(_) => ast.clone(),
        // TODO the implementation for lookup is not correct
        ExprAST::Variable(v) => {
            if global.global_scope.contains_key(v) {
                return ExprAST::Callable(CallableObject::GlobalFunction(v.clone()));
            }
            if global.has_builtin_function(v) {
                return ExprAST::Callable(CallableObject::BuiltinFunction(v.clone()));
                // return lazy_solve(global, local,
                //                   &ExprAST::Callable(CallableObject::GlobalFunction(v.clone())));
            }
            let result = lookup_local_variable(global, local, v);
            result
        }

        ExprAST::CallCallableObjectByname(func_name, params) => {
            // Is this a local function?
            let callee : ExprAST = find_callee(global, local, func_name, params);
            solve(global, local, &callee)
        }
        ExprAST::If(if_expr) => {
            let cond = solve(global, local, &if_expr.condition);
            let cond = match cond {
                ExprAST::Bool(x) => x,
                _ => panic!("Expect a boolean value, got {:?}", build_expr_debug_strings(&cond))
            };
            let selected = if cond { &if_expr.then_case} else { &if_expr.else_case};
            // for s in build_expr_debug_strings(ast) {eprintln!("{}",s);}
            // info!("Condition is {}", cond);
            // info!("Selected {:?}", selected.debug_strings());
            // info!("Local env is {:?}", local);
            execute_block(global, local, selected, false)
        },
        ExprAST::CallBuiltinFunction(func_name, params) => {
            let mut solved_params = Vec::with_capacity(params.len());
            for p in params {
                let rp = solve(global, local, &p);
                solved_params.push(rp);
            }
            builtin::call_builtin_function(&func_name, solved_params)
        },
        // ExprAST::CallBuiltinFunction(_func_name, _params) => {
        //     debug!("Skip builtin when lazy eval {:?}", build_expr_debug_strings(ast));
        //     ast.clone()
        // }
        ExprAST::List(list) => {
            ExprAST::List(solve_list(global, local, list))
        },

        ExprAST::Closure(clos) => {
            ExprAST::Callable(Closure(
                clos.clone(),
                Rc::new(local.clone())
            ))
        },
        _ => {
            panic!("Not supported ast yet : {:?}", build_expr_debug_strings(ast));
        }
    };
    debug!("Lazy solving {:?} -> {:?}", build_expr_debug_strings(ast), result);
    result
}

fn solve_list(global: &GlobalState, local: &mut HashMap<String, ExprAST>,
              head: &Rc<IroncamelLinkedList>) -> Rc<IroncamelLinkedList>{
    if head.len == 0 {
        return Rc::new(IroncamelLinkedList::build_empty_list())
    }
    let solved_head = solve(global, local, head.hd());
    match head.tl() {
        Some(t) => {
            let rest = solve_list(global, local, &t);
            Rc::new(IroncamelLinkedList::cons(solved_head, &rest))
        }
        None => {
            Rc::new(IroncamelLinkedList::build(solved_head))
        }
    }

}

fn find_callee(global: &GlobalState, local: &mut HashMap<String, ExprAST>, func_name: &String, params: &Vec<Box<ExprAST>>) -> ExprAST {
    match local.get(func_name) {
        Some(x) => {
            let callee = match x {
                ExprAST::Callable(co) => co,
                _ => panic!("Expect a callable object, got {:?}", x)
            };
            return match callee {
                CallableObject::GlobalFunction(f) => {
                    ExprAST::CallCallableObjectByname(f.to_owned(),
                                                      box_expr(&solve_parameters(global, local, params)))
                }
                CallableObject::BuiltinFunction(f) => {
                    ExprAST::CallBuiltinFunction(f.to_owned(),
                                                 box_expr(
                                                     &solve_parameters(global, local, params)))
                }
                CallableObject::Closure(_,_) => { todo!() }
            };
        }
        None => { debug!("Not found variable ({}) in local scope", func_name)}
    }
    match global.has_builtin_function(func_name) {
        true => {
            let lazy_solved_params = solve_parameters(global, local, params);

            return ExprAST::CallBuiltinFunction(func_name.to_owned(),
                                                box_expr(&lazy_solved_params));
        },
        false =>  { info!("Not a builtin function ({}) ", func_name)}
    }
    match global.find_function(func_name) {
        Some(fun) => {

            return execute_function(global, fun,
                                    &solve_parameters(global, local, params), false);
        }
        None  => { info!("Not found variable ({}) in local scope", func_name)}
    }
    panic!("Can't find a callable object called ({})", func_name)
}

fn box_expr(input: &Vec<ExprAST>) -> Vec<Box<ExprAST>> {
    let mut result = Vec::with_capacity(input.len());
    for x in input {
        result.push(Box::new(x.to_owned()));
    }
    result
}

// This function is not lazy enough
fn lookup_local_variable(global: &GlobalState, local: &mut HashMap<String, ExprAST>, v: &str) -> ExprAST {
    let x = match local.get(v) {
        Some(a) => a,
        None =>{ panic!("Not found variable ({}) in local scope", v)}
    };
    let x = x.clone();

    let mut dirty = false;
    let result = match x {
        ExprAST::Int(_) | ExprAST::Bool(_)=> { x },
        ExprAST::Variable(_) => {
            dirty = true;
            solve(global, local, &x)
        }
        ExprAST::Block(_) => {todo!()}
        ExprAST::If(_) => {todo!()}
        ExprAST::CallCallableObjectByname(func_name, params) => {
            let rp = solve_parameters(global, local, &params);
            ExprAST::CallCallableObjectByname(func_name.to_owned(), box_expr(&rp))
        }
        ExprAST::Error => {todo!()}
        ExprAST::CallBuiltinFunction(func_name, params) => {
            let rp = solve_parameters(global, local, &params);
            ExprAST::CallBuiltinFunction(func_name.to_owned(), box_expr(&rp))
        }
        ExprAST::Callable(co) => {ExprAST::Callable(co.clone())}
        ExprAST::List(_) => { x }
        ExprAST::Closure(_) => { todo!() }
    };

    if dirty {
        info!("Local env updated for {}", v);
        local.insert(String::from(v), result.clone());
    }

    // info!("Lazy lookup ({}) -> -> is {:?}", v, result);
    result
}

fn solve_parameters(global: &GlobalState,
                    local: &mut HashMap<String, ExprAST>, params: &Vec<Box<ExprAST>>)
                    -> Vec<ExprAST>{
    let mut solved = Vec::with_capacity(params.len());
    for p in params {
        let rp = solve(global, local, p);
        solved.push(rp);
    }
    solved
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
