use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Boolean(bool),
    Literal(i64),
}

impl Type {
    pub fn to_string(&self) -> String {
        match &self {
            Self::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
            Self::Literal(l) => l.to_string()
        }
    }
}
