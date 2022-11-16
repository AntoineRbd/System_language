use crate::{
    error::Error,
    expr::{Binder, Expr},
    operator::Operator,
    r#type::Type,
};

pub struct Literal(pub i64);

impl Expr for Literal {
    fn eval_with_binder(&self, _binder: &mut Binder) -> Result<Type, Error> {
        Ok(Type::Literal(self.0))
    }
}
