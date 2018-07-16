#[cfg(not(target_arch = "wasm32"))]
mod core;
#[cfg(target_arch = "wasm32")]
pub mod core;

mod gfx;

#[cfg(not(target_arch = "wasm32"))]
mod math;
#[cfg(target_arch = "wasm32")]
pub mod math;

#[cfg(not(target_arch = "wasm32"))]
mod gfx_gl;
#[cfg(not(target_arch = "wasm32"))]
use self::gfx_gl as gfx_platform;

#[cfg(target_arch = "wasm32")]
pub mod gfx_web;
#[cfg(target_arch = "wasm32")]
pub use self::gfx_web as gfx_platform;

pub use self::core::Core;
pub use self::math::Math;
pub use self::gfx::Gfx;

use super::lang::*;
use self::core::*;
