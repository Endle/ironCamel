use std::fmt;
use crate::expr::{ExprAST, IfElseExpr};
use crate::interpreter::CallableObject;
use crate::parser::{StatementAST, LetBindingAST, AST, DEBUG_TREE_INDENT, FunctionAST, BlockAST, ReadAst, ProgramAST, WriteAst};

pub fn build_statement_debug_strings(statement: &StatementAST) -> Vec<String> {

    return match statement {
        StatementAST::Bind(lb) => lb.debug_strings(),
        // StatementAST::EmptyStatement => vec![String::from("EmptyStatemt")],
        StatementAST::Read(r) => build_read_operation_debug_strings(r),
        StatementAST::Write(w) => build_write_operation_debug_strings(w),
        StatementAST::Error=> vec![String::from("ERROR!!")]
    }

}

fn build_write_operation_debug_strings(write: &WriteAst) -> Vec<String> {
    let mut debug = Vec::with_capacity(3);
    let s = format!("{p} to {f} :",
                    p = write.impure_procedure_name,
                    f = write.file_handler);
    debug.push(s);
    for debug_str in build_expr_debug_strings(&write.expr){
        let s:String = DEBUG_TREE_INDENT.to_owned() + &debug_str;
        debug.push(s);
    }
    debug
}

fn build_read_operation_debug_strings(read: &ReadAst) -> Vec<String> {
    let s = format!("{p} from {f} to >> {v}",
            p = read.impure_procedure_name,
            f = read.file_handler,
        v = read.write_to_variable);
    vec![s]
}


impl AST for IfElseExpr {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(3);
        debug.push(format!("if ({con})", con=build_expr_debug_strings(&self.condition).join(" ")));
        debug.push(format!("{ind}then {con}",
                           ind=DEBUG_TREE_INDENT,
                           con=self.then_case.debug_strings().join(" ")));
        debug.push(format!("{ind}else {con}",
                           ind=DEBUG_TREE_INDENT,
                           con=self.else_case.debug_strings().join(" ")));
        debug
    }
}




pub fn build_expr_debug_strings(expr: &ExprAST) -> Vec<String> {
    return match expr {
        ExprAST::If(s) => s.debug_strings(),
        ExprAST::Int(i) => vec![  format!("Integer: {val}", val=i) ],
        ExprAST::Bool(b) => vec![ format!("Bool: {val}", val=if *b {"true"} else {"false"}) ],
        ExprAST::Variable(v)  => vec![  format!("Variable: {val}", val=v) ],
        ExprAST::CallFunction(func_name, args) => {
            let mut debug = Vec::with_capacity(1 + args.len());
            debug.push( format!("Call: {val}", val=func_name) );
            for expr in args {
                let single_line = build_expr_debug_strings(expr).join(" ");
                debug.push(DEBUG_TREE_INDENT.to_owned() + &single_line);
            }
            debug
        },
        ExprAST::Block(block) => block.debug_strings(),
        ExprAST::CallBuiltinFunction(func_name, args) => {
            let mut debug = Vec::with_capacity(1 + args.len());
            debug.push( format!("CallBuiltin: {val}", val=func_name) );
            for expr in args {
                let single_line = build_expr_debug_strings(expr).join(" ");
                debug.push(DEBUG_TREE_INDENT.to_owned() + &single_line);
            }
            debug
        }
        ExprAST::Error => vec![String::from("ERROR EXPR")],

        ExprAST::List(_) => vec![String::from("LinkedList")],
        ExprAST::Callable(co) => vec![build_callable_object_debug_string(co)],
    };
}

pub fn build_callable_object_debug_string(co: &CallableObject) -> String {
    match co {
        CallableObject::GlobalFunction(g) => { format!("Global function [{v}]", v=g)}
        CallableObject::BuiltinFunction(b) => { format!("Builtin function [{v}]", v=b)}
        CallableObject::Closure => { format!("Closure, not supported yet")}
    }
}

impl AST for LetBindingAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::new();
        debug.push(format!("Let {var} = ", var=&self.variable));
        for dbgs in build_expr_debug_strings(&self.expr) {
            let s:String = DEBUG_TREE_INDENT.to_owned() + &dbgs;
            debug.push(s);
        }
        debug
    }
}

impl AST for FunctionAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(1 + self.statements.len());
        debug.push(format!("Function: {fname} Args: {args}",
                           fname=&self.function_name, args=self.arguments.join(",")));
        for statement in &self.statements {
            for debug_str in build_statement_debug_strings(statement) {
                let s:String = DEBUG_TREE_INDENT.to_owned() + &debug_str;
                debug.push(s);
            }
        }
        for debug_str in build_expr_debug_strings(&self.return_expr) {
            let s:String = DEBUG_TREE_INDENT.to_owned() + &debug_str;
            debug.push(s);
        }
        debug
    }
}


impl AST for BlockAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(1 + self.statements.len());
        for statement in &self.statements {
            debug.extend(build_statement_debug_strings(statement));
        }
        debug.extend(build_expr_debug_strings(&self.return_expr) );
        debug
    }
}

impl AST for ProgramAST {
    // fn debug_strings(&self) -> Vec<String> {
    //     vec![String::from("Program")]
    // }
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::new();
        // let fname = &self.function_name;
        debug.push(format!("Program"));
        for f in &self.functions {
            for dbgs in f.debug_strings() {
                let s:String = DEBUG_TREE_INDENT.to_owned() + &dbgs;
                debug.push(s);
            }
        }
        debug
    }
}
impl fmt::Debug for ProgramAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let oneline = self.debug_strings().join("\n");
        write!(f, "\n{}", oneline)
    }
}