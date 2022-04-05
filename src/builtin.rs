// This file should only be used at runtime

use log::{debug, info};
use crate::debug_output::build_expr_debug_strings;
use crate::expr::ExprAST;

pub const LIST_OPERATIONS: &[&str; 3] = &["cons", "car", "cdr"];
pub const ARITHMETIC_OPERATORS: &[&str; 8] = &["<=", ">=", "+", "-", "*", "==", ">", "<", ];

pub fn perform_write(method_name:&str, file_handler: &str, data:&ExprAST) {
    assert_eq!(method_name, "writeline");
    assert_eq!(file_handler, "stdout");
    writeline(data);
}

fn write(e: &ExprAST) {
    match e {
        ExprAST::Int(x) => print!("{}", x),
        ExprAST::Bool(x) => {
            if *x {print!("true")} else {print!("false")}
        }
        _ => todo!()
    }
}
fn writeline(e: &ExprAST) {
    write(e);
    print!("\n");
}

enum ArithmeticCalcOp {
    Add, Minus, Multiple
}

pub fn call_builtin_function(func_name: &str, params: Vec<ExprAST>) -> ExprAST {
    debug!("Called builtin {}", func_name);
    match func_name {
        "==" => arithmetic_equal(&params),
        "+"  => arithmetic_calc(ArithmeticCalcOp::Add, &params),
        "-"  => arithmetic_calc(ArithmeticCalcOp::Minus, &params),
        "*"  => arithmetic_calc(ArithmeticCalcOp::Multiple, &params),
        _ => panic!("Builtin function ({}) not found", func_name)
    }
}
fn arithmetic_calc(op: ArithmeticCalcOp, p: &Vec<ExprAST>) -> ExprAST {
    assert_eq!(p.len(), 2);
    let a = unpack_num(&p[0]);
    let b = unpack_num(&p[1]);
    let result = match op {
        ArithmeticCalcOp::Add => a + b,
        ArithmeticCalcOp::Minus => a - b,
        ArithmeticCalcOp::Multiple => a * b,
    };
    ExprAST::Int(result)
}

fn arithmetic_equal(p: &Vec<ExprAST>) -> ExprAST {
    assert_eq!(p.len(), 2);
    let a = unpack_num(&p[0]);
    let b = unpack_num(&p[1]);
    if a == b {
        ExprAST::Bool(true)
    } else {
        ExprAST::Bool(false)
    }
}

fn unpack_num(e: &ExprAST) -> i64 {
    match e {
        ExprAST::Int(x) => *x,
        _ => panic!("Expected int, got {:?}", build_expr_debug_strings(e))
    }
}



pub struct IroncamelLinkedList {
    value: Box<ExprAST>,
    len: usize, // Allows us to calculate list size with O(1) cost
    next: std::rc::Rc<IroncamelLinkedList>
}

impl Clone for IroncamelLinkedList {
    fn clone(&self) -> Self {
        IroncamelLinkedList {
            value: self.value.clone(),
            len: self.len,
            next: std::rc::Rc::clone(&self.next)
            // https://stackoverflow.com/a/61950053/1166518
        }
    }
}