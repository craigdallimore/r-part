//use crate::{State, particle::Particle};
use crate::State;
use web_sys::WebGl2RenderingContext;

pub fn draw_scene(
  ctx: &web_sys::WebGl2RenderingContext,
  state: &State
) -> () {

  ctx.viewport(
    0,
    0,
    state.dimensions.0 as i32,
    state.dimensions.1 as i32
  );

  ctx.clear_color(0.0, 0.0, 0.0, 0.0);
  ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

  ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3)

}

