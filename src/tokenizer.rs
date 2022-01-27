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


    IdentifierToken(String),


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

    warn!("{:?}", &result);
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

    // TODO I should consider primitives
    let (len, identifier) = read_next_identifier(code, pos);
    assert!(identifier.is_some());
    (len, identifier.unwrap())
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
    let iden: String = result.into_iter().collect();
    assert_eq!(iden.len(), len);
    let token = Token::IdentifierToken(iden);
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

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    // "loop" => Keyword::Loop,
    // "continue" => Keyword::Continue,
    // "break" => Keyword::Break,
    "fn" => Token::KeywordFn,
    // "extern" => Keyword::Extern,
};
fn read_next_keyword(code: &Vec<char>, pos: usize) -> (usize, Option<Token>) {
    // warn!("read token");
    for (key, value) in KEYWORDS.entries() {
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
        // warn!("{}", literal);
    }
    return (0, None)
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