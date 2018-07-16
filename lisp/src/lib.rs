#![cfg_attr(target_arch = "wasm32", feature(proc_macro, wasm_custom_section, wasm_import_module))]

extern crate combine;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate glium;
#[cfg(not(target_arch = "wasm32"))]
extern crate rand;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;

mod lang;
#[cfg(not(target_arch = "wasm32"))]
mod modules;
#[cfg(target_arch = "wasm32")]
pub mod modules;

pub use self::lang::Context;
