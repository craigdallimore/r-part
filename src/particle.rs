use crate::vector::*;

#[derive(Debug, Clone)]
pub struct Particle {
  pub position: Vect2d,
  pub velocity: Vect2d,
  pub last_position: Vect2d,
  pub energy: f64
}

impl Particle {
  pub fn new() -> Particle {
    Particle {
      position: Vect2d (0.0, 0.0),
      velocity: Vect2d (0.0, 0.0),
      last_position: Vect2d (0.0, 0.0),
      energy: -1.0
    }
  }
}

