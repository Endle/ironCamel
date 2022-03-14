// This is part of parser. However, as Expr is the most complicated part when building the AST
//      I'm separating it to a new file

use log::{error, warn};
use crate::parser::AST;
use crate::tokenizer::Token;
use crate::tokenizer::Token::{Integer64, LiteralTrue, LiteralFalse, KeywordIf, KeywordThen, KeywordElse};




/* In other parts in my parsr, I would use Option to wrap the AST object
    However, I just found that I can't warp a dyn trait: https://users.rust-lang.org/t/why-doesnt-option-support-dyn-trait/45353/11
    I don't like this inconsistency (well the structure s not perfect)
    Just don't deep into Rust too much yet. 2022-03-08
*/
pub fn try_read_expr(tokens: &Vec<Token>, pos: usize) -> (Box<dyn ExprAST>, Option<usize>) {
    warn!("try expr {:?}", tokens[pos]);

    match tokens[pos] {
        Integer64(x) => {
            let expr = IntegerLiteral{value:x};
            return (Box::new(expr), Some(1));
        },
        LiteralTrue => {
            return (Box::new(BooleanLiteralTrue{}), Some(1));
        },
        LiteralFalse => {
            return (Box::new(BooleanLiteralFalse{}), Some(1));
        }
        Token::KeywordIf => {
            todo!()
            // return try_read_if_expr(tokens, pos);
        }
        _ => {
            error!("Not supported yet!");
            ()
        }
    }

    (Box::new(InvalidExpr{}), None)
}
/*
fn try_read_if_expr(tokens: &Vec<Token>, pos: usize) -> (Box<dyn ExprAST>, Option<usize>) {
    let mut len = 0;
    assert_eq!(KeywordIf, tokens[pos + len]);
    len += 1;

    let (condition, con_len) = try_read_expr(tokens, len+pos);
    let con_len = con_len.unwrap();
    len += con_len;

    assert_eq!(KeywordThen, tokens[pos + len]);
    len += 1;

    let (then_case, con_len) = try_read_expr(tokens, len+pos);
    let con_len = con_len.unwrap();
    len += con_len;

    assert_eq!(KeywordElse, tokens[pos + len]);
    len += 1;

    let (KeywordElse, con_len) = try_read_expr(tokens, len+pos);
    let con_len = con_len.unwrap();
    len += con_len;

    todo!()
}

 */

pub trait ExprAST : AST {

}
pub struct IntegerLiteral {
    pub value: i64
}

pub struct IfElseExpr {
    pub condition:Box<dyn ExprAST>,
    pub then_case: Box<dyn ExprAST>,
    pub else_case: Box<dyn ExprAST>
}
impl ExprAST for IfElseExpr {}

impl AST for IfElseExpr {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::new();
        debug.push(format!("if"));
        // for dbgs in &self.expr.debug_strings() {
        //     let s:String = DEBUG_TREE_INDENT.to_owned() + &dbgs;
        //     debug.push(s);
        // }
        debug
    }
}
impl ExprAST for IntegerLiteral {}
impl AST for IntegerLiteral {
    fn debug_strings(&self) -> Vec<String> {
        vec![
            format!("Integer: {val}", val=&self.value)
        ]
    }
}

pub struct BooleanLiteralTrue{}
impl AST for BooleanLiteralTrue { fn debug_strings(&self) -> Vec<String> { vec! [ format!("true") ] } }
impl ExprAST for BooleanLiteralTrue {}

pub struct BooleanLiteralFalse{}
impl AST for BooleanLiteralFalse { fn debug_strings(&self) -> Vec<String> { vec! [ format!("false") ] } }
impl ExprAST for BooleanLiteralFalse {}

pub struct InvalidExpr {}
impl AST for InvalidExpr { fn debug_strings(&self) -> Vec<String> { vec![String::from("InvalidExpr")] } }
impl ExprAST for InvalidExpr{}