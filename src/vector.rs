#[derive(Debug, Clone)]
pub struct Vec2 (pub f32, pub f32);

impl Vec2 {
  pub fn new() -> Vec2 {
    Vec2 (0.0, 0.0)
  }
}
