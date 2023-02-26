//! # firebase-js-rs
//!
//! Unofficial Wasm bindings for Firebase JS SDKs.
//!

mod bindings;
mod config;

pub use bindings::*;
pub use config::Config;
pub use wasm_bindgen::closure::Closure;
