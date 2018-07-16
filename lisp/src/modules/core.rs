use super::*;

pub struct Core;

impl NativeModule for Core {
    fn register(&mut self) -> NativeModuleDescription {
        let mut desc = NativeModuleDescription::new("core");
        desc.register_function("bool", _bool);
        desc.register_function("number", number);
        desc.register_function("float", float);
        desc.register_function("integer", integer);
        desc.register_function("char", char);
        desc.register_function("string", string);
        desc.register_function("trim", trim);
        desc.register_function("concat", concat);
        desc.register_function("print", print);
        desc.register_function("println", println);
        desc.register_function("read", read);
        desc.register_function("readln", readln);
        desc.register_function("eval", eval);
        desc.register_function("add", add);
        desc.register_function("sub", sub);
        desc.register_function("mul", mul);
        desc.register_function("div", div);
        desc.register_function("mod", rem);
        desc.register_function("list", list);
        desc.register_function("typeof", type_of);
        desc.register_function("not", not);
        desc.register_function("eq", eq);
        desc.register_function("gt", gt);
        desc.register_function("lt", lt);
        desc.register_function("and", and);
        desc.register_function("or", or);
        desc.register_function("map", map);
        desc.register_function("fill", fill);
        desc.register_function("cons", cons);
        desc.register_function("car", car);
        desc.register_function("cdr", cdr);
        desc.register_function("nth", nth);
        desc.register_function("len", len);
        desc.register_function("yield_loop", yield_loop);
        desc.register_function("debug_scopes", debug_scopes);

        desc
    }
}

#[cfg(target_arch = "wasm32")]
pub mod platform {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = massLispConsole, js_name = printLine)]
        pub fn print_line(line: String);

        #[wasm_bindgen(js_namespace = massLispConsole, js_name = print)]
        pub fn print(text: String);

        #[wasm_bindgen(js_namespace = massLispConsole, js_name = readLine)]
        pub fn read_line() -> String;
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub fn print_line(line: String) {
        println!("{}", line);
    }

    pub fn print(text: String) {
        use std::io::Write;
        print!("{}", text);
        let _ = ::std::io::stdout().flush();
    }

    pub fn read_line() -> String {
        let mut buf = String::new();
        let _ = ::std::io::stdin().read_line(&mut buf);

        buf
    }
}

pub fn number(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let num = match args.into_iter().next() {
        Some(obj) => number_impl(&obj),
        None => Number::Integer(0),
    };

    num.into()
}

pub fn number_impl(obj: &Object) -> Number {
    match obj {
        Object::Boolean(val) => if *val { Number::Integer(1) } else { Number::Integer(0) },
        Object::Number(num) => *num,
        Object::String(string) => {
            if let Ok(val) = string.parse::<i64>() {
                Number::Integer(val)
            } else if let Ok(val) = string.parse::<f64>(){
                Number::Float(val)
            } else {
                Number::Integer(0)
            }
        }
        _ => Number::Integer(0),
    }
}

pub fn float(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let num = match args.into_iter().next() {
        Some(obj) => float_impl(&obj),
        None => Number::Float(0.0),
    };

    Object::Number(num)
}

pub fn float_impl(obj: &Object) -> Number {
    match number_impl(obj) {
        Number::Integer(n) => Number::Float(n as f64),
        n @ _ => n,
    }
}

pub fn integer(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let num = match args.into_iter().next() {
        Some(obj) => integer_impl(&obj),
        None => Number::Integer(0),
    };

    num.into()
}

pub fn integer_impl(obj: &Object) -> Number {
    match number_impl(obj) {
        Number::Float(n) => Number::Integer(n as i64),
        n @ _ => n,
    }
}

pub fn string(_ctx: &mut Context, args: Vec<Object>) -> Object {
    if let Some(obj) = args.get(0) {
        string_impl(obj).into()
    } else {
        "".into()
    }
}

pub fn string_impl(obj: &Object) -> String {
    obj.to_string()
}

pub fn char(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    let c = args.next().map(|v| match integer_impl(&v) {
        Number::Integer(i) if i >= 0 && i <= 0x7f => i as u8 as char,
        _ => '\0',
    }).unwrap_or('\0');

    c.to_string().into()
}

