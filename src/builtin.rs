// This file should only be used at runtime


use std::io::{BufRead, Write};
use std::rc::Rc;
use log::{debug, info};
use crate::debug_output::build_expr_debug_strings;
use crate::expr::ExprAST;
use crate::interpreter::{GlobalState, IroncamelFileInfo};

pub const IRONCAMEL_BUILTIN_FUNCTIONS: &[&str; 7] = &["cons", "hd", "tl", "list", "is_empty",
    "atoi", "strtok"];
pub const ARITHMETIC_OPERATORS: &[&str; 8] = &["<=", ">=", "+", "-", "*", "==", ">", "<", ];
pub const IO_OPERATIONS: &[&str; 5] = &["readstr", "writeline", "writelist", "fopen_read", "fopen_write"];

pub(crate) fn perform_read(method_name:&str, file_handler: &str, global_state: &mut GlobalState) -> ExprAST {
    let mut fop = global_state.open_file_list.get_mut(file_handler).unwrap();
    match method_name {
        "readstr" => {
            let mut s = String::new();
            match fop {
                IroncamelFileInfo::FileRead(buf) => {
                    buf.read_line(&mut s);
                }
                IroncamelFileInfo::FileWrite(_) | IroncamelFileInfo::Stdout => { panic!() }
                IroncamelFileInfo::Stdin => {
                    let mut t = String::new();
                    std::io::stdin().read_line(&mut t).unwrap();
                    s = t.trim_end().to_owned();
                }
            };
            let expr = ExprAST::StringLiteral(s);
            expr
        },
        _ => panic!("No such write function ({})", method_name)
    }
}

pub fn perform_write(method_name:&str, file_handler: &str, data:&ExprAST, global_state: &mut GlobalState) {
    let mut fop: &mut IroncamelFileInfo = global_state.open_file_list.get_mut(file_handler).unwrap();
    match method_name {
        "writeline" => writeline(data, fop),
        "writelist" => writelist(data, fop),
        _ => panic!("No such write function ({})", method_name)
    }
}

fn writelist(list: &ExprAST, fop: &mut IroncamelFileInfo) {
    let mut list = match  list {
        ExprAST::List(l) => l,
        _ => panic!("Expect a list, got {:?}", build_expr_debug_strings(list)),
    };
    while list.len > 0 {
        write(&list.value, fop);
        write_internal(" ", fop);
        list = &*match &list.next {
            Some(l) => l,
            None => break
        }
    }
    write_internal("\n", fop);
}

fn write_internal(s: &str, fop: &mut IroncamelFileInfo) {
    match fop {
        IroncamelFileInfo::FileWrite(fs) => {
            fs.write_all(s.as_ref());
        },
        IroncamelFileInfo::Stdout => {
            print!("{}", s);
        },
        IroncamelFileInfo::FileRead(_) | IroncamelFileInfo::Stdin => {panic!()}

    }
}
fn write(e: &ExprAST, fop: &mut IroncamelFileInfo) {
    match e {
        ExprAST::Int(x) => write_internal(&x.to_string(), fop),
        ExprAST::Bool(x) => {
            if *x {write_internal("true", fop)} else {write_internal("false", fop)}
        }
        ExprAST::StringLiteral(s) => write_internal(s, fop),
        _ => panic!("Unsupported expr: {:?}", build_expr_debug_strings(e))
    }
}
fn writeline(e: &ExprAST, fop: &mut IroncamelFileInfo) {
    write(e, fop);
    write_internal("\n", fop);
}

enum ArithmeticCalcOp {
    Add, Minus, Multiple
}
enum ArithmeticCmpOp {
    Gt, Lt, Geq, Leq, Eq
}

