use std::fmt;
use log::{debug, info, warn, error};
use crate::expr::{ExprAST, InvalidExpr, try_read_expr};
use crate::tokenizer::Token;
use crate::tokenizer::Token::{IdentifierToken, KeywordFn, KeywordLet, LeftCurlyBracket, LeftParentheses, OperatorAssign, RightCurlyBracket, RightParentheses, Semicolon, SpaceToken};
const DEBUG_TREE_INDENT: &'static str = "|-- ";
const INVALID_PLACEHOLDER: &str = "stub";

pub trait AST {
    fn debug_strings(&self) -> Vec<String>;

}

pub struct ProgramAST {
    pub functions : Vec<FunctionAST>
}

pub struct FunctionAST {
    pub function_name : String,
    pub statements : Vec<Box<dyn StatementAST>>
}

pub trait StatementAST : AST {

}
struct LetBindingAST {
    pub variable: String,
    pub expr : Box<dyn ExprAST>
}


pub fn build_ast(tokens: &Vec<Token>) -> ProgramAST {
    warn!("Building ast");
    warn!("{:?}", tokens);
    let mut functions = Vec::new();
    let mut pos = 0;
    while pos < tokens.len() {
        if tokens[pos] == SpaceToken {
            pos +=1 ;
            continue;
        }
        let (funAST, len) = readFunctionAST(tokens, pos);
        debug!("Got fun");
        functions.push(funAST);
        pos += len;
    }
    let mut result = ProgramAST{functions};
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
    let mut statements: Vec<Box<dyn StatementAST>> = Vec::new();
    loop {
        let (statement, sta_len) = try_readStatementAST(tokens, pos+len);
        match sta_len {
            None => break,
            _ => ()
        }
        // let statement = statement.unwrap();
        let sta_len = sta_len.unwrap();
        statements.push(Box::new(statement));
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

fn try_readStatementAST(tokens: &Vec<Token>, pos: usize) -> (impl StatementAST, Option<usize>) {
    // Try read an assignment
    let (assignment, len) = try_read_let_binding(tokens, pos);
    match len {
        Some(_) => return (assignment, len),
        None => { info!("Not an assignment"); ()}
    }
    error!("TODO not implemented");
    ( generate_invalid_let_binding_ast(), None)
}

fn generate_invalid_let_binding_ast() -> LetBindingAST {
    LetBindingAST{ variable: INVALID_PLACEHOLDER.to_string(), expr: Box::new(InvalidExpr{}) }
}
fn try_read_let_binding(tokens: &Vec<Token>, pos: usize) -> (LetBindingAST, Option<usize>) {
    warn!("try assignment {:?}", tokens[pos]);
    let mut len = 0;
    let stub = generate_invalid_let_binding_ast();
    if tokens[pos+len] != KeywordLet {
        return (stub, None);
    }
    len += 1;

    let mut var_name;
    if let IdentifierToken(name) = &tokens[pos+len] {
        var_name = name;
        warn!("identifier for assign {:?}", var_name);
    } else {
        return (stub, None);
    }
    len += 1;

    if tokens[pos+len] != OperatorAssign {
        return (stub, None);
    }
    len += 1;


    let (expr, expr_len) = try_read_expr(tokens, pos + len);
    match expr_len {
        None => {
            info!("Not a valid expression");
            return (stub, None);
        },
        _ => ()
    };
    let expr_len = expr_len.unwrap();
    len += expr_len;

    if tokens[pos+len] != Semicolon {
        return (stub, None);
    }
    len += 1;
    let assignment = LetBindingAST {
        variable : var_name.clone(),
        expr: Box::new(expr)
    };
    (assignment, Some(len))
}

impl AST for FunctionAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(1 + self.statements.len());
        // let fname = &self.function_name;
        debug.push(format!("Function: {fname}", fname=&self.function_name));
        for statement in &self.statements {
            for dbgs in statement.debug_strings() {
                let s:String = DEBUG_TREE_INDENT.to_owned() + &dbgs;
                debug.push(s);
            }
        }
        debug
    }

}

impl AST for ProgramAST {
    // fn debug_strings(&self) -> Vec<String> {
    //     vec![String::from("Program")]
    // }
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::new();
        // let fname = &self.function_name;
        debug.push(format!("Program"));
        for f in &self.functions {
            for dbgs in f.debug_strings() {
                let s:String = DEBUG_TREE_INDENT.to_owned() + &dbgs;
                debug.push(s);
            }
        }
        debug
    }
}
impl fmt::Debug for ProgramAST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let oneline = self.debug_strings().join("\n");
        write!(f, "\n{}", oneline)
    }
}

impl StatementAST for LetBindingAST {
}

impl AST for LetBindingAST {
    fn debug_strings(&self) -> Vec<String> {
        vec![String::from("Assignment")]
    }

}