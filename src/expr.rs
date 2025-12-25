// This is part of parser. However, as Expr is the most complicated part when building the AST
//      I'm separating it to a new file

use crate::builtin::IroncamelLinkedList;
use crate::debug_output::build_expr_debug_strings;
use crate::interpreter::CallableObject;
use crate::parser::{read_argument_list, read_block, BlockAST, DEBUG_TREE_INDENT};
use crate::tokenizer::Token;
use crate::tokenizer::Token::{
    Integer64, KeywordElse, KeywordIf, KeywordThen, LeftParentheses, LiteralFalse, LiteralTrue,
    RightParentheses,
};
use log::{debug, error, info, warn};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub enum ExprAST {
    Int(i64),
    Bool(bool),
    StringLiteral(String),
    Variable(String),
    Block(BlockAST),
    If(IfElseExpr),
    Closure(Rc<ClosureAST>),

    CallCallableObjectByname(String, Vec<Box<ExprAST>>),
    Error,

    // Below in involved by interpreter
    CallBuiltinFunction(String, Vec<Box<ExprAST>>),
    Callable(CallableObject),
    List(Rc<IroncamelLinkedList>),
}

impl Debug for ExprAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", build_expr_debug_strings(self).join("\t"))
    }
}

/* In other parts in my parsr, I would use Option to wrap the AST object
    However, I just found that I can't warp a dyn trait: https://users.rust-lang.org/t/why-doesnt-option-support-dyn-trait/45353/11
    I don't like this inconsistency (well the structure s not perfect)
    Just don't deep into Rust too much yet. 2022-03-08
*/
pub fn try_read_expr(tokens: &Vec<Token>, pos: usize) -> (ExprAST, Option<usize>) {
    debug!("try to read an expr, current token {:?}", tokens[pos]);

    match &tokens[pos] {
        Integer64(x) => {
            return (ExprAST::Int(*x), Some(1));
        }
        Token::LiteralString(s) => return (ExprAST::StringLiteral(s.to_owned()), Some(1)),
        LiteralTrue => {
            return (ExprAST::Bool(true), Some(1));
        }
        LiteralFalse => {
            return (ExprAST::Bool(false), Some(1));
        }
        Token::KeywordIf => {
            let (ast, len) = read_if_expr(tokens, pos);
            return (ExprAST::If(ast), Some(len));
        }
        Token::IdentifierToken(s) => {
            let (call, len) = try_read_function_call(tokens, pos);
            match &call {
                ExprAST::Error => return (ExprAST::Variable(s.to_owned()), Some(1)),
                ExprAST::CallCallableObjectByname(_callee, _args) => return (call, Some(len)),
                _ => panic!("Unexpected read result for identifier!"),
            }
        }
        Token::VerticalBar => {
            let (ast, len) = read_closure(tokens, pos);
            return (ExprAST::Closure(Rc::new(ast)), Some(len));
        }
        _ => {
            error!("Not an expr head {:?}!", tokens[pos]);
            ()
        }
    }

    (ExprAST::Error, None)
}

// The syntax to call a function or a clojure is same. Therefore, use the same code in parser
fn try_read_function_call(tokens: &Vec<Token>, pos: usize) -> (ExprAST, usize) {
    let mut len = 0;
    let mut parameters = Vec::new();
    let func_name;
    match &tokens[pos] {
        Token::IdentifierToken(s) => func_name = s,
        _ => panic!("Unexpected token"),
    };
    len += 1;

    match tokens[pos + len] {
        LeftParentheses => (),
        // It means this is not a function call
        _ => return (ExprAST::Error, 0),
    };
    len += 1;
    while tokens[pos + len] != Token::RightParentheses {
        if tokens[pos + len] == Token::Comma {
            len += 1;
            continue;
        }
        let (expr, expr_len) = try_read_expr(tokens, pos + len);
        match expr_len {
            None => panic!("There should be a valid expr!"),
            Some(el) => {
                parameters.push(Box::new(expr));
                len += el;
            }
        };
    }

    debug!(
        "Found such function call {}, ({:?})",
        func_name,
        parameters.len()
    );
    assert_eq!(tokens[pos + len], RightParentheses);
    len += 1;

    (
        ExprAST::CallCallableObjectByname(func_name.to_owned(), parameters),
        len,
    )
}

fn read_if_expr(tokens: &Vec<Token>, pos: usize) -> (IfElseExpr, usize) {
    let mut len = 0;
    assert_eq!(KeywordIf, tokens[pos + len]);
    len += 1;

    let (condition, con_len) = try_read_expr(tokens, len + pos);
    let con_len = con_len.unwrap();
    len += con_len;

    assert_eq!(KeywordThen, tokens[pos + len]);
    len += 1;

    let (then_case, con_len) = read_block(tokens, len + pos);
    len += con_len;

    assert_eq!(KeywordElse, tokens[pos + len]);
    len += 1;

    let (else_case, con_len) = read_block(tokens, len + pos);
    len += con_len;

    let ast = IfElseExpr {
        condition: Box::new(condition),
        then_case,
        else_case,
    };
    (ast, len)
}

fn read_closure(tokens: &Vec<Token>, pos: usize) -> (ClosureAST, usize) {
    let mut len = 0;

    assert_eq!(Token::VerticalBar, tokens[pos + len]);
    len += 1;

    let (arguments, len_args) = read_argument_list(tokens, pos + len);
    len += len_args;
    warn!("Get argument list {:?}, consumed {}", &arguments, len_args);

    assert_eq!(Token::VerticalBar, tokens[pos + len]);
    len += 1;

    let (block, len_block) = read_block(tokens, pos + len);
    len += len_block;
    let result = ClosureAST {
        params: arguments,
        block,
    };
    (result, len)
}

pub struct IntegerLiteral {
    pub value: i64,
}

#[derive(Clone)]
pub struct IfElseExpr {
    pub condition: Box<ExprAST>,
    pub then_case: BlockAST,
    pub else_case: BlockAST,
}

#[derive(Clone)]
pub struct ClosureAST {
    pub params: Vec<String>,
    pub block: BlockAST,
}
