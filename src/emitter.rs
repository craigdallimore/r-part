use rand::prelude::*;
use crate::particle::*;
use crate::vector::*;

fn is_outside_area(point: &Vect2d, area: &Vect2d) -> bool {
  return point.0 < 0.0 ||
    point.1 < 0.0 ||
    point.0 > area.0 ||
    point.0 > area.1;
}

pub struct Emitter {
  position: Vect2d,
  initial_force: Vect2d,
  initial_range: Vect2d,
  initial_energy: f64,
  initial_energy_range: f64,
  max_particles: usize,
  particles: Vec<Particle>,
  steering: Vect2d,
}

impl Emitter {
  pub fn new() -> Emitter {
    Emitter {
      position: Vect2d::new(),
      initial_force: Vect2d::new(),
      initial_range: Vect2d::new(),
      initial_energy: 0.0,
      initial_energy_range: 0.0,
      max_particles: 0,
      particles: Vec::new(),
      steering: Vect2d::new(),
    }
  }

  fn add_particle(self: &mut Self) -> () {
    let mut p = Particle::new();
    self.reset_particle(&mut p);
    self.particles.push(p);
  }

  fn reset_particle<'a>(self: &Self, p: &'a mut Particle) -> &'a mut Particle {

    let mut rng = rand::thread_rng();
    let gen1: f64 = rng.gen();
    let gen2: f64 = rng.gen();
    let gen3: f64 = rng.gen();

    let x:f64 = self.initial_force.0 + gen1 * self.initial_range.0;
    let y:f64 = self.initial_force.1 + gen2 * self.initial_range.1;
    let energy = self.initial_energy + gen3 * self.initial_energy_range;

    p.velocity.0 = x;
    p.velocity.1 = y;
    p.position.0 = self.position.0;
    p.position.1 = self.position.1;
    p.last_position.0 = self.position.0;
    p.last_position.1 = self.position.1;
    p.energy = energy;
    p

  }

  fn update_particle(self: &Self, p: &Particle, time: f64, dimensions: &Vect2d) -> Particle {

    let mut p = p.clone();

    if p.energy <= 0.0 || is_outside_area(&p.position, &dimensions) {
      self.reset_particle(&mut p);
    }

    p.velocity = p.velocity + (self.steering * time);

    p.last_position.0 = p.position.0;
    p.last_position.1 = p.position.1;

    // Mutate position
    p.position = p.position + (p.velocity * time);

    // Mutate energy
    p.energy = p.energy - time;
    p

  }

  pub fn update(self: &mut Self, time: f64, dimensions: Vect2d) {
    if self.particles.len() < self.max_particles {
      // Currently adding one particle per update
      // Potential improvements:
      // - add particles at a variable rate
      // - reset off-canvas particles instead of adding new ones
      self.add_particle();
    }

    self.particles = self.particles
        .iter()
        .map(|p| self.update_particle(p, time, &dimensions))
        .collect();

  }

}
