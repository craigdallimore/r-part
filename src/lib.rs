// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

use wasm_bindgen::prelude::*;
use rand::prelude::*;

struct Vec2 (f32, f32);

struct Particle {
  position: Vec2,
  velocity: Vec2,
  lastPosition: Vec2,
  energy: f32
}

impl Particle {
  fn new() -> Particle {
    Particle {
      position: Vec2 (0.0, 0.0),
      velocity: Vec2 (0.0, 0.0),
      lastPosition: Vec2 (0.0, 0.0),
      energy: -1.0
    }
  }
}

struct Emitter {
  position: Vec2,
  initialForce: Vec2,
  initialRange: Vec2,
  initialEnergy: f32,
  initialEnergyRange: f32,
  maxParticles: f32,
  particles: Vec<Particle>,
  steering: Vec2,
}

impl Emitter {
  fn new() -> Emitter {
    Emitter {
      position: Vec2 (0.0, 0.0),
      initialForce: Vec2 (0.0, 0.0),
      initialRange: Vec2 (0.0, 0.0),
      initialEnergy: 0.0,
      initialEnergyRange: 0.0,
      maxParticles: 0.0,
      particles: Vec::new(),
      steering: Vec2 (0.0, 0.0),
    }
  }

  fn resetParticle(self: Self, p: &mut Particle) -> () {

    let mut rng = rand::thread_rng();
    let x:f32 = self.initialForce.0 + rng.gen() + self.initialRange.0;
    let y:f32 = self.initialForce.1 + rng.gen() + self.initialRange.1;
    let energy = self.initialEnergyRange + run.gen() + self.initialEnergyRange;

    p.velocity.0 = x;
    p.velocity.1 = y;
    p.position.0 = self.position.0;
    p.position.1 = self.position.1;
    p.lastPosition.0 = self.position.0;
    p.lastPosition.1 = self.position.1;
    p.energy = energy;

  }




}













#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
  use web_sys::console;

  console::log_1(&"Running WASM :)".into());

  Ok(())
}
