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
        match self {
            Expr::Literal(i) => return *i,
            Expr::BinOp { l, op, r } =>
                match op {
                    Operator::Plus => return l.eval() + r.eval(),
                    Operator::Minus => return l.eval() - r.eval(),
                    Operator::Multiply=> return l.eval() * r.eval(),
                    Operator::Divide =>
                        if r.eval() != 0 {
                            return l.eval() / r.eval()
                        }
                        else {
                            return -1;
                        }
                },
            Expr::IfExpr { cond, true_branch, false_branch } => 
                if cond.eval() != 0 {
                    return true_branch.eval();
                }
                else {
                    return false_branch.eval();
                },
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

#[test]
fn test_minus() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let min = Expr::BinOp { l: Box::new(l1), op: Operator::Minus, r: Box::new(l2) };
    assert_eq!(min.eval(), 10 - 2);
}

#[test]
fn test_multiply() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let mult = Expr::BinOp{l: Box::new(l1), op: Operator::Multiply, r: Box::new(l2)};
    assert_eq!(mult.eval(), 10 * 2);
}

#[test]
fn test_multiply_by_0() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(0);

    let mult = Expr::BinOp{l: Box::new(l1), op: Operator::Multiply, r: Box::new(l2)};
    assert_eq!(mult.eval(), 10 * 0);
}

#[test]
fn test_divide() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let div = Expr::BinOp{l: Box::new(l1), op: Operator::Divide, r: Box::new(l2)};
    assert_eq!(div.eval(), 10 / 2);
}

#[test]
fn test_divide_by_0() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(0);

    let div = Expr::BinOp{l: Box::new(l1), op: Operator::Divide, r: Box::new(l2)};
    assert_eq!(div.eval(), -1);
}

#[test]
fn test_if_expr_true() {
    let l1 = Expr::Literal(10);

    let test = Expr::IfExpr{cond: Box::new(l1), true_branch: Box::new(Expr::Literal(0)), false_branch: Box::new(Expr::Literal(-1))};
    assert_eq!(test.eval(), 0);
}

#[test]
fn test_if_expr_false() {
    let l1 = Expr::Literal(0);

    let test = Expr::IfExpr{cond: Box::new(l1), true_branch: Box::new(Expr::Literal(0)), false_branch: Box::new(Expr::Literal(-1))};
    assert_eq!(test.eval(), -1);
}
