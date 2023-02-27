// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod emitter;
mod vector;
mod particle;

use game_loop::game_loop;
use emitter::Emitter;
use vector::Vect2d;

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

  fn update(&mut self, time: f64) {
    let dimensions = Vect2d(200.0, 200.0);
    self.emitter.update(time, dimensions);
  }

  fn render(&self) {

    use web_sys::console;

    let serialized = serde_json::to_string(&self.emitter.particles).unwrap();
    console::log_2(&"state".into(), &serialized.into());

  }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  use web_sys::console;

  console::log_1(&"Running WASM :)".into());
  let mut game = State::new();

  game.emitter.max_particles = 100;

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, |g| {
    g.game.render();
  });

  Ok(())
}
