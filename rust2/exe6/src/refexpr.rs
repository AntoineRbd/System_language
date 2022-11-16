use crate::{
    error::Error,
    expr::{Binder, Expr},
    operator::Operator,
    r#type::Type,
};

pub struct RefExpr {
    pub name: String,
}

impl Expr for RefExpr {
    fn eval_with_binder(&self, binder: &mut Binder) -> Result<Type, Error> {
        if !binder.contains_key(&self.name) {
            Err(Error::UnknowVariable)
        } else {
            Ok(binder[&self.name])
        }
    }
}
