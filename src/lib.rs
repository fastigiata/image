#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod adaptor;
mod core;
mod loader;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

