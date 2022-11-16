use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
    Or,
    And,
}

impl Operator {
    pub fn eval(&self, l1: i32, l2: i32) -> Result<i32, Error> {
        match self {
            Self::Plus => Ok(l1 + l2),
            Self::Minus => Ok(l1 - l2),
            Self::Multiply => Ok(l1 * l2),
            Self::Divide => {
                if l2 == 0 {
                    Err(Error::DivideByZero)
                } else {
                    Ok(l1 / l2)
                }
            }
            Self::Or => Ok(l1 | l2),
            Self::And => Ok(l1 & l2),
        }
    }
}

