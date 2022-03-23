// This is part of parser. However, as Expr is the most complicated part when building the AST
//      I'm separating it to a new file

use log::{error, warn};
use crate::parser::{AST, BlockAST, read_block, DEBUG_TREE_INDENT};
use crate::tokenizer::Token;
use crate::tokenizer::Token::{Integer64, LiteralTrue, LiteralFalse, KeywordIf, KeywordThen, KeywordElse};




/* In other parts in my parsr, I would use Option to wrap the AST object
    However, I just found that I can't warp a dyn trait: https://users.rust-lang.org/t/why-doesnt-option-support-dyn-trait/45353/11
    I don't like this inconsistency (well the structure s not perfect)
    Just don't deep into Rust too much yet. 2022-03-08
*/
pub fn try_read_expr(tokens: &Vec<Token>, pos: usize) -> (ExprAST, Option<usize>) {
    warn!("try expr {:?}", tokens[pos]);

    match &tokens[pos] {
        Integer64(x) => {
            let expr = IntegerLiteral{value: *x };
            return (ExprAST::Int(*x), Some(1));
        },
        LiteralTrue => {
            return (ExprAST::Bool(true), Some(1));
        },
        LiteralFalse => {
            return (ExprAST::Bool(false), Some(1));
        }
        Token::KeywordIf => {
            let (ast, len) = read_if_expr(tokens, pos);
            return (ExprAST::If(ast), Some(len));
        }
        Token::IdentifierToken(s) => {
            // let ast = Variable{name: s.to_owned() };
            return (ExprAST::Variable(s.to_owned()), Some(1));
        }
        _ => {
            error!("Not supported yet!");
            ()
        }
    }

    (ExprAST::Error, None)
}

fn read_if_expr(tokens: &Vec<Token>, pos: usize) -> (IfElseExpr, usize) {
    let mut len = 0;
    assert_eq!(KeywordIf, tokens[pos + len]);
    len += 1;

    let (condition, con_len) = try_read_expr(tokens, len+pos);
    let con_len = con_len.unwrap();
    len += con_len;

    assert_eq!(KeywordThen, tokens[pos + len]);
    len += 1;

    let (then_case, con_len) = read_block(tokens, len+pos);
    len += con_len;

    assert_eq!(KeywordElse, tokens[pos + len]);
    len += 1;

    let (else_case, con_len) = read_block(tokens, len+pos);
    len += con_len;

    let ast = IfElseExpr{
        condition: Box::new(condition),
        then_case,
        else_case
    };
    (ast, len)
}

pub enum ExprAST {
    Int(i64),
    Bool(bool),
    Variable(String),
    Block(BlockAST),
    If(IfElseExpr),


    Error
}


pub struct IntegerLiteral {
    pub value: i64
}

pub struct IfElseExpr {
    pub condition: Box<ExprAST>,
    pub then_case: BlockAST,
    pub else_case: BlockAST
}


