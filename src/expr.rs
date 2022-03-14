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
pub fn try_read_expr(tokens: &Vec<Token>, pos: usize) -> (Box<dyn ExprAST>, Option<usize>) {
    warn!("try expr {:?}", tokens[pos]);

    match &tokens[pos] {
        Integer64(x) => {
            let expr = IntegerLiteral{value: *x };
            return (Box::new(expr), Some(1));
        },
        LiteralTrue => {
            return (Box::new(BooleanLiteralTrue{}), Some(1));
        },
        LiteralFalse => {
            return (Box::new(BooleanLiteralFalse{}), Some(1));
        }
        Token::KeywordIf => {
            let (ast, len) = read_if_expr(tokens, pos);
            return (Box::new(ast), Some(len));
        }
        Token::IdentifierToken(s) => {
            let ast = Variable{name: s.to_owned() };
            return (Box::new(ast), Some(1));
        }
        _ => {
            error!("Not supported yet!");
            ()
        }
    }

    (Box::new(InvalidExpr{}), None)
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
        condition,
        then_case,
        else_case
    };
    (ast, len)
}



pub trait ExprAST : AST {

}
pub struct IntegerLiteral {
    pub value: i64
}

pub struct IfElseExpr {
    pub condition:Box<dyn ExprAST>,
    pub then_case: BlockAST,
    pub else_case: BlockAST
}
impl ExprAST for IfElseExpr {}

impl AST for IfElseExpr {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(3);
        debug.push(format!("if {con}", con=self.condition.debug_strings().join(" ")));
        debug.push(format!("{ind}then {con}",
                           ind=DEBUG_TREE_INDENT,
                           con=self.then_case.debug_strings().join(" ")));
        debug.push(format!("{ind}else {con}",
                           ind=DEBUG_TREE_INDENT,
                           con=self.else_case.debug_strings().join(" ")));
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

pub struct Variable{
    pub name:String
}
impl AST for Variable { fn debug_strings(&self) -> Vec<String> { vec! [ format!("Var {n}", n=self.name) ] } }
impl ExprAST for Variable {}

pub struct InvalidExpr {}
impl AST for InvalidExpr { fn debug_strings(&self) -> Vec<String> { vec![String::from("InvalidExpr")] } }
impl ExprAST for InvalidExpr{}