mod ast;
mod eval;
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);
#[test]
fn calc() {
    let expr = parser::exprParser::new().parse("22 * 44 + 66").unwrap();
    println!("{:?}", expr.clone());
    assert_eq!(&format!("{:?}", expr),
        "Add(Mul(Int(22), Int(44)), Int(66))");
}
fn main() {
    println!("Hello, world!");
}
