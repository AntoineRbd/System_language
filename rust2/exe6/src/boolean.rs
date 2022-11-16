use crate::{
    error::Error,
    expr::{Binder, Expr},
    operator::Operator,
    r#type::Type,
};

pub struct Boolean(pub bool);

impl Expr for Boolean {
    fn eval_with_binder(&self, _binder: &mut Binder) -> Result<Type, Error> {
        Ok(Type::Boolean(self.0))
    }
}