pub fn trim(_ctx: &mut Context, args: Vec<Object>) -> Object {
    match args.into_iter().next() {
        Some(obj) => string_impl(&obj).trim().into(),
        None => "".into()
    }
}

pub fn concat(_ctx: &mut Context, args: Vec<Object>) -> Object {
    concat_impl(args).into()
}

pub fn concat_impl(args: Vec<Object>) -> String {
    let strings: Vec<_> = args
        .into_iter()
        .map(|s| match s {
            Object::List(list) => concat_impl(list.to_vec()),
            _ => string_impl(&s)
        })
        .collect();

    strings.concat()
}

pub fn print(_ctx: &mut Context, args: Vec<Object>) -> Object {
    platform::print(concat_impl(args));

    Object::Nil
}

pub fn println(_ctx: &mut Context, args: Vec<Object>) -> Object {
    platform::print_line(concat_impl(args));

    Object::Nil
}

pub fn read(_ctx: &mut Context, _args: Vec<Object>) -> Object {
    use std::io::Read;
    let mut buf = [0; 1];
    let _ = ::std::io::stdin().read_exact(&mut buf);

    (buf[0] as i64).into()
}

pub fn readln(_ctx: &mut Context, _args: Vec<Object>) -> Object {
    platform::read_line().into()
}

pub fn eval(ctx: &mut Context, args: Vec<Object>) -> Object {
    if let Some(obj) = args.get(0) {
        let source = string_impl(obj);
        ctx.eval_module(source)
    } else {
        Object::Nil
    }
}

pub fn add(_ctx: &mut Context, args: Vec<Object>) -> Object {
    use std::ops::*;
    math_op_impl(args, i64::add, f64::add).into()
}

pub fn sub(_ctx: &mut Context, args: Vec<Object>) -> Object {
    use std::ops::*;
    math_op_impl(args, i64::sub, f64::sub).into()
}

pub fn mul(_ctx: &mut Context, args: Vec<Object>) -> Object {
    use std::ops::*;
    math_op_impl(args, i64::mul, f64::mul).into()
}

pub fn div(_ctx: &mut Context, args: Vec<Object>) -> Object {
    use std::ops::*;
    math_op_impl(args, i64::div, f64::div).into()
}

pub fn rem(_ctx: &mut Context, args: Vec<Object>) -> Object {
    use std::ops::*;
    math_op_impl(args, i64::rem, f64::rem).into()
}

pub fn math_op_impl(args: Vec<Object>, op_i: fn(i64, i64) -> i64, op_f: fn(f64, f64) -> f64) -> Number {
    let mut args = args.into_iter()
            .map(|n| number_impl(&n));
    if let Some(first) = args.next() {
        let num = args
            .fold(first, |acc, next| {
                match acc {
                    Number::Integer(acc) => {
                        match next {
                            Number::Integer(n) => Number::Integer(op_i(acc, n)),
                            Number::Float(n) => Number::Float(op_f(acc as f64, n)),
                        }
                    },
                    Number::Float(acc) => {
                        match next {
                            Number::Integer(n) => Number::Float(op_f(acc, n as f64)),
                            Number::Float(n) => Number::Float(op_f(acc, n)),
                        }
                    },
                }
            });

        num
    } else {
        Number::Integer(0)
    }
}

pub fn list(_ctx: &mut Context, args: Vec<Object>) -> Object {
    args.into()
}

fn map_impl<F: Fn(&Object) -> Object>(obj: Object, func: F) -> Object {
    match obj {
        Object::List(list) => list.iter().map(|o| func(o)).collect::<Vec<_>>().into(),
        _ => func(&obj)
    }
}

fn reduce_impl<F: Fn(Object, &Object) -> Object>(obj: Object, init: Object, func: F) -> Object {
    match obj {
        Object::List(list) => list.iter().fold(init, |acc, o| func(acc, o)),
        _ => func(init, &obj)
    }
}

