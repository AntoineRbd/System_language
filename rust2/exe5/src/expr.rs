use std::collections::HashMap;

use lexpr::{cons::ListIter, Number, Value};
use serde::{Deserialize, Serialize};

use crate::{error::Error, operator::Operator, r#type::Type};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    Literal(i64),
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

    fn parse_binop(operator: &Box<str>, mut values: &mut ListIter) -> Result<Expr, Error> {
        let operator = Operator::from_char(operator)?;
        let l = Expr::parser_dispatcher(&mut values)?;
        let r = Expr::parser_dispatcher(&mut values)?;
        Ok(Expr::BinOp {
            l: Box::new(l),
            op: operator,
            r: Box::new(r),
        })
    }

    fn parse_let_expr(mut values: &mut ListIter) -> Result<Expr, Error> {
        if let Some(Value::Symbol(variable_name)) = values.next() {
            let value = Expr::parser_dispatcher(&mut values)?;
            let next = Expr::parser_dispatcher(&mut values)?;
            Ok(Expr::LetExpr {
                name: (*variable_name).to_string(),
                value: Box::new(value),
                next: Box::new(next),
            })
        } else {
            Err(Error::Parsing)
        }
    }

    fn parse_ref_expr(value: &Box<str>) -> Result<Expr, Error> {
        Ok(Expr::RefExpr {
            name: value.to_string(),
        })
    }

    fn parse_if_expr(mut values: &mut ListIter) -> Result<Expr, Error> {
        let cond = Expr::parser_dispatcher(&mut values)?;
        let true_branch = Expr::parser_dispatcher(&mut values)?;
        let false_branch = Expr::parser_dispatcher(&mut values)?;

        Ok(Expr::IfExpr {
            cond: Box::new(cond),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        })
    }

    fn parse_literal(number: &Number, _values: &ListIter) -> Result<Expr, Error> {
        if let Some(number) = number.as_i64() {
            Ok(Expr::Literal(number))
        } else {
            Err(Error::Parsing)
        }
    }

    fn parser_dispatcher_symbol(value: &Box<str>, values: &mut ListIter) -> Result<Expr, Error> {
        match &**value {
            "-" | "+" | "/" | "*" | "|" | "&" => Expr::parse_binop(value, values),
            "let" => Expr::parse_let_expr(values),
            "if" => Expr::parse_if_expr(values),
            "true" => Ok(Expr::Boolean(true)),
            "false" => Ok(Expr::Boolean(false)),
            _ => Expr::parse_ref_expr(value),
        }
    }

    fn parser_dispatcher_list(values: &mut ListIter) -> Result<Expr, Error> {
        let res = Expr::parser_dispatcher(values);
        if values.is_empty() {
            res
        } else {
            Err(Error::Parsing)
        }
    }

    fn parser_dispatcher(values: &mut ListIter) -> Result<Expr, Error> {
        if let Some(value) = values.next() {
            println!("value: {value:?}");
            let res = match value {
                Value::Symbol(value) => Expr::parser_dispatcher_symbol(&value, values),
                Value::Number(value) => Ok(Expr::parse_literal(value, values)?),
                Value::Cons(value) => Expr::parser_dispatcher_list(&mut value.list_iter()),
                _ => Err(Error::Parsing),
            };
            return res;
        }
        Err(Error::InvalidType)
    }

    pub fn parse_lexpr(string: &str) -> Result<Expr, Error> {
        let parser = lexpr::parse::from_str(string).expect("Unable");

        if let Some(mut list_iter) = parser.list_iter() {
            Expr::parser_dispatcher_list(&mut list_iter)
        } else {
            Err(Error::Parsing)
        }
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
        // let a = 12; in a;
        // + a
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

    #[test]
    fn lexpr_binop_test() -> Result<(), Error> {
        let lexpr_string = "(+ 12 2)";
        let expr = Expr::parse_lexpr(lexpr_string)?;
        assert_eq!(expr.eval()?, Type::Literal(12 + 2));
        Ok(())
    }

    #[test]
    fn lexpr_let_ref_test() -> Result<(), Error> {
        let lexpr_string = "(let a 12 (+ a 12))";
        let expr = Expr::parse_lexpr(lexpr_string)?;
        assert_eq!(expr.eval()?, Type::Literal(12 + 12));
        Ok(())
    }

    #[test]
    fn lexpr_if_test() -> Result<(), Error> {
        let lexpr_string = "(if true 15 18)";
        let expr = Expr::parse_lexpr(lexpr_string)?;
        assert_eq!(expr.eval()?, Type::Literal(15));
        Ok(())
    }

    #[test]
    fn lexpr_if_false_test() -> Result<(), Error> {
        let lexpr_string = "(if false 15 (+ 1 1))";
        let expr = Expr::parse_lexpr(lexpr_string)?;
        assert_eq!(expr.eval()?, Type::Literal(2));
        Ok(())
    }

    #[test]
    fn lexpr_final_test() -> Result<(), Error> {
        let lexpr_string = "(let a 12 (+ a (if true 15 18)))";
        let expr = Expr::parse_lexpr(lexpr_string)?;
        assert_eq!(expr.eval()?, Type::Literal(12 + 15));
        Ok(())
    }

    #[test]
    fn lexpr_error_test() -> Result<(), Error> {
        let lexpr_string = "(+ 1 1 1)";
        let expr = Expr::parse_lexpr(lexpr_string);
        assert_eq!(expr, Err(Error::Parsing));

        let lexpr_string = "(let a 12 (+ a (if true 15 18 34)))";
        let expr = Expr::parse_lexpr(lexpr_string);
        assert_eq!(expr, Err(Error::Parsing));

        let lexpr_string = "(let a 12 (+ a 1 (if true 15 18)))";
        let expr = Expr::parse_lexpr(lexpr_string);
        assert_eq!(expr, Err(Error::Parsing));

        Ok(())
    }
}
