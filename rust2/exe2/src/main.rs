#[derive(Debug, PartialEq)]
enum Error {
    DivideByZero,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug, PartialEq)]
enum Expr<'a> {
    BinOp {
        l: &'a Expr<'a>,
        op: Operator,
        r: &'a Expr<'a>,
    },
    IfExpr {
        cond: &'a Expr<'a>,
        true_branch: &'a Expr<'a>,
        false_branch:&'a  Expr<'a>,
    },
    Literal(i32),
}

impl<'a> Expr<'a> {
    fn eval(&self) -> Result<i32, Error> {
        match self {
            Self::Literal(n) => Ok(*n),
            Self::BinOp { l, op, r } => {
                op.eval(l.eval()?, r.eval()?)
            },
            Self::IfExpr { cond, true_branch, false_branch } => {
                if cond.eval()? > 0 {
                    true_branch.eval()
                } else {
                    false_branch.eval()
                }
            },
        }
    } 
}

impl Operator {
    fn eval(&self, l1: i32, l2: i32) -> Result<i32, Error> {
        match self {
            Self::Plus => Ok(l1 + l2),
            Self::Minus => Ok(l1 - l2),
            Self::Multiply => Ok(l1 * l2),
            Self::Divide => {
                if l2 == 0 { Err(Error::DivideByZero) } else { Ok(l1 / l2) }
            },
        }
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn literal_test() {
        let literral = Expr::Literal(12).eval();
        assert_eq!(literral, Ok(12));
    }

    #[test]
    fn literal_add_test() {
        let l1 = Expr::Literal(12);
        let l2 = Expr::Literal(21);

        let add = Expr::BinOp { l: &l1, op: Operator::Plus, r: &l2 };
        assert_eq!(add.eval(), Ok(12 + 21));
    }

    #[test]
    fn true_condition_test() {
        let l1 = Expr::Literal(12);
        let true_branch = Expr::Literal(1);
        let false_branch = Expr::Literal(-1);
        let cond = Expr::IfExpr { cond: &l1, true_branch: &true_branch, false_branch: &false_branch };
        assert_eq!(cond.eval(), Ok(1));
    }

    #[test]
    fn false_condition_test() {
        let l1 = Expr::Literal(0);
        let true_branch = Expr::Literal(1);
        let false_branch = Expr::Literal(-1);
        let cond = Expr::IfExpr { cond: &l1, true_branch: &true_branch, false_branch: &false_branch };
        assert_eq!(cond.eval(), Ok(-1));
    }

    #[test]
    fn false_condition_test_2() {
        let l1 = Expr::Literal(-12);
        let true_branch = Expr::Literal(1);
        let false_branch = Expr::Literal(-1);
        let cond = Expr::IfExpr { cond: &l1, true_branch: &true_branch, false_branch: &false_branch };
        assert_eq!(cond.eval(), Ok(-1));
    }

    #[test]
    fn expression_complex_test() {
        let l1 = Expr::Literal(12);
        let false_branch = Expr::Literal(-1);
        let true_branch = Expr::BinOp { l: &Expr::Literal(12), op: Operator::Plus, r: &Expr::Literal(21) };
        let cond = Expr::IfExpr { cond: &l1, true_branch: &true_branch, false_branch: &false_branch };
        assert_eq!(cond.eval(), Ok(12 + 21));
    }

    #[test]
    fn divide_zero_test() {
        let l1 = Expr::Literal(12);
        let false_branch = Expr::Literal(-1);
        let true_branch = Expr::BinOp { l: &Expr::Literal(12), op: Operator::Divide, r: &Expr::Literal(0) };
        let cond = Expr::IfExpr { cond: &l1, true_branch: &true_branch, false_branch: &false_branch };
        assert_eq!(cond.eval(), Err(Error::DivideByZero));
    }
}
