use crate::expr::{ExprAST, IfElseExpr, IntegerLiteral};
use crate::parser::{StatementAST, LetBindingAST, AST, DEBUG_TREE_INDENT, FunctionAST, BlockAST};

pub fn build_statement_debug_strings(statement: &StatementAST) -> Vec<String> {

    return match statement {
        StatementAST::Bind(lb) => lb.debug_strings(),
        EmptyStatement => vec![String::from("EmptyStatement")],
        IOAction=> vec![String::from("IO Not supported!")],
        Error=> vec![String::from("ERROR!!")]
    }

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
        ExprAST::CallCallableObject(func_name, args) => {
            let mut debug = Vec::with_capacity(1 + args.len());
            debug.push( format!("Call: {val}", val=func_name) );
            for expr in args {
                let single_line = build_expr_debug_strings(expr).join(" ");
                debug.push(DEBUG_TREE_INDENT.to_owned() + &single_line);
            }
            debug
        }
        _ => vec![String::from("Expr Unknown type")]
    };
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