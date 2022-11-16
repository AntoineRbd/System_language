use crate::{expr::Expr, error::Error};

pub fn Print(expr: Box<dyn Expr>) -> Result<String, Error>{
    let res = expr.eval()?.to_string();
    println!("{}", res);
    Ok(res)
}


#[cfg(test)]
mod test {
    use crate::{ binop::BinOp, boolean::Boolean, ifexpr::IfExpr, letexpr::LetExpr, literal::Literal,
        refexpr::RefExpr,
    };

    use super::*;

    #[test]
    fn simple_literal_test() -> Result<(), Error>{
        let l = Box::new(Literal(12));
        let literral = Print(l)?;
        assert_eq!(literral, "12".to_string());
        Ok(())
    }
}

