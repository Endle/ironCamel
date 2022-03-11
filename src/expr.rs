// This is part of parser. However, as Expr is the most complicated part when building the AST
//      I'm separating it to a new file

use log::{error, warn};
use crate::parser::AST;
use crate::tokenizer::Token;
use crate::tokenizer::Token::Integer64;




/* In other parts in my parsr, I would use Option to wrap the AST object
    However, I just found that I can't warp a dyn trait: https://users.rust-lang.org/t/why-doesnt-option-support-dyn-trait/45353/11
    I don't like this inconsistency (well the structure s not perfect)
    Just don't deep into Rust too much yet. 2022-03-08
*/
pub fn try_read_expr(tokens: &Vec<Token>, pos: usize) -> (Box<dyn ExprAST>, Option<usize>) {
    warn!("try expr {:?}", tokens[pos]);
    warn!("Only support integers now");


    match tokens[pos] {
        Integer64(x) => {
            let expr = IntegerLiteral{value:x};
            return (Box::new(expr), Some(1));
        },
        _ => {
            error!("Not supported yet!");
            ()
        }
    }

    (Box::new(InvalidExpr{}), None)
}

pub trait ExprAST : AST {

}
pub struct IntegerLiteral {
    pub value: i64
}

pub struct InvalidExpr {}

impl AST for InvalidExpr {
    fn debug_strings(&self) -> Vec<String> {
        vec![String::from("InvalidExpr")]
    }
}

impl ExprAST for InvalidExpr{}
impl ExprAST for IntegerLiteral {}

impl AST for IntegerLiteral {
    fn debug_strings(&self) -> Vec<String> {
        vec![
            format!("Function: {val}", val=&self.value)
        ]
    }
}