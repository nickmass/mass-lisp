use super::*;

#[derive(Debug, Clone)]
pub enum Literal {
    Boolean(bool),
    Nil,
    Number(Number),
    String(String),
    Exception(Exception),
}

impl From<bool> for Literal {
    fn from(other: bool) -> Self {
        Literal::Boolean(other)
    }
}

impl From<Number> for Literal {
    fn from(other: Number) -> Self {
        Literal::Number(other)
    }
}

impl From<String> for Literal {
    fn from(other: String) -> Self {
        Literal::String(other)
    }
}

impl From<Exception> for Literal {
    fn from(other: Exception) -> Self {
        Literal::Exception(other)
    }
}

impl ::std::fmt::Display for Literal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Literal::*;
        match *self {
            Boolean(val) => write!(f, "{}", val)?,
            Nil => write!(f, "nil")?,
            Number(val) => write!(f, "{}", val)?,
            String(ref val) => write!(f, "\"{}\"", val)?,
            Exception(ref _ex) => write!(f, "[exception]")?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl From<i64> for Number {
    fn from(other: i64) -> Self {
        Number::Integer(other)
    }
}

impl From<f64> for Number {
    fn from(other: f64) -> Self {
        Number::Float(other)
    }
}

impl ::std::fmt::Display for Number {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Number::Integer(value) => write!(f, "{}", value),
            Number::Float(value) => write!(f, "{}", value),
        }
    }
}

