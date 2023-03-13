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

  let count = state.emitter.particles.len();

  // TODO can we get rid ofthis clone?
  let iter = state.emitter.particles.clone().into_iter();

  let arr = iter.fold(Vec::new(), |mut acc, p| {
    acc.push(p.position.0);
    acc.push(p.position.1);
    acc
  });
  let vertices: &[f64] = &arr[..];

  unsafe {

    let positions_array_buf_view = js_sys::Float64Array::view(vertices);

    ctx.buffer_data_with_array_buffer_view(
      WebGl2RenderingContext::ARRAY_BUFFER,
      &positions_array_buf_view,
      WebGl2RenderingContext::STATIC_DRAW
    );
  }

  ctx.draw_arrays(WebGl2RenderingContext::LINES, 0, count as i32);

}