//Should do this properly with types instead of string cmping everything eventually
pub fn eq(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let args: Vec<_> = args
        .into_iter()
        .map(|o| string_impl(&o))
        .collect();

    let equal = args
        .windows(2)
        .all(|pair| pair[0] == pair[1]);

    equal.into()
}

pub fn gt(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    let left = args.next().unwrap_or(Object::Nil);
    let right = args.next().unwrap_or(Object::Nil);

    match (left, right) {
        (Object::Number(Number::Integer(l)),
         Object::Number(Number::Integer(r))) => (l > r).into(),
        (Object::Number(Number::Float(l)),
         Object::Number(Number::Float(r))) => (l > r).into(),
        (Object::Number(Number::Integer(l)),
         Object::Number(Number::Float(r))) => (l as f64 > r).into(),
        (Object::Number(Number::Float(l)),
         Object::Number(Number::Integer(r))) => (l > r as f64).into(),
        (Object::String(l), Object::String(r)) => (l > r).into(),
        (Object::Boolean(l), Object::Boolean(r)) => (l > r).into(),
        _ => Object::Nil
    }
}

pub fn lt(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    let left = args.next().unwrap_or(Object::Nil);
    let right = args.next().unwrap_or(Object::Nil);

    match (left, right) {
        (Object::Number(Number::Integer(l)),
         Object::Number(Number::Integer(r))) => (l < r).into(),
        (Object::Number(Number::Float(l)),
         Object::Number(Number::Float(r))) => (l < r).into(),
        (Object::Number(Number::Integer(l)),
         Object::Number(Number::Float(r))) => ((l as f64) < r).into(),
        (Object::Number(Number::Float(l)),
         Object::Number(Number::Integer(r))) => (l < r as f64).into(),
        (Object::String(l), Object::String(r)) => (l < r).into(),
        (Object::Boolean(l), Object::Boolean(r)) => (l < r).into(),
        _ => Object::Nil
    }
}

pub fn _bool(_ctx: &mut Context, args: Vec<Object>) -> Object {
    if let Some(val) = args.into_iter().next() {
        bool_impl(&val).into()
    } else {
        false.into()
    }
}

pub fn not(_ctx: &mut Context, args: Vec<Object>) -> Object {
    if let Some(val) = args.into_iter().next() {
        (!bool_impl(&val)).into()
    } else {
        true.into()
    }
}

pub fn and(_ctx: &mut Context, args: Vec<Object>) -> Object {
    args.into_iter()
        .map(|b| bool_impl(&b))
        .fold(true, |acc, next| acc && next)
        .into()
}

pub fn or(_ctx: &mut Context, args: Vec<Object>) -> Object {
    args.into_iter()
        .map(|b| bool_impl(&b))
        .fold(false, |acc, next| acc || next)
        .into()
}

pub fn bool_impl(obj: &Object) -> bool {
    match *obj {
        Object::Boolean(b) => b,
        Object::Number(Number::Integer(0)) => false,
        Object::Nil => false,
        _ => true
    }
}

pub fn map(ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    let target = args.next();
    let callee = args.next().unwrap_or(Object::Nil);

    let result = match target {
        Some(Object::String(s)) => s.chars().map(|c| {
            let s = c.to_string();
            callee.clone().call(ctx, vec![s.into()])
        }).collect(),
        Some(Object::List(l)) => l.iter().filter_map(|i| {
            match callee.clone().call(ctx, vec![i.clone()]) {
                Object::Nil => None,
                item @ _ => Some(item),
            }
        }).collect(),
        None | Some(Object::Nil) => vec![Object::Nil],
        Some(a) => { vec![callee.call(ctx, vec![a])] }
    };

    result.into()
}

pub fn fill(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    let count = integer_impl(&args.next().unwrap_or(Object::Nil));

    let count = if let Number::Integer(count) = count {
        if count < 0 {
            0
        } else {
            count as usize
        }
    } else {
        0
    };

    let value = args.next().unwrap_or(Object::Nil);

    vec![value; count].into()
}

