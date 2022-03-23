use std::fmt;
use log::{debug, info, warn, error};
use crate::expr::{ExprAST, InvalidExpr, try_read_expr};
use crate::tokenizer::Token;
use crate::tokenizer::Token::{IdentifierToken, KeywordFn, KeywordLet, LeftCurlyBracket, LeftParentheses, OperatorAssign, RightCurlyBracket, RightParentheses, Semicolon, SpaceToken};
pub const DEBUG_TREE_INDENT: &'static str = "|-- ";
const INVALID_PLACEHOLDER: &str = "stub";

pub trait AST {
    fn debug_strings(&self) -> Vec<String>;
}

pub struct ProgramAST {
    pub functions : Vec<FunctionAST>
}

pub struct FunctionAST {
    pub function_name : String,
    pub statements : Vec<StatementAST>,
    pub return_expr: Box<dyn ExprAST>
}


pub enum StatementAST {
    LetBindingAST(LetBindingAST),
    EmptyStatement,
    IOAction,
    Error
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
        let (funAST, len) = read_function(tokens, pos);
        debug!("Got fun");
        functions.push(funAST);
        pos += len;
    }
    let mut result = ProgramAST{functions};
    result
}

fn read_function(tokens: &Vec<Token>, pos: usize) -> (FunctionAST, usize) {
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



    let (block, block_len) = read_block(tokens, pos+len);
    len += block_len;

    let fun = FunctionAST{
        function_name: function_name.clone(),
        statements : block.statements,
        return_expr: block.return_expr
    };
    warn!("Read a function \n{:?}", fun.debug_strings());

    (fun, len)

}

pub(crate) fn read_block(tokens: &Vec<Token>, pos: usize) -> (BlockAST, usize) {
    let mut len = 0;

    assert_eq!(tokens[pos + len], LeftCurlyBracket);
    len += 1;


    let mut statements: Vec<StatementAST> = Vec::new();
    loop {
        let (statement, sta_len) = try_readStatementAST(tokens, pos+len);
        match sta_len {
            None => break,
            _ => ()
        }
        // let statement = statement.unwrap();
        let sta_len = sta_len.unwrap();
        statements.push(statement);
        assert!(sta_len > 0);
        info!("The statement consumed {} tokens: {:?}",
            sta_len, &tokens[len..len+sta_len]);
        len += sta_len;
    }

    let (return_expr, return_val_len) = try_read_expr(tokens, pos+len);
    match &return_val_len {
        None => panic!("This block has no return expression! Got {:?}", tokens[pos+len]),
        Some(e) => (),
    }
    len += return_val_len.unwrap();
    assert_eq!(tokens[pos + len], RightCurlyBracket);
    len += 1;

    let block = BlockAST{ statements, return_expr };
    (block, len)
}

fn try_readStatementAST(tokens: &Vec<Token>, pos: usize) -> (StatementAST, Option<usize>) {
    // Try read an assignment
    let (assignment, len) = try_read_let_binding(tokens, pos);
    match len {
        Some(_) => return ( StatementAST::LetBindingAST(assignment), len),
        None => { info!("Not an assignment"); ()}
    }
    error!("TODO not implemented");
    ( StatementAST::LetBindingAST(generate_invalid_let_binding_ast()), None)
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
        expr
    };
    (assignment, Some(len))
}

pub struct BlockAST {
    pub statements : Vec<StatementAST>,
    pub return_expr: Box<dyn ExprAST>
}

impl AST for BlockAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(1 + self.statements.len());
        for statement in &self.statements {
            debug.extend(build_statement_debug_strings(statement));
        }
        debug.extend(self.return_expr.debug_strings());
        debug
    }
}

impl ExprAST for BlockAST {

}

impl AST for FunctionAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::with_capacity(1 + self.statements.len());
        debug.push(format!("Function: {fname}", fname=&self.function_name));
        for statement in &self.statements {
            for debug_str in build_statement_debug_strings(statement) {
                let s:String = DEBUG_TREE_INDENT.to_owned() + &debug_str;
                debug.push(s);
            }
        }
        for debug_str in self.return_expr.debug_strings() {
            let s:String = DEBUG_TREE_INDENT.to_owned() + &debug_str;
            debug.push(s);
        }
        debug
    }

}

pub fn build_statement_debug_strings(statement: &StatementAST) -> Vec<String> {

    return match statement {
        StatementAST::LetBindingAST(lb) => lb.debug_strings(),
        EmptyStatement => vec![String::from("EmptyStatement")],
        IOAction=> vec![String::from("IO Not supported!")],
        Error=> vec![String::from("ERROR!!")]
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


pub struct LetBindingAST {
    pub variable: String,
    pub expr : Box<dyn ExprAST>
}
impl AST for LetBindingAST {
    fn debug_strings(&self) -> Vec<String> {
        let mut debug = Vec::new();
        debug.push(format!("Let {var} = ", var=&self.variable));
        for dbgs in &self.expr.debug_strings() {
                let s:String = DEBUG_TREE_INDENT.to_owned() + &dbgs;
                debug.push(s);
        }
        debug
    }

}