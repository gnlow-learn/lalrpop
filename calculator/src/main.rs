#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator4);
pub mod ast;

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66").unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}