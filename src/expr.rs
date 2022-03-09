// This is part of parser. However, as Expr is the most complicated part when building the AST
//      I'm separating it to a new file

use log::{error, warn};
use crate::parser::AST;
use crate::tokenizer::Token;
use crate::tokenizer::Token::Integer64;

pub struct ExprAST {

}

pub fn try_read_expr(tokens: &Vec<Token>, pos: usize) -> (Option<ExprAST>, usize) {
    warn!("try expr {:?}", tokens[pos]);
    warn!("Only support integers now");

    let expr = ExprAST{};
    match tokens[pos] {
        Integer64(x) => {
            warn!("Dropping the value now {}", x);
            return (Some(expr), 1);
        },
        _ => {
            error!("Not supported yet!");
            ()
        }
    }

    (None, 0)
}
impl AST for ExprAST {

}