pub fn cons(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    match (args.next(), args.next()) {
        (Some(o), Some(Object::List(l))) => {
            let mut new_list = l.to_vec();
            new_list.push(o);
            new_list.into()
        },
        (Some(Object::Nil), Some(Object::Nil)) => Object::Nil,
        (Some(a), Some(Object::Nil)) => vec![a].into(),
        (Some(Object::Nil), Some(a)) => vec![a].into(),
        (Some(a), Some(b)) => vec![a, b].into(),
        (Some(Object::Nil), None) => Object::Nil,
        (Some(a), None) => vec![a].into(),
        _ => Object::Nil
    }
}

pub fn car(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    match args.next() {
        Some(Object::List(l)) => l.get(0).map(|f| f.clone()).unwrap_or(Object::Nil),
        Some(o) => o,
        _ => Object::Nil
    }
}

pub fn cdr(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    match args.next() {
        Some(Object::List(l)) => {
            if l[1..].len() > 0 {
                l[1..].to_vec().into()
            } else {
                Object::Nil
            }
        },
        Some(o) => o,
        _ => Object::Nil
    }
}

pub fn len(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    match args.next() {
        Some(Object::List(l)) => (l.len() as i64).into(),
        Some(Object::String(s)) => (s.len() as i64).into(),
        _ => Object::Nil
    }
}

pub fn nth(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let mut args = args.into_iter();
    let collection = args.next().unwrap_or(Object::Nil);
    let index = integer_impl(&args.next().unwrap_or(Object::Nil));
    let value = args.next();

    let index = if let Number::Integer(index) = index {
        if index < 0 {
            0
        } else {
            index as usize
        }
    } else {
        0
    };

    match collection {
        Object::String(s) => {
            if let Some(val) = value {
                let mut chars: Vec<_> = s.chars().collect();
                chars.get_mut(index).and_then(|s| val.to_string().chars().next().map(|c| *s = c));
                chars.iter().collect::<String>().into()
            } else {
                s
                    .chars()
                    .nth(index)
                    .map(|c| c.to_string().into())
                    .unwrap_or(Object::Nil)
            }
        },
        Object::List(l) => {
            if let Some(val) = value {
                let mut l = l.to_vec();
                l.get_mut(index).map(|v| *v = val);
                l.into()
            } else {
                l
                    .get(index)
                    .map(|i| i.clone())
                    .unwrap_or(Object::Nil)
            }
        },
        _ => Object::Nil
    }
}

pub fn yield_loop(ctx: &mut Context, args: Vec<Object>) -> Object {
    let (condition, body) = if args.len() > 1 {
        (args.get(0), args.get(1))
    } else {
        (None, args.get(0))
    };

    let result = condition
        .map(|cond| bool_impl(&cond.clone().eval(ctx)))
        .unwrap_or(true);

    if result {
        body.map(|body| body.clone().eval(ctx) );

        let new_args: Vec<_> = args.iter().map(|arg| {
            match arg {
                Object::Quote(quote) => quote.reshell().into(),
                _ => arg.clone(),
            }
        }).collect();


        let callback = NativeFunction {
            ptr: yield_loop,
        };

        ctx.do_yield(callback.into(), new_args);
    }

    Object::Nil
}

pub fn type_of(_ctx: &mut Context, args: Vec<Object>) -> Object {
    let _type = args.get(0).map(|obj| {
        match *obj {
            Object::Nil => "nil",
            Object::Boolean(_) => "boolean",
            Object::Number(_) => "number",
            Object::String(_) => "string",
            Object::List(_) => "list",
            Object::Function(_) => "function",
            Object::NativeFunction(_) => "nativefunction",
            Object::NativeObject(_) => "nativeobject",
            Object::Quote(_) => "quote",
            Object::Exception(_) => "exception",
        }
    }).unwrap_or("nil");

    _type.into()
}

pub fn debug_scopes(ctx: &mut Context, _args: Vec<Object>) -> Object {
    for (idx, scope) in ctx.get_scopes().iter().enumerate() {
        platform::print_line(format!("{}:", idx));
        for (k, v) in scope {
            platform::print_line(format!("\t{}: {}", ctx.get_ident_name(*k).unwrap_or("<unknown>"), v));
        }
    }

    Object::Nil
}
