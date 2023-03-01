// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod emitter;
mod vector;
mod particle;

use game_loop::game_loop;
use emitter::Emitter;
use vector::Vect2d;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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

  fn update(&mut self, time: f64) {
    let dimensions = Vect2d(200.0, 200.0);
    self.emitter.update(time, dimensions);
  }

  fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {

    use web_sys::console;

    let serialized = serde_json::to_string(&self.emitter.particles).unwrap();
    console::log_2(&"state".into(), &serialized.into());

    ctx.begin_path();
    ctx.move_to(100.0, 0.0);
    ctx.line_to(100.0, 200.0);
    ctx.stroke();

  }
}

fn get_context() -> web_sys::CanvasRenderingContext2d {

  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();

  return canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();

}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

  let context = get_context();
  let mut game = State::new();

  game.emitter.max_particles = 100;

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, move |g| {
    g.game.render(&context);
  });

  Ok(())
}
