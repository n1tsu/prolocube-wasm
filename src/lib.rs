use euclid::*;
use wasm_bindgen::prelude::*;

extern crate web_sys;

mod cube;
mod utils;
mod render;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
