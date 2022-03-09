use log::{debug, info, warn, error};
use crate::expr::try_read_expr;
use crate::tokenizer::Token;
use crate::tokenizer::Token::{IdentifierToken, KeywordFn, KeywordLet, LeftCurlyBracket, LeftParentheses, OperatorAssign, RightCurlyBracket, RightParentheses, Semicolon, SpaceToken};

pub trait AST {
    fn debug_strings(&self) -> Vec<String>;
    const DEBUG_TREE_INDENT: &'static str = " -- ";
}

pub struct ProgramAST {

}

struct FunctionAST {
    pub function_name : String,
    pub statements : Vec<StatementAST>
}
struct StatementAST {

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
    let mut statements = Vec::new();
    loop {
        let (statement, sta_len) = try_readStatementAST(tokens, pos+len);
        match statement {
            None => break,
            _ => ()
        }
        let statement = statement.unwrap();
        statements.push(statement);
        assert!(sta_len > 0);
        info!("The statement consumed {} tokens: {:?}",
            sta_len, &tokens[len..len+sta_len]);
        len += sta_len;
    }

    assert_eq!(tokens[pos + len], RightCurlyBracket);
    len += 1;

    let fun = FunctionAST{
        function_name: function_name.clone(),
        statements
    };
    warn!("Read a function \n{:?}", fun.debug_strings());

    (fun, len)

}

fn try_readStatementAST(tokens: &Vec<Token>, pos: usize) -> (Option<StatementAST>, usize) {
    // Try read an assignment
    let (assignment, len) = try_read_assignment(tokens, pos);
    match assignment {
        Some(_) => return (assignment, len),
        None => { info!("Not an assignment"); ()}
    }
    error!("TODO not implemented");
    (None, 0)
}

fn try_read_assignment(tokens: &Vec<Token>, pos: usize) -> (Option<StatementAST>, usize) {
    warn!("try assignment {:?}", tokens[pos]);
    let mut len = 0;

    if tokens[pos+len] != KeywordLet {
        return (None, 0);
    }
    len += 1;

    let mut var_name;
    if let IdentifierToken(name) = &tokens[pos+len] {
        var_name = name;
        warn!("identifier for assign {:?}", var_name);
    } else {
        return (None, 0);
    }
    len += 1;

    if tokens[pos+len] != OperatorAssign {
        return (None, 0);
    }
    len += 1;


    let (expr, expr_len) = try_read_expr(tokens, pos + len);
    match expr_len {
        None => {
            info!("Not a valid expression");
            return (None, 0);
        },
        _ => ()
    };
    let expr_len = expr_len.unwrap();
    len += expr_len;

    if tokens[pos+len] != Semicolon {
        return (None, 0);
    }
    len += 1;
    let assignment = StatementAST{};
    (Some(assignment), len)
}

impl AST for FunctionAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(1 + self.statements.len());
        // let fname = &self.function_name;
        debug.push(format!("Function: {fname}", fname=&self.function_name));
        for statement in &self.statements {
            for dbgs in statement.debug_strings() {
                let s:String = Self::DEBUG_TREE_INDENT.to_owned() + &dbgs;
                debug.push(s);
            }
        }
        debug
    }

}

impl AST for ProgramAST {
    fn debug_strings(&self) -> Vec<String> {
        vec![String::from("Program")]
    }

}
impl AST for StatementAST {
    fn debug_strings(&self) -> Vec<String> {
        vec![String::from("Statement")]
    }

}