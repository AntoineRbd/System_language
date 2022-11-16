use crate::{
    error::Error,
    expr::{Binder, Expr},
    operator::Operator,
    r#type::Type,
};

pub struct LetExpr {
    pub name: String,
    pub value: Box<dyn Expr>,
    pub next: Box<dyn Expr>,
}

impl Expr for LetExpr {
    fn eval_with_binder(&self, binder: &mut Binder) -> Result<Type, Error> {
        let value = self.value.eval_with_binder(binder)?;
        binder.insert(self.name.clone(), value);
        let res = self.next.eval_with_binder(binder)?;
        binder.remove(&self.name);
        Ok(res)
    }
}
