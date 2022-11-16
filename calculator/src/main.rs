#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator5);
pub mod ast;

#[test]
fn calculator5() {
    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13 * 3").unwrap();
    println!("{:?}", expr);
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");
}