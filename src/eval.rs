use crate::ast::{Expr, Expr::*};

fn eval(expr: Expr) -> i32 {
    match expr {
        Int(i) => i,
        Add(a,b) => eval(*a) + eval(*b),
        Sub(a,b) => eval(*a) - eval(*b),
        Mul(a,b) => eval(*a) * eval(*b),
        Div(a,b) => eval(*a) / eval(*b),
    }
}

#[test]
fn test_eval() {
    assert_eq!(eval(Add(Box::new(Int(1)),Box::new(Int(2)))), 3);
}
