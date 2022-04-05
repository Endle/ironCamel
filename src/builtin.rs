// This file should only be used at runtime

use std::rc::Rc;
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
    next: Option<std::rc::Rc<IroncamelLinkedList>>
}

impl IroncamelLinkedList {
    pub fn build(expr: ExprAST) -> IroncamelLinkedList {
        IroncamelLinkedList {
            value: Box::new(expr),
            len: 1,
            next: None
        }
    }
    pub fn cons(expr: ExprAST, tail: &std::rc::Rc<IroncamelLinkedList>) -> IroncamelLinkedList {
        IroncamelLinkedList {
            value: Box::new(expr),
            len: 1 + tail.len,
            next: Some(std::rc::Rc::clone(tail)),
        }
    }

    pub fn as_vector(&self) -> Vec<Box<ExprAST>> {
        if self.len == 1 {
            vec![self.value.clone()]
        } else {
            match &self.next {
                None => { panic!("Expected to have a tail") }
                Some(next) => {
                    assert_eq!(next.len+1, self.len);
                    let mut result = next.as_vector();
                    result.push(self.value.clone());
                    result
                }
            }
        }
    }
    pub fn as_vector_i64(&self) -> Vec<i64> {
        let exprs = self.as_vector();
        let mut result = Vec::with_capacity(exprs.len());
        assert_eq!(exprs.len(), self.len);
        for i in 0..self.len {
            let v = match *exprs[i] {
                ExprAST::Int(x) => x,
                _ => panic!("Expect an integer")
            };
            result.push(v);
        }
        result.reverse();
        result
    }
}

impl Clone for IroncamelLinkedList {
    fn clone(&self) -> Self {
        // https://stackoverflow.com/a/61950053/1166518
        let next = match &self.next {
            Some(s) => Some(std::rc::Rc::clone(s)),
            None => None
        };
        IroncamelLinkedList {
            value: self.value.clone(),
            len: self.len,
            next
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::builtin::IroncamelLinkedList;
    use crate::expr::ExprAST;

    fn gei(x:i64) -> ExprAST { //generate expr int
        ExprAST::Int(x)
    }
    #[test]
    fn build_linkedlist() {
        let list = IroncamelLinkedList::build(gei(5));
        assert_eq!(list.len, 1);
    }

    #[test]
    fn insert_to_list() {
        let l1 = IroncamelLinkedList::build(gei(5));
        assert_eq!(l1.as_vector_i64(), vec![5]);
        let l2 = IroncamelLinkedList::cons(gei(42), &std::rc::Rc::new(l1));
        assert_eq!(l2.as_vector_i64(), vec![42, 5]);
    }
}
