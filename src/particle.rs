use crate::vector::*;

pub struct Particle {
  pub position: Vec2,
  pub velocity: Vec2,
  pub lastPosition: Vec2,
  pub energy: f32
}

impl Particle {
  pub fn new() -> Particle {
    Particle {
      position: Vec2 (0.0, 0.0),
      velocity: Vec2 (0.0, 0.0),
      lastPosition: Vec2 (0.0, 0.0),
      energy: -1.0
    }
  }
}

