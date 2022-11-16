#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub calculator3);

use calculator3::ExprParser;

//#[test]
fn main() {
    assert!(ExprParser::new().parse("22").is_ok());
    assert!(ExprParser::new().parse("(22)").is_ok());
    assert!(ExprParser::new().parse("((((22))))").is_ok());
    assert!(ExprParser::new().parse("((22)").is_err());
    println!("{:?}", ExprParser::new().parse("2 + 3 * 4"));
}