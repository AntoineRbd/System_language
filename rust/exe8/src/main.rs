#[derive(Debug)]
enum Operator {
    Plus, Minus, Divide, Multiply
}

#[derive(Debug)]
enum Expr {
    BinOp { l: Box<Expr>, op: Operator, r: Box<Expr> },
    IfExpr { cond: Box<Expr>, true_branch: Box<Expr>, false_branch: Box<Expr> },
    Literal(i32)
}

impl Expr {
    fn eval(&self) -> i32 {
        return match self {
            Expr::Literal(a) => *a,
            //Expr::IfExpr { cond, true_branch, false_branch } => eval(*IfExpr(cond, true_branch, false_branch)),
            Expr::BinOp { l, op, r } => self.eval(self.BinOp(l, op, r, self)),
            _ => 0 as i32
        };
    }

    fn BinOp(&self, l: Box<Expr>, op: Operator, r: box<Expr>) -> Expr::Literal {
        return match op {
            Operator::Plus => self.eval(l) + self.eval(r),
            Operator::Minus => self.eval(l) - self.eval(r),
            Operator::Divide => self.eval(l) / self.eval(r),
            Operator::Multiply=> self.eval(l) * self.eval(r),
        };
    }
}

fn main() {
}

#[test]
fn test_plus() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let add = Expr::BinOp { l: Box::new(l1), op: Operator::Plus, r: Box::new(l2) };
    assert_eq!(add.eval(), 10 + 2);
}
