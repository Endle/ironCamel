use log::{debug, warn};
use crate::tokenizer::Token;
use crate::tokenizer::Token::{IdentifierToken, KeywordFn, LeftCurlyBracket, LeftParentheses, RightCurlyBracket, RightParentheses, SpaceToken};

pub trait AST {
}

pub struct ProgramAST {

}

struct FunctionAST {

}


pub fn build_ast(tokens: &Vec<Token>) -> ProgramAST {
    warn!("Building ast");
    warn!("{:?}", tokens);
    let mut pos = 0;
    while pos < tokens.len() {
        if tokens[pos] == SpaceToken {
            pos +=1 ;
            continue;
        }
        let (funAST, len) = readFunctionAST(tokens, pos);
        warn!("Got fun");
        pos += len;
    }
    let mut result = ProgramAST{};
    result
}

fn readFunctionAST(tokens: &Vec<Token>, pos: usize) -> (FunctionAST, usize) {
    let mut len = 0;

    assert_eq!(tokens[pos + len], KeywordFn);
    len += 1;

    let mut function_name;
    if let IdentifierToken(name) = &tokens[pos+len] {
        function_name = name;
    } else {
        panic!("Expect IdentfierToken for function name, got {:?}", tokens[pos+len]);
    }
    debug!("Function name is {}", function_name);
    len += 1;

    assert_eq!(tokens[pos + len], LeftParentheses);
    len += 1;
    // CRITICAL TODO: read argument list

    assert_eq!(tokens[pos + len], RightParentheses);
    len += 1;

    assert_eq!(tokens[pos + len], LeftCurlyBracket);
    len += 1;

    // TODO read statements

    assert_eq!(tokens[pos + len], RightCurlyBracket);
    len += 1;

    let fun = FunctionAST{};
    (fun, len)

}

impl AST for FunctionAST {

}

impl AST for ProgramAST {

}