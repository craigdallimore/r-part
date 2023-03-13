// https://github.com/rustwasm/wasm-bindgen/issues/2882
#![allow(non_snake_case,non_upper_case_globals)]

mod emitter;
mod vector;
mod particle;
mod draw;
mod webgl;

use game_loop::game_loop;
use emitter::Emitter;
use vector::Vect2d;
use draw::draw_scene;
use web_sys::WebGl2RenderingContext;
use webgl::link_program;
use webgl::compile_shader;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::Serialize;

#[derive(Serialize)]
pub struct State {
  emitter: Emitter,
  dimensions: Vect2d
}

impl State {
  fn new(dimensions: Vect2d) -> State {
    let mut e = Emitter::new();

    e.position = Vect2d(100.0, 100.0);
    e.steering = Vect2d(0.0, 60.0);
    e.max_particles = 240;
    e.initial_force = Vect2d(-30.0, -30.0);
    e.initial_range = Vect2d(50.0, 10.0);
    e.initial_energy = 10.0;
    e.initial_energy_range = 3.0;

    State {
      emitter: e,
      dimensions
    }
  }

  fn update(&mut self, time: f64) {
    self.emitter.update(time, self.dimensions);
  }

  fn render(&self, ctx: &WebGl2RenderingContext) {
    draw_scene(ctx, self);
  }
}

fn get_context() -> Result<WebGl2RenderingContext, JsValue> {

  let document = web_sys::window().unwrap().document().unwrap();

  let canvas = document.get_element_by_id("canvas").unwrap();

  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

  let ctx = canvas
    .get_context("webgl2")?
    .unwrap()
    .dyn_into::<WebGl2RenderingContext>()?;

  let vert_shader = compile_shader(
    &ctx,
    WebGl2RenderingContext::VERTEX_SHADER,
    r##"#version 300 es

      in vec4 position;

      void main() {
        gl_Position = position;
      }

    "##,
  )?;

  let frag_shader = compile_shader(
    &ctx,
    WebGl2RenderingContext::FRAGMENT_SHADER,
    r##"#version 300 es

      precision highp float;
      out vec4 outColor;

      void main() {
        outColor = vec4(1, 1, 1, 1);
      }

    "##,
  )?;

  let program = link_program(
    &ctx,
    &vert_shader,
    &frag_shader,
  )?;

  ctx.use_program(Some(&program));

  // A buffer for positions
  let buffer = ctx.create_buffer().ok_or("Failed to create buffer")?;

  // Bind the buffer to a _bind point_; when we refer to
  // ARRAY_BUFFER we are referring to this buffer
  ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

  let vertices: [f32; 9] = [-0.7, -0.7, 0.0,  0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

  unsafe {

    let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

    // Put the data in the buffer. STATIC_DRAW is a hint to WebGL
    // for how to optimise access to the buffer (something like that).
    ctx.buffer_data_with_array_buffer_view(
      WebGl2RenderingContext::ARRAY_BUFFER,
      &positions_array_buf_view,
      WebGl2RenderingContext::STATIC_DRAW
    );

  }

  // Create attribute state and make it the current state
  let vao = ctx.create_vertex_array()
    .ok_or("Could not create vertex array object")?;

  ctx.bind_vertex_array(Some(&vao));

  // Locate the attribute we can use to supply data to the program
  // "look up where the vertex data needs to go"
  let position_attribute_location = ctx.get_attrib_location(&program, "position");

  // Turn on the attribute; vertexarray will get data from
  // the buffer associated with this attribute
  ctx.enable_vertex_attrib_array(position_attribute_location as u32);

  // Configure the attribute and bind the current ARRAY_BUFFER to the attribute.
  ctx.vertex_attrib_pointer_with_i32(
    position_attribute_location as u32,
    3,
    WebGl2RenderingContext::FLOAT,
    false,
    0,
    0,
  );

  //let vert_count = (vertices.len() / 3) as i32;
  //draw(&ctx, vert_count);

  Ok(ctx)
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {

  let dimensions = Vect2d(300.0, 400.0);
  let mut game = State::new(dimensions);
  let ctx = get_context()?;

  game.emitter.max_particles = 100;

  game_loop(game, 240, 0.1, |g| {
    g.game.update(g.last_frame_time());
  }, move |g| {
    g.game.render(&ctx);
  });

  Ok(())
}
