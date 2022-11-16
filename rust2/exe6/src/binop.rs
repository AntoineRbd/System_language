use crate::{
    error::Error,
    expr::{Binder, Expr},
    operator::Operator,
    r#type::Type,
};

pub struct BinOp {
    pub l: Box<dyn Expr>,
    pub op: Operator,
    pub r: Box<dyn Expr>,
}

impl Expr for BinOp {
    fn eval_with_binder(&self, binder: &mut Binder) -> Result<Type, Error> {
        if let (Type::Literal(left_value), Type::Literal(right_value)) = (
            self.l.eval_with_binder(binder)?,
            self.r.eval_with_binder(binder)?,
        ) {
            Ok(Type::Literal(self.op.eval(left_value, right_value)?))
        } else {
            Err(Error::InvalidType)
        }
    }
}
