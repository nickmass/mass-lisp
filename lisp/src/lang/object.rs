use super::*;

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Boolean(bool),
    Number(Number),
    String(Wrap<String>),
    List(Wrap<Vec<Object>>),
    Function(Wrap<Function>),
    NativeFunction(NativeFunction),
    NativeObject(Wrap<::std::any::Any>),
    Quote(Wrap<Quote>),
    Exception(Wrap<Exception>),
}

impl Object {
    pub fn create_native_object<T: ::std::any::Any + Clone + 'static>(obj: T) -> Object {
        Object::NativeObject(wrap(obj))
    }

    pub fn call(self, ctx: &mut Context, args: Vec<Object>) -> Object {
        match self {
            Object::Function(ref func) => {
                let args: Vec<_> = args.into_iter().map(|a| a.eval(ctx)).collect();

                ctx.create_scope();

                func.args.iter().zip(args).for_each(|(ident, value)| {
                    ctx.declare_ident(*ident, value)
                });

                let return_val = func.body.eval(ctx);
                ctx.drop_scope();

                return_val
            },
            Object::NativeFunction(ref func) => {
                let args: Vec<_> = args.into_iter().map(|a| a.eval(ctx)).collect();
                (func.ptr)(ctx, args)
            },
            Object::Quote(ref quote) => {
                let val = if quote.shelled {
                    quote.unshell().into()
                } else {
                    quote.inner.eval(ctx)
                };

                args
                    .into_iter()
                    .fold(val, |_acc, arg| arg.eval(ctx))
            },
            Object::Exception(ref excep) => {
                eprintln!("Exception:\n{}", excep.message);

                args
                    .into_iter()
                    .fold(Object::Nil, |_acc, arg| arg.eval(ctx))
            },
            _ => {
                args
                    .into_iter()
                    .fold(self.eval(ctx), |_acc, arg| arg.eval(ctx))
            }
        }
    }

    pub fn eval(self, ctx: &mut Context) -> Object {
        match self {
            Object::Quote(ref quote) => {
                if quote.shelled {
                    quote.unshell().into()
                } else {
                    quote.inner.eval(ctx)
                }
            },
            Object::Exception(ref excep) => {
                eprintln!("Exception:\n{}", excep.message);
                Object::Nil
            },
            _ => self,
        }
    }
}

impl<'a> From<&'a Literal> for Object {
    fn from(other: &'a Literal) -> Self {
        match *other {
            Literal::Nil => Object::Nil,
            Literal::Boolean(val) => Object::Boolean(val),
            Literal::Number(val) => Object::Number(val),
            Literal::String(ref val) => Object::String(wrap(val.clone())),
            Literal::Exception(ref val) => Object::Exception(wrap(val.clone())),
        }
    }
}

impl From<String> for Object {
    fn from(other: String) -> Self {
        Object::String(wrap(other))
    }
}

impl<'a> From<&'a str> for Object {
    fn from(other: &'a str) -> Self {
        Object::String(wrap(other.to_string()))
    }
}

impl From<Number> for Object {
    fn from(other: Number) -> Self {
        Object::Number(other)
    }
}

impl From<i64> for Object {
    fn from(other: i64) -> Self {
        Object::Number(other.into())
    }
}

impl From<f64> for Object {
    fn from(other: f64) -> Self {
        Object::Number(other.into())
    }
}

impl From<bool> for Object {
    fn from(other: bool) -> Self {
        Object::Boolean(other)
    }
}

impl<T: Into<Object>> From<Vec<T>> for Object {
    fn from(other: Vec<T>) -> Self {
        Object::List(wrap(
            other
                .into_iter()
                .map(|x| x.into())
                .collect()
        ))
    }
}

impl From<Exception> for Object {
    fn from(other: Exception) -> Self {
        Object::Exception(wrap(other))
    }
}

impl From<Quote> for Object {
    fn from(other: Quote) -> Self {
        Object::Quote(wrap(other))
    }
}

impl From<NativeFunction> for Object {
    fn from(other: NativeFunction) -> Self {
        Object::NativeFunction(other)
    }
}

impl ::std::fmt::Display for Object {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Object::*;
        match *self {
            Nil => write!(f, "nil"),
            Boolean(ref value) => write!(f, "{}", value),
            Number(ref value) => write!(f, "{}", value),
            String(ref value) => write!(f, "{}", value),
            List(ref value) => {
                let strings: Vec<_> = value.iter().map(|i| i.to_string()).collect();
                write!(f, "List({})", strings.join(" "))
            },
            Function(_) => write!(f, "[function]"),
            NativeFunction(_) => write!(f, "[native code]"),
            NativeObject(_) => write!(f, "[native object]"),
            Quote(ref quote) => write!(f, "Quote({})", quote.inner),
            Exception(_) => write!(f, "[exception]"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub args: Vec<Ident>,
    pub body: Wrap<Expression>,
}

#[derive(Clone)]
pub struct NativeFunction {
    pub ptr: fn(&mut Context, Vec<Object>) -> Object
}

impl ::std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "NativeFunction(native code)")
    }
}

#[derive(Debug, Clone)]
pub struct Quote {
    inner: Wrap<Token>,
    shelled: bool,
}

impl Quote {
    pub fn new(exp: Token) -> Self {
        Quote {
            inner: wrap(exp),
            shelled: true,
        }
    }

    pub fn unshell(&self) -> Self {
        Quote {
            inner: self.inner.clone(),
            shelled: false
        }
    }

    pub fn reshell(&self) -> Self {
        Quote {
            inner: self.inner.clone(),
            shelled: true
        }
    }
}

#[derive(Debug, Clone)]
pub struct Exception {
    message: Wrap<String>,
}

impl Exception {
    pub fn message<T: Into<String>>(message: T) -> Self {
        Exception {
            message: wrap(message.into())
        }
    }
}
