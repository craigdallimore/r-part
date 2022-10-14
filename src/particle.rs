use crate::vector::*;

#[derive(Debug, Clone)]
pub struct Particle {
  pub position: Vec2,
  pub velocity: Vec2,
  pub last_position: Vec2,
  pub energy: f32
}

impl Particle {
  pub fn new() -> Particle {
    Particle {
      position: Vec2 (0.0, 0.0),
      velocity: Vec2 (0.0, 0.0),
      last_position: Vec2 (0.0, 0.0),
      energy: -1.0
    }
  }
}

