use std::collections::HashMap;
use std::rc::Rc;

mod context;
mod expression;
mod keyword;
mod literal;
mod object;
mod parse;

pub use self::context::Context;
pub use self::expression::*;
pub use self::keyword::*;
pub use self::literal::*;
pub use self::object::*;

type Wrap<T> = Rc<T>;
fn wrap<T>(inner: T) -> Wrap<T> {
    Rc::new(inner)
}

#[derive(Debug)]
pub struct IdentIntern {
    idents: HashMap<String, usize>,
}

impl IdentIntern {
    pub fn new() -> Self {
        IdentIntern {
            idents: HashMap::new(),
        }
    }

    pub fn get_or_add<T: Into<String>>(&mut self, ident: T) -> Ident {
        let next = self.idents.len();
        let val = self.idents.entry(ident.into()).or_insert(next);
        Ident(*val)
    }

    pub fn get_name(&self, ident: Ident) -> Option<&str> {
        for (k, v) in &self.idents {
            if *v == ident.0 {
                return Some(k)
            }
        }
        None
    }
}

pub trait NativeModule {
    fn register(&mut self) -> NativeModuleDescription;
    fn import(&mut self, &mut Context) -> Object {
        Object::Nil
    }
}

pub struct NativeModuleDescription {
    name: String,
    funcs: Vec<NativeModuleFunction>,
}

impl NativeModuleDescription {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            funcs: Vec::new(),
        }
    }

    pub fn register_function<T: Into<String>>(&mut self, name: T, ptr: fn(&mut Context, Vec<Object>) -> Object) {
        self.funcs.push(NativeModuleFunction {
            name: name.into(),
            ptr
        });
    }
}

pub struct NativeModuleFunction {
    name: String,
    ptr: fn(&mut Context, Vec<Object>) -> Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ident(usize);
