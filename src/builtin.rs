// This file should only be used at runtime

use crate::expr::ExprAST;

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
