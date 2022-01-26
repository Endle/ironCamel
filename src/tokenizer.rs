pub enum Token {
    //Bracket
    LeftParentheses,
    RightParentheses,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
}
use crate::tokenizer::Token::*;

pub fn convert_source_to_tokens(code: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut pos = 0;
    let code_vec:Vec<char> = code.chars().collect();
    while pos < code.len() {
        let (len, token) = read_next_token(&code_vec, pos);
        pos += len;
        result.push(token);
    }
    result
}

// Return: length of the token, the token
fn read_next_token(code: &Vec<char>, pos: usize) -> (usize, Token) {
    assert!(pos < code.len());

    match read_next_bracket(code[pos]) {
         None => (),
         Some(e) => return (1, e),
    };
    todo!()
}


fn read_next_bracket(c:char) -> Option<Token> {
    match c {
        '{' => Some(LeftCurlyBracket),
        '}' => Some(RightParentheses),
        '(' => Some(LeftParentheses),
        ')' => Some(RightParentheses),
        '[' => Some(LeftSquareBracket),
        ']' => Some(RightSquareBracket),
        _ => None,
    }

}
