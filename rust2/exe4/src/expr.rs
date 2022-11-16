use std::collections::HashMap;

use crate::{r#type::Type, error::Error, operator::Operator};

#[derive(Debug, PartialEq)]
pub enum Expr {
    BinOp {
        l: Box<Expr>,
        op: Operator,
        r: Box<Expr>,
    },
    IfExpr {
        cond: Box<Expr>,
        true_branch: Box<Expr>,
        false_branch: Box<Expr>,
    },
    LetExpr {
        name: String,
        value: Box<Expr>,
        next: Box<Expr>,
    },
    RefExpr {
        name: String,
    },
    Boolean(bool),
    Literal(i32),
}

type Binder = HashMap<String, Type>;
impl Expr {
    fn eval_with_binder(&self, binder: &mut Binder) -> Result<Type, Error> {
        match self {
            Self::Literal(n) => Ok(Type::Literal(*n)),
            Self::BinOp { l, op, r } => {
                if let (Type::Literal(left_value), Type::Literal(right_value)) =
                    (l.eval_with_binder(binder)?, r.eval_with_binder(binder)?)
                {
                    Ok(Type::Literal(op.eval(left_value, right_value)?))
                } else {
                    Err(Error::InvalidType)
                }
            }
            Self::IfExpr {
                cond,
                true_branch,
                false_branch,
            } => {
                if let Type::Boolean(cond) = cond.eval_with_binder(binder)? {
                    if cond {
                        true_branch.eval_with_binder(binder)
                    } else {
                        false_branch.eval_with_binder(binder)
                    }
                } else {
                    Err(Error::InvalidType)
                }
            }
            Self::LetExpr { name, value, next } => {
                let value = value.eval_with_binder(binder)?;
                binder.insert(name.clone(), value);
                let res = next.eval_with_binder(binder)?;
                binder.remove(name);
                Ok(res)
            }
            Self::RefExpr { name } => {
                if !binder.contains_key(name) {
                    Err(Error::UnknowVariable)
                } else {
                    Ok(binder[name])
                }
            }
            Self::Boolean(v) => Ok(Type::Boolean(*v)),
        }
    }

    pub fn eval(&self) -> Result<Type, Error> {
        let mut binder = Binder::new();
        self.eval_with_binder(&mut binder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_literal_test() {
        let literral = Expr::Literal(12).eval();
        assert_eq!(literral, Ok(Type::Literal(12)));
    }

    #[test]
    fn simple_add_literal_test() {
        let l1 = Expr::Literal(12);
        let l2 = Expr::Literal(21);

        let add = Expr::BinOp {
            l: Box::new(l1),
            op: Operator::Plus,
            r: Box::new(l2),
        };
        assert_eq!(add.eval(), Ok(Type::Literal(12 + 21)));
    }

    #[test]
    fn simple_true_condition() {
        let l1 = Expr::Boolean(true);
        let true_branch = Expr::Literal(1);
        let false_branch = Expr::Literal(-1);
        let cond = Expr::IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(1)));
    }

    #[test]
    fn simple_false_condition() {
        let l1 = Expr::Boolean(false);
        let true_branch = Expr::Literal(1);
        let false_branch = Expr::Literal(-1);
        let cond = Expr::IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(-1)));
    }

    #[test]
    fn simple_false_condition2() {
        let l1 = Expr::Boolean(false);
        let true_branch = Expr::Literal(1);
        let false_branch = Expr::Literal(-1);
        let cond = Expr::IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(-1)));
    }

    #[test]
    fn complex_expression_test() {
        let l1 = Expr::Boolean(true);
        let false_branch = Expr::Literal(-1);
        let true_branch = Expr::BinOp {
            l: Box::new(Expr::Literal(12)),
            op: Operator::Plus,
            r: Box::new(Expr::Literal(21)),
        };
        let cond = Expr::IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(12 + 21)));
    }

    #[test]
    fn divide_zero_test() {
        let l1 = Expr::Boolean(true);
        let false_branch = Expr::Literal(-1);
        let true_branch = Expr::BinOp {
            l: Box::new(Expr::Literal(12)),
            op: Operator::Divide,
            r: Box::new(Expr::Literal(0)),
        };
        let cond = Expr::IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Err(Error::DivideByZero));
    }

    #[test]
    fn simple_let_ref_test() {
        let next_expr = Expr::RefExpr {
            name: "a".to_string(),
        };
        let let_expr = Expr::LetExpr {
            name: "a".to_string(),
            value: Box::new(Expr::Literal(12)),
            next: Box::new(next_expr),
        };

        assert_eq!(let_expr.eval(), Ok(Type::Literal(12)));
    }

    #[test]
    fn unknow_variable_test() {
        let next_expr = Expr::RefExpr {
            name: "a".to_string(),
        };
        let let_expr = Expr::LetExpr {
            name: "b".to_string(),
            value: Box::new(Expr::Literal(12)),
            next: Box::new(next_expr),
        };
        assert_eq!(let_expr.eval(), Err(Error::UnknowVariable));
    }

    #[test]
    fn unknow_variable2_test() {
        let next_expr = Expr::RefExpr {
            name: "a".to_string(),
        };
        let let_expr = Expr::LetExpr {
            name: "a".to_string(),
            value: Box::new(Expr::Literal(12)),
            next: Box::new(next_expr),
        };

        let a_expr = Expr::RefExpr {
            name: "a".to_string(),
        };
        let bin_op = Expr::BinOp {
            l: Box::new(let_expr),
            op: Operator::Plus,
            r: Box::new(a_expr),
        };

        assert_eq!(bin_op.eval(), Err(Error::UnknowVariable));
    }

    #[test]
    fn wrong_type_if_condition_test() {
        let l1 = Expr::Literal(3);
        let false_branch = Expr::Literal(-1);
        let true_branch = Expr::BinOp {
            l: Box::new(Expr::Literal(12)),
            op: Operator::Plus,
            r: Box::new(Expr::Literal(21)),
        };
        let cond = Expr::IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Err(Error::InvalidType));
    }

    #[test]
    fn wrong_type_operator_test() {
        let op = Expr::BinOp {
            l: Box::new(Expr::Boolean(true)),
            op: Operator::Plus,
            r: Box::new(Expr::Literal(21)),
        };
        assert_eq!(op.eval(), Err(Error::InvalidType));
    }
}

