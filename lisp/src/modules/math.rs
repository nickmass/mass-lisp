use super::*;

pub struct Math;

impl NativeModule for Math {
    fn register(&mut self) -> NativeModuleDescription {
        let mut desc = NativeModuleDescription::new("math");

        desc.register_function("rand", rand);
        desc.register_function("sqrt", sqrt);
        desc.register_function("sin", sin);
        desc.register_function("cos", cos);
        desc.register_function("tan", tan);
        desc.register_function("asin", asin);
        desc.register_function("acos", acos);
        desc.register_function("atan", atan);
        desc.register_function("atan2", atan2);
        desc.register_function("pi", pi);
        desc.register_function("pow", pow);

        desc
    }
}

#[cfg(target_arch = "wasm32")]
pub mod platform {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Math, js_name = random)]
        pub fn random() -> f32;
    }

    pub fn rand() -> f64 {
        random() as f64
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub fn rand() -> f64 {
        ::rand::random::<f64>()
    }
}

pub fn rand(_ctx: &mut Context, _args: Vec<Object>) -> Object {
    let r = platform::rand();
    r.into()
}

fn float_op<T: ::std::borrow::Borrow<Object>>(arg: Option<T>, op: fn(f64) -> f64) -> Object {
    let arg = arg.map(|arg| float_impl(arg.borrow()));

    match arg {
        Some(Number::Float(num)) => op(num).into(),
        _ => Object::Nil
    }
}

pub fn sqrt(_ctx: &mut Context, args: Vec<Object>) -> Object {
    float_op(args.get(0), f64::sqrt)
}

pub fn sin(_ctx: &mut Context, args: Vec<Object>) -> Object {
    float_op(args.get(0), f64::sin)
}

pub fn cos(_ctx: &mut Context, args: Vec<Object>) -> Object {
    float_op(args.get(0), f64::cos)
}

pub fn tan(_ctx: &mut Context, args: Vec<Object>) -> Object {
    float_op(args.get(0), f64::tan)
}

pub fn asin(_ctx: &mut Context, args: Vec<Object>) -> Object {
    float_op(args.get(0), f64::asin)
}

pub fn acos(_ctx: &mut Context, args: Vec<Object>) -> Object {
    float_op(args.get(0), f64::acos)
}

pub fn atan(_ctx: &mut Context, args: Vec<Object>) -> Object {
    float_op(args.get(0), f64::atan)
}

pub fn atan2(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let a = args.get(0).map(|arg| float_impl(arg));
    let b = args.get(1).map(|arg| float_impl(arg));

    match (a, b) {
        (Some(Number::Float(a)), Some(Number::Float(b))) => f64::atan2(a, b).into(),
        _ => Object::Nil
    }
}

pub fn pow(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let a = args.get(0).map(|arg| float_impl(arg));
    let b = args.get(1).map(|arg| float_impl(arg));

    match (a, b) {
        (Some(Number::Float(a)), Some(Number::Float(b))) => f64::powf(a, b).into(),
        _ => Object::Nil
    }
}

pub fn pi(_ctx: &mut Context, _args: Vec<Object>) -> Object {
    ::std::f64::consts::PI.into()
}
