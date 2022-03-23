use crate::expr::ExprAST;
use crate::parser::{StatementAST, LetBindingAST, AST, DEBUG_TREE_INDENT, FunctionAST, BlockAST};

pub fn build_statement_debug_strings(statement: &StatementAST) -> Vec<String> {

    return match statement {
        StatementAST::Bind(lb) => lb.debug_strings(),
        EmptyStatement => vec![String::from("EmptyStatement")],
        IOAction=> vec![String::from("IO Not supported!")],
        Error=> vec![String::from("ERROR!!")]
    }

}

pub fn build_expr_debug_strings(expr: &ExprAST) -> Vec<String> {
    vec![String::from("Expr")]
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