pub fn call_builtin_function(func_name: &str, params: Vec<ExprAST>) -> ExprAST {
    debug!("Called builtin {} with params: {:?}", func_name, &params);
    match func_name {
        "==" => arithmetic_cmp(ArithmeticCmpOp::Eq, &params),
        ">" => arithmetic_cmp(ArithmeticCmpOp::Gt, &params),
        "<" => arithmetic_cmp(ArithmeticCmpOp::Lt, &params),
        ">=" => arithmetic_cmp(ArithmeticCmpOp::Geq, &params),
        "<=" => arithmetic_cmp(ArithmeticCmpOp::Leq, &params),
        "+"  => arithmetic_calc(ArithmeticCalcOp::Add, &params),
        "-"  => arithmetic_calc(ArithmeticCalcOp::Minus, &params),
        "*"  => arithmetic_calc(ArithmeticCalcOp::Multiple, &params),
        "list" => {
            ExprAST::List(Rc::new(IroncamelLinkedList::build_list(params.as_slice())))
        }
        "cons" => {
            assert_eq!(params.len(), 2);
            let tail = match &params[1] {
                ExprAST::List(l) => l,
                _ => panic!("Expect a list as the second param, got {:?}",&params[1])
            };
            let result = IroncamelLinkedList::cons(params[0].clone(),
                                                   tail);
            ExprAST::List(Rc::new(result))
        },
        "hd" => {
            assert_eq!(params.len(), 1);
            match &params[0] {
                ExprAST::List(l) => l.hd().clone(),
                _ => panic!("Expect a list, got {:?}",&params[0])
            }
        },
        "tl" => {
            assert_eq!(params.len(), 1);
            match &params[0] {
                ExprAST::List(l) => {
                    match l.tl() {
                        Some(t) => ExprAST::List(t),
                        None => build_empty_list_expr()
                    }
                },
                _ => panic!("Expect a list, got {:?}",&params[0])
            }
        },
        "is_empty" => {
            assert_eq!(params.len(), 1);
            match &params[0] {
                ExprAST::List(l) => {
                    let r = l.len == 0;
                    ExprAST::Bool(r)
                },
                _ => panic!("Expect a list, got {:?}",&params[0])
            }
        },
        "atoi" => {
            assert_eq!(params.len(), 1);
            match &params[0] {
                ExprAST::StringLiteral(s) => {
                    let x:i64 = s.parse().unwrap();
                    ExprAST::Int(x)
                },
                _ => panic!("Expect a String, got {:?}",&params[0])
            }
        },
        "strtok" => {
            assert_eq!(params.len(), 2);
            let origin_str = match &params[0] {
                ExprAST::StringLiteral(s) => s,
                _ => panic!("Expect a String, got {:?}",&params[0])
            };
            let delims = match &params[1] {
                ExprAST::StringLiteral(s) => s,
                _ => panic!("Expect a String, got {:?}",&params[0])
            };
            debug!("got delims {}", delims);
            let delim_list: Vec<char> = delims.chars().collect();
            debug!("We have {} delims", delim_list.len());
            let split_str: Vec<&str> = origin_str.split(&delim_list[..]).collect();
            debug!("split: {:?}", split_str);
            let mut result: Vec<ExprAST> = Vec::new();
            for s in split_str {
                if s.is_empty() { continue }
                let e = ExprAST::StringLiteral(s.to_owned());
                result.push(e);
            }
            ExprAST::List(
                Rc::new(IroncamelLinkedList::build_list(&result)
                ))

        },
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

fn arithmetic_cmp(op: ArithmeticCmpOp, p: &Vec<ExprAST>) -> ExprAST {
    assert_eq!(p.len(), 2);
    let a = unpack_num(&p[0]);
    let b = unpack_num(&p[1]);
    let result = match op {
        ArithmeticCmpOp::Eq => a == b,
        ArithmeticCmpOp::Gt => a > b,
        ArithmeticCmpOp::Lt => a < b,
        ArithmeticCmpOp::Geq => a >= b,
        ArithmeticCmpOp::Leq => a <= b
    };
    ExprAST::Bool(result)
}

fn unpack_num(e: &ExprAST) -> i64 {
    match e {
        ExprAST::Int(x) => *x,
        _ => panic!("Expected int, got {:?}", build_expr_debug_strings(e))
    }
}


pub struct IroncamelLinkedList {
    value: Box<ExprAST>,
    pub(crate) len: usize, // Allows us to calculate list size with O(1) cost
    next: Option<std::rc::Rc<IroncamelLinkedList>>
}

pub fn build_empty_list_expr() -> ExprAST {
    ExprAST::List(Rc::new(IroncamelLinkedList::build_empty_list()))
}
impl IroncamelLinkedList {
    pub(crate) fn build_empty_list() -> IroncamelLinkedList{
        IroncamelLinkedList {
            value: Box::new(ExprAST::Error),
            len: 0,
            next: None
        }
    }
    pub fn build_list(exprs: &[ExprAST]) -> IroncamelLinkedList {
        if exprs.len() == 0 {
            IroncamelLinkedList::build_empty_list()
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
    pub fn cons(expr: ExprAST, tail: &Rc<IroncamelLinkedList>) -> IroncamelLinkedList {
        IroncamelLinkedList {
            value: Box::new(expr),
            len: 1 + tail.len,
            next: Some(std::rc::Rc::clone(tail)),
        }
    }
    pub fn hd(&self) -> &ExprAST {
        assert!(self.len > 0);
        &self.value
    }
    pub fn tl(&self) -> Option<Rc<IroncamelLinkedList>> {
        assert!(self.len > 0);
        if self.len > 1 {
            let tail = self.next.as_ref().unwrap();
            Some(std::rc::Rc::clone(&tail))
        } else {
            None
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

        let v = l2.hd();
        match v {
            ExprAST::Int(x) => assert_eq!(*x, 42),
            _ => assert!(false)
        };

        let l3 = l2.tl();
        match l3 {
            Some(l3) => assert_eq!(l3.as_vector_i64(), vec![5]),
            None => assert!(false)
        };

    }
}

