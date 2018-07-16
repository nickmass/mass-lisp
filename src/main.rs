extern crate lisp;

use lisp::Context;

fn main() {
    let mut ctx = Context::new();

    let mut args = ::std::env::args();
    let _executable = args.next();
    if let Some(path) = args.next() {
        let _res = ctx.import_module(path);
    } else {
        let source = r#"
(loop(println(eval(readln))))
"#;
        let _res = ctx.eval_module(source);
    }
}
