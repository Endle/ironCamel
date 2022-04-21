use log::{debug, info, warn, error};
use crate::debug_output::{build_expr_debug_strings,build_statement_debug_strings};
use crate::expr::{ExprAST, try_read_expr};
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
#[derive(Clone)]
pub struct FunctionAST {
    pub function_name : String,
    pub arguments: Vec<String>,
    pub statements : Vec<StatementAST>,
    pub return_expr: Box<ExprAST>
}
#[derive(Clone)]
pub struct BlockAST {
    pub statements : Vec<StatementAST>,
    pub return_expr: Box<ExprAST>
}

pub fn function2block(ast: FunctionAST) -> BlockAST {
    BlockAST {
        statements: ast.statements,
        return_expr: ast.return_expr
    }
}
#[derive(Clone)]
pub enum StatementAST {
    Bind(LetBindingAST),
    Read(ReadAst),
    Write(WriteAst),
    FileOpen(FileOpenAst),
    Error
}



pub fn build_ast(tokens: &Vec<Token>) -> ProgramAST {
    warn!("Building ast");
    debug!("{:?}", tokens);
    let mut functions = Vec::new();
    let mut pos = 0;
    while pos < tokens.len() {
        if tokens[pos] == SpaceToken {
            pos +=1 ;
            continue;
        }
        let (fun_ast, len) = read_function(tokens, pos);
        debug!("Got fun");
        functions.push(fun_ast);
        pos += len;
    }
    ProgramAST{functions}
}

fn read_function(tokens: &Vec<Token>, pos: usize) -> (FunctionAST, usize) {
    let mut len = 0;

    assert_eq!(tokens[pos + len], KeywordFn);
    len += 1;

    let function_name;
    if let IdentifierToken(name) = &tokens[pos+len] {
        function_name = name;
    } else {
        panic!("Expect IdentfierToken for function name, got {:?}", tokens[pos+len]);
    }
    debug!("Function name is {}", function_name);
    len += 1;

    assert_eq!(tokens[pos + len], LeftParentheses);
    len += 1;

    let (arguments, len_args) = read_argument_list(tokens, pos+len);
    len += len_args;
    debug!("Get argument list {:?}, consumed {}", &arguments, len_args);

    assert_eq!(tokens[pos + len], RightParentheses);
    len += 1;



    let (block, block_len) = read_block(tokens, pos+len);
    len += block_len;

    let fun = FunctionAST{
        function_name: function_name.clone(),
        arguments,
        statements : block.statements,
        return_expr: block.return_expr
    };
    info!("Read a function \n{:?}", fun.debug_strings());

    (fun, len)

}

pub fn read_argument_list(tokens: &Vec<Token>, pos: usize) -> (Vec<String>, usize) {
    let mut result = Vec::new();
    let mut len = 0;
    'each_token: loop {
        debug!("Try {:?} for read argument list, pos={}, len={}", tokens[pos+len], pos, len);
        if tokens[pos+len] == RightParentheses
        || tokens[pos+len] == Token::VerticalBar{
            break 'each_token;
        }
        match &tokens[pos + len] {
            Token::SpaceToken => (),
            Token::Comma => (),
            Token::IdentifierToken(id) =>  {
                debug!("Find a Identifier {:?}", tokens[pos+len]);
                result.push(id.to_owned())
            },
            _ =>  { panic!("Unexpected token when reading argument list {:?}", tokens[pos+len]) }
        };
        len += 1;
    }
    (result, len)
}

pub(crate) fn read_block(tokens: &Vec<Token>, pos: usize) -> (BlockAST, usize) {
    let mut len = 0;

    assert_eq!(tokens[pos + len], LeftCurlyBracket);
    len += 1;


    let mut statements: Vec<StatementAST> = Vec::new();
    loop {
        let (statement, sta_len) = try_read_statement_ast(tokens, pos+len);
        match sta_len {
            None => break,
            _ => ()
        }
        // let statement = statement.unwrap();
        let sta_len = sta_len.unwrap();
        statements.push(statement);
        assert!(sta_len > 0);
        debug!("The statement consumed {} tokens: {:?}",
            sta_len, &tokens[pos+len..pos+len+sta_len]);
        len += sta_len;
    }

    let (return_expr, return_val_len) = try_read_expr(tokens, pos+len);
    match &return_val_len {
        None => panic!("This block has no return expression! Got {:?}", tokens[pos+len]),
        Some(_) => (),
    }
    len += return_val_len.unwrap();
    assert_eq!(tokens[pos + len], RightCurlyBracket);
    len += 1;

    let block = BlockAST{ statements, return_expr: Box::new(return_expr) };
    (block, len)
}

