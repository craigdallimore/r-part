use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  use web_sys::console;

  console::log_1(&"Running WASM".into());

  Ok(())
}
