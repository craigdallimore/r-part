use crate::{State, particle::Particle};
use web_sys::{WebGl2RenderingContext};

pub fn draw_scene(
  ctx: &web_sys::WebGl2RenderingContext,
  state: &State
) -> () {

  /*
  ctx.clear(
    0.0,
    0.0,
    state.dimensions.0,
    state.dimensions.1
  );
  */

  for p in state.emitter.particles.iter() {
    draw_particle(ctx, &p);
  }

}

fn draw_particle(
  ctx: &web_sys::WebGl2RenderingContext,
  p: &Particle
) -> () {

  /*
  ctx.save();
  ctx.set_stroke_style(&"hsla(10, 10, 10, 1)".into());
  ctx.begin_path();
  ctx.move_to(p.last_position.0, p.last_position.1);
  ctx.line_to(p.position.0, p.position.1);
  ctx.stroke();
  ctx.close_path();
  ctx.restore();
  */

}
