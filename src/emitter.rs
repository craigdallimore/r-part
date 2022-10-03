use rand::prelude::*;
use crate::particle::*;
use crate::vector::*;

fn is_outside_area(point: &Vec2, area: &Vec2) -> bool {
  return point.0 < 0.0 ||
    point.1 < 0.0 ||
    point.0 > area.0 ||
    point.0 > area.1;
}

pub struct Emitter {
  position: Vec2,
  initial_force: Vec2,
  initial_range: Vec2,
  initial_energy: f32,
  initial_energy_range: f32,
  max_particles: usize,
  particles: Vec<Particle>,
  steering: Vec2,
}

impl Emitter {
  fn new() -> Emitter {
    Emitter {
      position: Vec2::new(),
      initial_force: Vec2::new(),
      initial_range: Vec2::new(),
      initial_energy: 0.0,
      initial_energy_range: 0.0,
      max_particles: 0,
      particles: Vec::new(),
      steering: Vec2::new(),
    }
  }

  fn add_particle(self: &mut Self) -> () {
    let mut p = Particle::new();
    self.reset_particle(&mut p);
    self.particles.push(p);
  }

  fn reset_particle(self: &Self, p: &mut Particle) -> () {

    let mut rng = rand::thread_rng();
    let gen1: f32 = rng.gen();
    let gen2: f32 = rng.gen();
    let gen3: f32 = rng.gen();

    let x:f32 = self.initial_force.0 + gen1 * self.initial_range.0;
    let y:f32 = self.initial_force.1 + gen2 * self.initial_range.1;
    let energy = self.initial_energy_range + gen3 * self.initial_energy_range;

    p.velocity.0 = x;
    p.velocity.1 = y;
    p.position.0 = self.position.0;
    p.position.1 = self.position.1;
    p.lastPosition.0 = self.position.0;
    p.lastPosition.1 = self.position.1;
    p.energy = energy;

  }

  fn update_particle(self: &Self, p: &mut Particle, time: f32, dimensions: &Vec2) {
    if p.energy <= 0.0 || is_outside_area(&p.position, &dimensions) {
      self.reset_particle(p);
      return;
    }

    //vec.add(p.velocity, vec.multiply([...this.steering], time));

    p.lastPosition.0 = p.position.0;
    p.lastPosition.1 = p.position.1;

    // Mutate position
    //vec.add(p.position, vec.multiply([...p.velocity], time));

    // Mutate energy
    p.energy = p.energy - time;

  }

  fn update(self: &mut Self, time: f32, dimensions: Vec2) {
    if self.particles.len() < self.max_particles {
      // Currently adding one particle per update
      // Potential improvements:
      // - add particles at a variable rate
      // - reset off-canvas particles instead of adding new ones
      self.add_particle();
    }

    for i in 1..self.particles.len() {
      let &mut particle = &mut self.particles[i];
      self.update_particle(&mut particle, time, &dimensions);
    }

  }

}
