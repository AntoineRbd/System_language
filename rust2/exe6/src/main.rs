use crate::expr::Expr;

mod binop;
mod boolean;
mod error;
mod expr;
mod ifexpr;
mod letexpr;
mod literal;
mod operator;
mod refexpr;
mod r#type;
mod print;

fn main() {
    println!("Hello World\n");
    let lexpr_string = "(+ 12 2)";
}
