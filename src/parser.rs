use log::{warn};
use crate::tokenizer::Token;

pub trait AST {
}

pub struct ProgramAST {

}

impl AST for ProgramAST {

}

pub fn build_ast(tokens: &Vec<Token>) -> ProgramAST {
    warn!("Building ast");
    let mut result = ProgramAST{};
    result
}