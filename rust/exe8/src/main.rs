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

fn eval(e: Expr) -> i32 {
    return match e {
        Expr::Literal(a) => a,
        Expr::IfExpr { cond, true_branch, false_branch } => eval(*IfExpr(cond, true_branch, false_branch)),
        Expr::BinOp { l, op, r } => eval(*BinOp(l, op, r)),
        _ => 0 as i32
    };
}

fn BinOp(l: Box<Expr>, op: Operator, r: Box<Expr>) -> Box<Expr> {
    let intR = eval(*r);
    let intL = eval(*l);

    return match op {
        Operator::Plus => Box::new(Expr::Literal(intL + intR)),
        Operator::Minus => Box::new(Expr::Literal(intL - intR)),
        Operator::Multiply=> Box::new(Expr::Literal(intL * intR)),
        Operator::Divide => {
            if intR != 0 {
                return Box::new(Expr::Literal(intL / intR));
            }
            else {
                return Box::new(Expr::Literal(-1));
            }
        },
        _ => Box::new(Expr::Literal(-1))

    };
}

fn IfExpr(cond: Box<Expr>, true_branch: Box<Expr>, false_branch: Box<Expr>) -> Box<Expr> {
    if eval(*cond) != 0 {
        return true_branch;
    }
    else {
        return false_branch;
    }
}

fn main() {
}

#[test]
fn test_plus() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let add = Expr::BinOp { l: Box::new(l1), op: Operator::Plus, r: Box::new(l2) };
    assert_eq!(eval(add), 10 + 2);
}

#[test]
fn test_minus() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let min = Expr::BinOp { l: Box::new(l1), op: Operator::Minus, r: Box::new(l2) };
    assert_eq!(eval(min), 10 - 2);
}

#[test]
fn test_multiply() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let mult = Expr::BinOp{l: Box::new(l1), op: Operator::Multiply, r: Box::new(l2)};
    assert_eq!(eval(mult), 10 * 2);
}

#[test]
fn test_multiply_by_0() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(0);

    let mult = Expr::BinOp{l: Box::new(l1), op: Operator::Multiply, r: Box::new(l2)};
    assert_eq!(eval(mult), 10 * 0);
}

#[test]
fn test_divide() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(2);

    let div = Expr::BinOp{l: Box::new(l1), op: Operator::Divide, r: Box::new(l2)};
    assert_eq!(eval(div), 10 / 2);
}

#[test]
fn test_divide_by_0() {
    let l1 = Expr::Literal(10);
    let l2 = Expr::Literal(0);

    let div = Expr::BinOp{l: Box::new(l1), op: Operator::Divide, r: Box::new(l2)};
    assert_eq!(eval(div), -1);
}

#[test]
fn test_if_expr_true() {
    let l1 = Expr::Literal(10);

    let test = Expr::IfExpr{cond: Box::new(l1), true_branch: Box::new(1), false_branch: Box::new(0)};
    assert_eq!(eval(test), );
}
