// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod emitter;
mod vector;
mod particle;

use crate::emitter::*;
use crate::vector::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn onTick(x:f64) {
  use web_sys::console;

  let t:JsValue = x.into();
  console::log_2(&"tick {}".into(), &t);

}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  use web_sys::console;

  console::log_1(&"Running WASM :)".into());

  let mut e = Emitter::new();
  e.update(0.0, Vect2d::new());

  Ok(())
}
