// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod emitter;
mod vector;
mod particle;

use emitter::Emitter;
use js_sys::Function;
use vector::Vect2d;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct State {
  emitter: Emitter
}

impl State {
  fn new() -> State {
    State {
      emitter: Emitter::new()
    }
  }
}

fn update_state(state: &mut State, tick: f64) -> &State {
  state.emitter.update(tick, Vect2d::new());
  state
}

fn on_tick(tick: f64) {
  use web_sys::console;

  let mut initial_state = State::new();


  let state = update_state(&mut initial_state, tick);
  let serialized = serde_json::to_string(&state).unwrap();
  console::log_2(&"state".into(), &serialized.into());
}

fn init() {

  let window = web_sys::window().expect("should have a window");

  let x:Rc<RefCell<Option<Closure<dyn FnMut(f64) -> ()>>>> = Rc::new(RefCell::new(None));
  let y = x.clone();

  let mut lastTick:f64 = 0.0;

  *y.borrow_mut() = Some(Closure::<dyn FnMut(f64)>::new(move |time: f64| {

    let tick = time - lastTick;
    lastTick = time;
    on_tick(tick);

    let xbinding = x.borrow();
    let xclo: &Closure<dyn FnMut(f64) -> ()> = xbinding.as_ref().unwrap();
    let xjsval: &JsValue = xclo.as_ref(); // as_ref converts a type to shared reference of (usually inferred) input type
    let xjsfuncRef: &Function = xjsval.unchecked_ref(); // unchecked_ref casts to a reference to the specified type

    let xwindow = web_sys::window().expect("should have a window");
    xwindow.request_animation_frame(xjsfuncRef).expect("raf error");

  }));

  let binding = y.borrow();
  let clo: &Closure<dyn FnMut(f64) -> ()> = binding.as_ref().unwrap();
  let jsval: &JsValue = clo.as_ref(); // as_ref converts a type to shared reference of (usually inferred) input type
  let jsfuncRef: &Function = jsval.unchecked_ref(); // unchecked_ref casts to a reference to the specified type

  window.request_animation_frame(jsfuncRef).expect("raf error");

}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  use web_sys::console;

  console::log_1(&"Running WASM :)".into());

  init();

  Ok(())
}

// -------------------------------------------------------

