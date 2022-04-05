// This file should only be used at runtime

use std::rc::Rc;
use log::{debug, info};
use crate::debug_output::build_expr_debug_strings;
use crate::expr::ExprAST;

pub const LIST_OPERATIONS: &[&str; 4] = &["cons", "car", "cdr", "list"];
pub const ARITHMETIC_OPERATORS: &[&str; 8] = &["<=", ">=", "+", "-", "*", "==", ">", "<", ];
pub const WRITE_OPERATIONS: &[&str; 2] = &["writeline", "writelist"];

pub fn perform_write(method_name:&str, file_handler: &str, data:&ExprAST) {
    assert_eq!(file_handler, "stdout");
    match method_name {
        "writeline" => writeline(data),
        "writelist" => writelist(data),
        _ => panic!("No such write function ({})", method_name)
    }
}

fn writelist(list: &ExprAST) {
    let mut list = match  list {
        ExprAST::List(l) => l,
        _ => panic!("Expect a list, got {:?}", build_expr_debug_strings(list)),
    };
    while list.len > 0 {
        write(&list.value);
        print!(" ");
        list = &*match &list.next {
            Some(l) => l,
            None => break
        }
    }
    print!("\n");
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
        "list" => {
            ExprAST::List(IroncamelLinkedList::build_list(params.as_slice()))
        }
        "cons" => {
            assert_eq!(params.len(), 2);
            let tail = match &params[1] {
                ExprAST::List(l) => l,
                _ => panic!("Expect a list as the second param, got {:?}",&params[1])
            };
            let result = IroncamelLinkedList::cons(params[0].clone(),
                                                   &Rc::new(tail.clone()));
            ExprAST::List(result)
        }
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
    fn build_list(exprs: &[ExprAST]) -> IroncamelLinkedList {
        if exprs.len() == 0 {
            IroncamelLinkedList {
                value: Box::new(ExprAST::Error),
                len: 0,
                next: None
            }
        } else {
            if exprs.len() == 1 {
                IroncamelLinkedList::build(exprs[0].clone())
            } else {
                let tail_exprs = &exprs[1..];
                assert_eq!(exprs.len(), tail_exprs.len()+1);
                let tail_link = IroncamelLinkedList::build_list(tail_exprs);
                assert_eq!(tail_link.len, tail_exprs.len());
                IroncamelLinkedList::cons(exprs[0].clone(), &Rc::new(tail_link))
            }
        }
    }
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
    pub fn car(&self) -> &ExprAST {
        assert!(self.len > 0);
        &self.value
    }
    pub fn cdr(&self) -> Option<Rc<IroncamelLinkedList>> {
        match &self.next {
            Some(tail) => Some(std::rc::Rc::clone(tail)),
            None => None
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

    #[test]
    fn car_cdr_list() {
        let l1 = IroncamelLinkedList::build(gei(5));
        assert_eq!(l1.as_vector_i64(), vec![5]);
        let l2 = IroncamelLinkedList::cons(gei(42), &std::rc::Rc::new(l1));
        assert_eq!(l2.as_vector_i64(), vec![42, 5]);

        let v = l2.car();
        match v {
            ExprAST::Int(x) => assert_eq!(*x, 42),
            _ => assert!(false)
        };

        let l3 = l2.cdr();
        match l3 {
            Some(l3) => assert_eq!(l3.as_vector_i64(), vec![5]),
            None => assert!(false)
        };

    }
}
