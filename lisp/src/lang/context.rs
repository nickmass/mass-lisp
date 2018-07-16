use std::collections::HashMap;

use super::*;
use super::super::modules;

pub struct Context {
    interner: IdentIntern,
    scopes: Vec<HashMap<Ident, Object>>,
    native_modules: HashMap<String, (Box<NativeModule>, NativeModuleDescription)>,
    modules: HashMap<String, Expression>,
    yield_point: Option<(Object, Vec<Object>)>,
}

impl Context {
    pub fn new() -> Self {
        let interner = IdentIntern::new();
        let scopes = vec![HashMap::new()];

        let mut ctx = Context {
            interner,
            scopes,
            native_modules: HashMap::new(),
            modules: HashMap::new(),
            yield_point: None,
        };

        ctx.import_defaults();

        ctx
    }

    fn import_defaults(&mut self)  {
        self.register_native_module(modules::Core);
        self.register_native_module(modules::Math);
        self.register_native_module(modules::Gfx);

        self.import_module("core");
    }

    pub fn get_scopes(&self) -> &[HashMap<Ident, Object>] {
        self.scopes.as_slice()
    }

    pub fn register_native_module<T: NativeModule + 'static>(&mut self, mut module: T) {
        let desc = module.register();
        self.native_modules.insert(desc.name.clone(), (Box::new(module), desc));
    }

    pub fn import_native_function<T: Into<String>>(&mut self, name: T, ptr: fn(&mut Context, Vec<Object>) -> Object) {
        let ident = self.get_or_add_ident(name);
        self.declare_ident(ident, Object::NativeFunction(NativeFunction{ ptr }));
    }

    pub fn import_module<T: AsRef<str>>(&mut self, path: T) -> Object {
        let path = path.as_ref();
        if let Some((mut module, desc)) = self.native_modules.remove(path) { //Gross gross gross
            for func in &desc.funcs {
                self.import_native_function(func.name.clone(), func.ptr);
            }
            let res = module.import(self);
            self.native_modules.insert(desc.name.clone(), (module, desc));
            res
        } else {
            let mut module: Option<Expression> = None;

            if let Some(found_module) = self.modules.get(path) {
                module = Some(found_module.clone());
            }

            module.unwrap_or_else(|| {
                use std::fs::File;
                File::open(path)
                    .and_then(|f| Ok(self.read_module(path, f)))
                    .unwrap_or_else(|e| Exception::message(e.to_string()).into())
            }).eval(self)
        }
    }

    pub fn eval_module<T: AsRef<str>>(&mut self, source: T) -> Object {
        use std::io::Cursor;
        let module = Cursor::new(source.as_ref());

        let id = self.modules.len();
        self.read_module(format!("<anonymous:{}>", id), module).eval(self)
    }

    fn read_module<S: Into<String>, T: ::std::io::Read>(&mut self, name: S, module: T) -> Expression {
        let name = name.into();
        parse::Module::load(&mut self.interner, module)
            .map(|exp| {
                self.modules.insert(name.clone(), exp.clone());
                exp
            })
            .unwrap_or_else(|err| err.into_exception(name).into())
    }

    pub fn create_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn drop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn get_ident_name(&self, ident: Ident) -> Option<&str> {
        self.interner.get_name(ident)
    }

    pub fn get_or_add_ident<T:  Into<String>>(&mut self, ident: T) -> Ident {
        self.interner.get_or_add(ident)
    }

    pub fn declare_ident(&mut self, ident: Ident, value: Object) {
        if let Some(top) = self.scopes.last_mut() {
            top.insert(ident, value);
        }
    }

    pub fn assign_ident(&mut self, ident: Ident, value: Object) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(val) = scope.get_mut(&ident) {
                *val = value;
                return;
            }
        }
    }

    pub fn resolve_ident(&self, ident: Ident) -> Object {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(&ident) {
                return val.clone();
            }
        }

        Object::Nil
    }

    pub fn resolve_literal(&self, lit: &Literal) -> Object {
        lit.into()
    }

    pub fn do_yield(&mut self, callee: Object, args: Vec<Object>) {
        self.yield_point = Some((callee, args));
    }

    pub fn resume(&mut self) -> bool {
        if let Some((func, args)) = self.yield_point.take() {
            func.call(self, args);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.interner = IdentIntern::new();
        self.scopes = vec![HashMap::new()];
        self.native_modules = HashMap::new();
        self.modules = HashMap::new();
        self.yield_point = None;

        self.import_defaults();
    }
}
