// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  use web_sys::console;

  console::log_1(&"Running WASM :)".into());

  Ok(())
}
