#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate lisp;
pub use lisp::Context;

use std::cell::RefCell;

thread_local!(static CONTEXT: RefCell<Context> = RefCell::new(Context::new()));

#[wasm_bindgen]
pub fn eval(source: String) -> String {
    CONTEXT.with(|ctx| {
        ctx.borrow_mut().eval_module(source).to_string()
    })
}

#[wasm_bindgen]
pub fn resume() -> bool {
    CONTEXT.with(|ctx| {
        ctx.borrow_mut().resume()
    })
}

#[wasm_bindgen]
pub fn reset() {
    CONTEXT.with(|ctx| {
        ctx.borrow_mut().reset()
    })
}
