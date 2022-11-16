use crate::{
    error::Error,
    expr::{Binder, Expr},
    operator::Operator,
    r#type::Type,
};

pub struct IfExpr {
    pub cond: Box<dyn Expr>,
    pub true_branch: Box<dyn Expr>,
    pub false_branch: Box<dyn Expr>,
}

impl Expr for IfExpr {
    fn eval_with_binder(&self, binder: &mut Binder) -> Result<Type, Error> {
        if let Type::Boolean(cond) = self.cond.eval_with_binder(binder)? {
            if cond {
                self.true_branch.eval_with_binder(binder)
            } else {
                self.false_branch.eval_with_binder(binder)
            }
        } else {
            Err(Error::InvalidType)
        }
    }
}