fn generate_stub_statement()  -> (StatementAST, Option<usize>) { (StatementAST::Error, None) }
fn try_read_statement_ast(tokens: &Vec<Token>, pos: usize) -> (StatementAST, Option<usize>) {
    // Try read an assignment
    let (assignment, len) = try_read_let_binding(tokens, pos);
    match len {
        Some(_) => return (StatementAST::Bind(assignment), len),
        None => { debug!("Not an assignment"); ()}
    };
    let (io, len) = try_read_io_operation(tokens, pos);
    match len {
        Some(_) => {
            info!("IO Operation: {:?}", build_statement_debug_strings(&io));
            return (io, len);
        },
        None => { debug!("Not IO"); ()}
    };

    debug!("Not a statement");
    generate_stub_statement()
}

// Read operation would consume exactly 6 tokens
// Write operation would consume 5 tokens, then read an expression, finally a semicolon
fn try_read_io_operation(tokens: &Vec<Token>, pos: usize) -> (StatementAST, Option<usize>) {
    if tokens.len() - pos < 6 { return generate_stub_statement(); }
    let procedure = match &tokens[pos] {
        IdentifierToken(s) => s,
        _ => { debug!("Not IO operation"); return generate_stub_statement();}
    };
    if tokens[pos+1] != Token::AddressSign { return generate_stub_statement(); }
    let file_handler = match &tokens[pos+2] {
        IdentifierToken(s) => s,
        _ => { panic!("Expect a file handle after @"); }
    };
    match &tokens[pos+3] {
        //read
        Token::OperatorGetFrom => {
            let var = match &tokens[pos+4] {
                IdentifierToken(s) => s,
                _ => { panic!("Expect a new variable"); }
            };
            let result = ReadAst{
                impure_procedure_name : procedure.to_owned(),
                file_handler : file_handler.to_owned(),
                write_to_variable: var.to_owned()
            };
            assert_eq!(tokens[pos+5], Token::Semicolon);
            debug!("Reading to var {}", var);
            (StatementAST::Read(result), Some(6))
        },
        //write
        Token::OperatorPutTo => {
            let mut len = 4;
            let (expr, expr_len) = try_read_expr(tokens, pos + len);
            let expr_len = match expr_len {
                Some(x) => x,
                None => { panic!("Expect a valid expr when write!")}
            };
            len += expr_len;
            assert_eq!(tokens[pos+len], Token::Semicolon);
            len += 1;

            let result = WriteAst {
                impure_procedure_name : procedure.to_owned(),
                file_handler : file_handler.to_owned(),
                expr: Box::from(expr)
            };
            info!("Write io");
            (StatementAST::Write(result), Some(len))
        },
        Token::OperatorAssign => {
            let filepath = match &tokens[pos+4] {
                Token::LiteralString(s) => s,
                _ => { panic!("Expect a Strin"); }
            };
            let result = FileOpenAst{
                impure_procedure_name : procedure.to_owned(),
                file_handler : file_handler.to_owned(),
                file_path: filepath.to_owned()
            };
            assert_eq!(tokens[pos+5], Token::Semicolon);
            (StatementAST::FileOpen(result), Some(6))
        }
        _ => { panic!("Expect << or >> or =, got {:?}", &tokens[pos+3]); }
    }
}

fn generate_invalid_let_binding_ast() -> LetBindingAST {
    LetBindingAST{ variable: INVALID_PLACEHOLDER.to_string(), expr: Box::new(ExprAST::Error) }
}

fn try_read_let_binding(tokens: &Vec<Token>, pos: usize) -> (LetBindingAST, Option<usize>) {
    debug!("try assignment {:?}", tokens[pos]);
    let mut len = 0;
    let stub = generate_invalid_let_binding_ast();
    if tokens[pos+len] != KeywordLet {
        return (stub, None);
    }
    len += 1;

    let var_name;
    if let IdentifierToken(name) = &tokens[pos+len] {
        var_name = name;
        debug!("identifier for assign {:?}", var_name);
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


#[derive(Clone)]
pub struct LetBindingAST {
    pub variable: String,
    pub expr : Box<ExprAST>
}

/* More formally, I should call it impure function.
However, I would make users safe to assume that all functions are pure
*/
#[derive(Clone)]
pub struct ReadAst {
    pub impure_procedure_name: String,
    pub file_handler: String,
    pub write_to_variable: String
}
#[derive(Clone)]
pub struct WriteAst {
    pub impure_procedure_name: String,
    pub file_handler: String,
    pub expr: Box<ExprAST>
}
#[derive(Clone)]
pub struct FileOpenAst {
    pub impure_procedure_name: String,
    pub file_handler: String,
    pub file_path: String
}