use super::*;

#[derive(Debug, Clone)]
pub struct Expression {
    pub tokens: Vec<Token>,
}

impl Expression {
    pub fn new() -> Self {
        Expression { tokens: vec![] }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn eval(&self, ctx: &mut Context) -> Object {
        let mut tokens = self.tokens.iter();

        if let Some(callee) = tokens.next() {
            match callee {
                Token::Keyword(ref lang_item) => {
                    lang_item.call(ctx, tokens.collect())
                },
                _ => {
                    let callee = callee.eval(ctx);
                    let args = tokens.map(|t| t.eval(ctx)).collect();

                    callee.call(ctx, args)
                }
            }
        } else {
            Object::Nil
        }
    }
}

impl From<Exception> for Expression {
    fn from(other: Exception) -> Expression {
        let lit: Literal = other.into();
        Expression {
            tokens: vec![lit.into()]
        }
    }
}

impl ::std::fmt::Display for Expression {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "(")?;
        let mut first = true;
        for t in &self.tokens {
            if first {
                write!(f, "{}", t)?;
            } else {
                write!(f, " {}", t)?;
            }
            first = false;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Expression(Expression),
    Ident(Ident),
    Keyword(Keyword),
    Literal(Literal),
}

impl Token {
    pub fn eval(&self, ctx: &mut Context) -> Object {
        match *self {
            Token::Ident(ident) => ctx.resolve_ident(ident),
            Token::Literal(ref lit) => ctx.resolve_literal(lit),
            Token::Keyword(ref keyword) => keyword.eval(ctx),
            Token::Expression(ref exp) => exp.eval(ctx),
        }
    }
}
impl From<Expression> for Token {
    fn from(other: Expression) -> Token {
        Token::Expression(other)
    }
}

impl From<Ident> for Token {
    fn from(other: Ident) -> Token {
        Token::Ident(other)
    }
}

impl From<Keyword> for Token {
    fn from(other: Keyword) -> Token {
        Token::Keyword(other)
    }
}

impl From<Literal> for Token {
    fn from(other: Literal) -> Token {
        Token::Literal(other)
    }
}

impl ::std::fmt::Display for Token {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Token::Ident(ident) => write!(f, "ident_{}", ident.0)?,
            Token::Literal(ref lit) => write!(f, "{}", lit)?,
            Token::Keyword(ref keyword) => write!(f, "{}", keyword)?,
            Token::Expression(ref exp) => write!(f, "{}", exp)?,
        }

        Ok(())
    }
}

