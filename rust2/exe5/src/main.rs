use crate::expr::Expr;

mod error;
mod expr;
mod operator;
mod r#type;

fn main() {
    println!("Hello World\n");
    //let lexpr_string = "(let a 12 (+ a (if true 15 18)))";
    let lexpr_string = "(+ 12 2)";

    let f = Expr::parse_lexpr(lexpr_string).expect("Unable to parse");
    println!("f: {f:?}");
    println!("{:?}", f.eval());
}
