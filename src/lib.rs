// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod emitter;
mod vector;
mod particle;

use web_sys::HtmlElement;

//use crate::emitter::*;
//use crate::vector::*;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;


fn setup_clicker() {
  use web_sys::console; // import raw API bindings

  let mut clicks = 0;

  // Closure is a handle to both a closure in rust and a closure in JS


  let window = web_sys::window().expect("should have a window");
  let document = window.document().expect("should have a document");


  let a = Closure::<dyn FnMut(web_sys::EventTarget)>::new(move |e: web_sys::EventTarget| {
    clicks += 1;
    console::log_2(&clicks.into(), &e.into());
  });

  let b = Closure::<dyn FnMut(f64)>::new(move |t: f64| {
    console::log_2(&"tick".into(), &t.into());
  });

  window.request_animation_frame(b.as_ref().unchecked_ref()).expect("raf error");

  document.get_element_by_id("clicker")
    .expect("should have #clicker")
    .dyn_ref::<HtmlElement>() // runtime checked cast to HtmlElement
    .expect("#clicker should be an element")
    .set_onclick(Some(a.as_ref().unchecked_ref()));

    // set_onclick :: Option<&js_sys::Function>
    // .as_ref :: Closure<T> -> &JsValue
    // .unchecked_ref ~ casts to a reference

  a.forget(); // Prevent a from being invalidated when dropped.
  b.forget();

}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  use web_sys::console;

  console::log_1(&"Running WASM :)".into());

  setup_clicker();

  Ok(())
}

// -------------------------------------------------------

