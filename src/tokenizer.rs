use phf::phf_map;
use log::{warn};


#[derive(Clone,PartialEq,
    Debug)]
pub enum Token {
    //Bracket
    LeftParentheses,
    RightParentheses,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,

    // Keywords
    KeywordFn,
    KeywordLet,


    OperatorEqual,
    OperatorAssign,
    Semicolon,


    IdentifierToken(String),


    Integer64(i64),

    SpaceToken, //It's not a valid token. I put it here for easier to implement

    PlaceholderToken,
}
use crate::tokenizer::Token::*;

pub fn convert_source_to_tokens(code: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut pos = 0;
    let code_vec:Vec<char> = code.chars().collect();
    while pos < code.len() {
        let (len, token) = read_next_token(&code_vec, pos);
        pos += len;
        if token != Token::SpaceToken {
            result.push(token);
        }
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
    match read_next_space(code[pos]) {
        None => (),
        Some(e) => return (1, e),
    };


    let (len, keyword) = read_next_keyword(code, pos);
    if keyword.is_some() {
        return (len, keyword.unwrap());
    }

    let (len, op) = read_next_operator(code, pos);
    if op.is_some() {
        return (len, op.unwrap());
    }

    let (len, primitive) = read_next_primitive(code, pos);
    if primitive.is_some() {
        return (len, primitive.unwrap());
    }
    let (len, identifier) = read_next_identifier(code, pos);
    assert!(identifier.is_some());
    (len, identifier.unwrap())
}

fn read_next_primitive(code: &Vec<char>, pos: usize) -> (usize, Option<Token>) {
    let mut prim_len = 0;
    let mut result: Vec<u8> = Vec::new();

    // TODO current only supoort integers
    // TODO no overflow detect
    while pos + prim_len < code.len() && code[pos+prim_len].is_digit(10) {
        result.push(code[pos+prim_len] as u8);
        prim_len += 1;
    }
    if result.len() == 0 {
        return (0, None);
    }
    assert!(result.len()>0);
    if result[0] == '0' as u8 {
        assert_eq!(result.len(), 1); //TODO hex support
        return (1, Some(Token::Integer64(0)));
    }
    let num:i64 = atoi::atoi(&result).unwrap();
    return (result.len(), Some(Token::Integer64(num)));
}


fn read_next_identifier(code: &Vec<char>, pos: usize) -> (usize, Option<Token>) {
    assert!(is_valid_identifier_first_letter(code[pos]));
    let mut result = Vec::new();
    let mut len = 0;
    result.push(code[pos + len]);
    len += 1;
    while pos + len < code.len() && is_valid_identifier_second_letter(code[pos+len]) {
        result.push(code[pos + len]);
        len += 1;
    }
    let identifier: String = result.into_iter().collect();
    assert_eq!(identifier.len(), len);
    let token = Token::IdentifierToken(identifier);
    (len, Some(token))
}

fn is_valid_identifier_second_letter(c: char) -> bool {
    if is_valid_identifier_first_letter(c){
        return true;
    }

    match c {
        '0' ..= '9' => true,
        _ => false
    }
}
fn is_valid_identifier_first_letter(c:char) -> bool {
    match c {
        'a' ..= 'z' => true,
        'A' ..= 'Z' => true,
        '_'     => true,
        _       => false
    }
}

fn get_next_token_in_map(code:&Vec<char>, pos:usize, map:&phf::Map<&'static str, Token>) -> (usize, Option<Token>) {
    for (key, value) in map.entries() {
        let literal: &str = *key;
        if remained_chars(code, pos) < literal.len() {
            continue;
        }
        let keyword_len = literal.len();
        let code_head = &code[pos.. pos + keyword_len];

        assert_eq!(code_head.len(), keyword_len);
        let code_head_str: String = code_head.iter().collect();
        assert_eq!(code_head_str.len(), keyword_len);

        if code_head_str == literal {
            return (keyword_len, Some( (*value).clone() ) );
        }
    }
    return (0, None)
}
static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "fn" => Token::KeywordFn,
    "let" => Token::KeywordLet,
};
fn read_next_keyword(code: &Vec<char>, pos: usize) -> (usize, Option<Token>) {
    get_next_token_in_map(code, pos, &KEYWORDS)
}

static OPERATORS: phf::Map<&'static str, Token> = phf_map! {
    "=" => Token::OperatorAssign,
    "==" => Token::OperatorEqual,
    ";" => Token::Semicolon,
};
fn read_next_operator(code: &Vec<char>, pos: usize) -> (usize, Option<Token>) {
    get_next_token_in_map(code, pos, &OPERATORS)
}


fn read_next_bracket(c:char) -> Option<Token> {
    match c {
        '{' => Some(LeftCurlyBracket),
        '}' => Some(RightCurlyBracket),
        '(' => Some(LeftParentheses),
        ')' => Some(RightParentheses),
        '[' => Some(LeftSquareBracket),
        ']' => Some(RightSquareBracket),
        _ => None,
    }
}

fn read_next_space(c:char) -> Option<Token> {
    match c {
        ' ' | '\n' | '\t'  => Some(SpaceToken),
        _ => None,
    }
}


fn remained_chars(code: &Vec<char>, pos: usize) -> usize {
    assert!(pos < code.len());
    code.len() - pos
}