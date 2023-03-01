// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod emitter;
mod vector;
mod particle;
mod draw;

use game_loop::game_loop;
use emitter::Emitter;
use vector::Vect2d;
use draw::draw_scene;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::Serialize;

#[derive(Serialize)]
pub struct State {
  emitter: Emitter,
  dimensions: Vect2d
}

impl State {
  fn new(dimensions: Vect2d) -> State {
    let mut e = Emitter::new();

    e.position = Vect2d(100.0, 100.0);
    e.steering = Vect2d(0.0, 60.0);
    e.max_particles = 240;
    e.initial_force = Vect2d(-30.0, -30.0);
    e.initial_range = Vect2d(50.0, 10.0);
    e.initial_energy = 10.0;
    e.initial_energy_range = 3.0;

    State {
      emitter: e,
      dimensions
    }
  }

  fn update(&mut self, time: f64) {
    self.emitter.update(time, self.dimensions);
  }

  fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {

    draw_scene(ctx, self);

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

  let dimensions = Vect2d(400.0, 400.0);
  let context = get_context();
  let mut game = State::new(dimensions);

  game.emitter.max_particles = 100;

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, move |g| {
    g.game.render(&context);
  });

  Ok(())
}
