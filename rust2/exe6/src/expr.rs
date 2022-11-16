use std::collections::HashMap;

use lexpr::{cons::ListIter, Number, Value};

use crate::{error::Error, operator::Operator, r#type::Type};

pub type Binder = HashMap<String, Type>;
pub trait Expr {
    fn eval_with_binder(&self, binder: &mut Binder) -> Result<Type, Error>;

    fn eval(&self) -> Result<Type, Error> {
        let mut binder = Binder::new();
        self.eval_with_binder(&mut binder)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        binop::BinOp, boolean::Boolean, ifexpr::IfExpr, letexpr::LetExpr, literal::Literal,
        refexpr::RefExpr,
    };

    use super::*;

    #[test]
    fn simple_literal_test() {
        let literral = Literal(12).eval();
        assert_eq!(literral, Ok(Type::Literal(12)));
    }

    #[test]
    fn simple_add_literal_test() {
        let l1 = Literal(12);
        let l2 = Literal(21);

        let add = BinOp {
            l: Box::new(l1),
            op: Operator::Plus,
            r: Box::new(l2),
        };
        assert_eq!(add.eval(), Ok(Type::Literal(12 + 21)));
    }

    #[test]
    fn simple_true_condition() {
        let l1 = Boolean(true);
        let true_branch = Literal(1);
        let false_branch = Literal(-1);
        let cond = IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(1)));
    }

    #[test]
    fn simple_false_condition() {
        let l1 = Boolean(false);
        let true_branch = Literal(1);
        let false_branch = Literal(-1);
        let cond = IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(-1)));
    }

    #[test]
    fn simple_false_condition2() {
        let l1 = Boolean(false);
        let true_branch = Literal(1);
        let false_branch = Literal(-1);
        let cond = IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(-1)));
    }

    #[test]
    fn complex_expression_test() {
        let l1 = Boolean(true);
        let false_branch = Literal(-1);
        let true_branch = BinOp {
            l: Box::new(Literal(12)),
            op: Operator::Plus,
            r: Box::new(Literal(21)),
        };
        let cond = IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Ok(Type::Literal(12 + 21)));
    }

    #[test]
    fn divide_zero_test() {
        let l1 = Boolean(true);
        let false_branch = Literal(-1);
        let true_branch = BinOp {
            l: Box::new(Literal(12)),
            op: Operator::Divide,
            r: Box::new(Literal(0)),
        };
        let cond = IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Err(Error::DivideByZero));
    }

    #[test]
    fn simple_let_ref_test() {
        let next_expr = RefExpr {
            name: "a".to_string(),
        };
        let let_expr = LetExpr {
            name: "a".to_string(),
            value: Box::new(Literal(12)),
            next: Box::new(next_expr),
        };

        assert_eq!(let_expr.eval(), Ok(Type::Literal(12)));
    }

    #[test]
    fn unknow_variable_test() {
        let next_expr = RefExpr {
            name: "a".to_string(),
        };
        let let_expr = LetExpr {
            name: "b".to_string(),
            value: Box::new(Literal(12)),
            next: Box::new(next_expr),
        };
        assert_eq!(let_expr.eval(), Err(Error::UnknowVariable));
    }

    #[test]
    fn unknow_variable2_test() {
        // let a = 12; in a;
        // + a
        let next_expr = RefExpr {
            name: "a".to_string(),
        };
        let let_expr = LetExpr {
            name: "a".to_string(),
            value: Box::new(Literal(12)),
            next: Box::new(next_expr),
        };

        let a_expr = RefExpr {
            name: "a".to_string(),
        };
        let bin_op = BinOp {
            l: Box::new(let_expr),
            op: Operator::Plus,
            r: Box::new(a_expr),
        };

        assert_eq!(bin_op.eval(), Err(Error::UnknowVariable));
    }

    #[test]
    fn wrong_type_if_condition_test() {
        let l1 = Literal(3);
        let false_branch = Literal(-1);
        let true_branch = BinOp {
            l: Box::new(Literal(12)),
            op: Operator::Plus,
            r: Box::new(Literal(21)),
        };

        let cond = IfExpr {
            cond: Box::new(l1),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        };
        assert_eq!(cond.eval(), Err(Error::InvalidType));
    }

    #[test]
    fn wrong_type_operator_test() {
        let op = BinOp {
            l: Box::new(Boolean(true)),
            op: Operator::Plus,
            r: Box::new(Literal(21)),
        };
        assert_eq!(op.eval(), Err(Error::InvalidType));
    }
}
