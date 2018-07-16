use super::*;

#[derive(Debug, Clone)]
pub enum Keyword {
    Func,
    If,
    Import,
    Lambda,
    Let,
    Set,
    Loop,
    Quote,
}

impl Keyword {
    pub fn call(&self, ctx: &mut Context, args: Vec<&Token>) -> Object {
        use self::Keyword::*;
        match *self {
            If => {
                let mut args = args.into_iter();
                match (args.next(), args.next(), args.next()) {
                    (cond, yes, no) => {
                        let cond = cond.map(|c| c.eval(ctx)).unwrap_or(Object::Nil);
                        match cond {
                            Object::Nil | Object::Boolean(false) => no.map(|n| n.eval(ctx)),
                            _ => yes.map(|y| y.eval(ctx)),
                        }
                    }
                }.unwrap_or(Object::Nil)
            },
            Import => {
                let mut args = args.into_iter();
                if let Some(path) = args.next() {
                    match path.eval(ctx) {
                        Object::String(s) => ctx.import_module(s.to_string()),
                        _ => Object::Nil,
                    }
                } else {
                    Object::Nil
                }
            },
            Func => {
                let mut args = args.into_iter();
                match (args.next(), args.next(), args.next()) {
                    (Some(Token::Ident(ident)), Some(Token::Expression(args)), Some(Token::Expression(body))) => {
                        let args = args.tokens.
                            iter()
                            .filter_map(|t| match *t {
                                Token::Ident(ident) => Some(ident),
                                _ => None,
                            })
                            .collect();

                        let fun = Function {
                            args,
                            body: wrap(body.clone())
                        };

                        ctx.declare_ident(*ident, Object::Function(wrap(fun)));
                        ctx.resolve_ident(*ident)
                    },
                    (Some(Token::Ident(ident)), Some(Token::Expression(body)), None) => {
                        let fun = Function {
                            args: Vec::new(),
                            body: wrap(body.clone())
                        };

                        ctx.declare_ident(*ident, Object::Function(wrap(fun)));
                        ctx.resolve_ident(*ident)
                    },
                    _ => Exception::message("invalid func").into()
                }
            },
            Lambda => {
                let mut args = args.into_iter();
                match (args.next(), args.next()) {
                    (Some(Token::Expression(args)), Some(Token::Expression(body))) => {
                        let args = args.tokens.
                            iter()
                            .filter_map(|t| match *t {
                                Token::Ident(ident) => Some(ident),
                                _ => None,
                            })
                            .collect();

                        let fun = Function {
                            args,
                            body: wrap(body.clone())
                        };

                        Object::Function(wrap(fun))
                    },
                    (Some(Token::Expression(body)), None) => {
                        let fun = Function {
                            args: Vec::new(),
                            body: wrap(body.clone())
                        };

                        Object::Function(wrap(fun))
                    },
                    _ => Exception::message("invalid lamda").into()
                }
            },
            Let => {
                let mut args = args.into_iter();
                if let Some(Token::Ident(ident)) = args.next() {
                    let value = args.next().map(|v| v.eval(ctx)).unwrap_or(Object::Nil);
                    ctx.declare_ident(*ident, value);
                    ctx.resolve_ident(*ident)
                } else {
                    Exception::message("invalid let").into()
                }
            },
            Set => {
                let mut args = args.into_iter();
                if let Some(Token::Ident(ident)) = args.next() {
                    let value = args.next().map(|v| v.eval(ctx)).unwrap_or(Object::Nil);
                    ctx.assign_ident(*ident, value);
                    ctx.resolve_ident(*ident)
                } else {
                    Exception::message("invalid let").into()
                }
            },
            Loop => {
                let mut args = args.into_iter();
                match (args.next(), args.next()) {
                    (Some(cond), Some(body)) => {
                        loop {
                            match cond.eval(ctx) {
                                Object::Nil | Object::Boolean(false) => break,
                                _ => (),
                            }
                            body.eval(ctx);
                        }
                    },
                    (Some(body), None) => {
                        loop {
                            body.eval(ctx);
                        }
                    },
                    _ => ()
                }
                Object::Nil
            },
            Quote => {
                let mut args = args.into_iter();
                match args.next() {
                    Some(t) => self::Quote::new(t.clone()).into(),
                    None => Object::Nil,
                }
            }
        }
    }

    pub fn eval(&self, _ctx: &mut Context) -> Object {
        Object::Nil
    }
}

impl ::std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Keyword::*;
        let s = match *self {
            Func => "func",
            If => "if",
            Import => "import",
            Lambda => "lambda",
            Let => "let",
            Set => "set",
            Loop => "loop",
            Quote => "quote",
        };

        write!(f, "{}", s)
    }
}
