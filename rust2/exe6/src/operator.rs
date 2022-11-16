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
    pub fn from_char(c: &str) -> Result<Self, Error> {
        match c {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "/" => Ok(Self::Divide),
            "*" => Ok(Self::Multiply),
            "|" => Ok(Self::Or),
            "&" => Ok(Self::And),
            _ => Err(Error::UnknowOperator),
        }
    }

    pub fn eval(&self, l1: i64, l2: i64) -> Result<i64, Error> {
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
