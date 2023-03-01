use crate::{State, particle::Particle};

pub fn draw_scene(
  ctx: &web_sys::CanvasRenderingContext2d,
  state: &State
) -> () {

  ctx.clear_rect(0.0, 0.0, state.dimensions.0, state.dimensions.1);
  ctx.set_stroke_style(&"hlsa(123, 50, 100, 1)".into());
  ctx.save();
  ctx.begin_path();
  ctx.move_to(100.0, 0.0);
  ctx.line_to(100.0, 100.0);
  ctx.close_path();
  ctx.stroke();
  ctx.restore();

  for p in state.emitter.particles.iter() {
    draw_particle(ctx, &p);
  }

}

fn draw_particle(
  ctx: &web_sys::CanvasRenderingContext2d,
  p: &Particle
) -> () {

  ctx.save();
  ctx.set_stroke_style(&"hsla(10, 10, 10, 1)".into());
  ctx.begin_path();
  ctx.move_to(p.last_position.0, p.last_position.1);
  ctx.line_to(p.position.0, p.position.1);
  ctx.stroke();
  ctx.close_path();
  ctx.restore();

}
