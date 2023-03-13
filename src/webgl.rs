use wasm_bindgen::{JsCast, JsValue};
use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram};

// https://rustwasm.github.io/wasm-bindgen/examples/webgl.html
pub fn compile_shader(
  ctx: &WebGl2RenderingContext,
  shader_type: u32,
  source: &str
) -> Result<WebGlShader, String> {

  let shader = ctx
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  ctx.shader_source(&shader, source);
  ctx.compile_shader(&shader);

  let compile_status = ctx.get_shader_parameter(
    &shader, WebGl2RenderingContext::COMPILE_STATUS
  ).as_bool().unwrap_or(false);

  if compile_status {
    Ok(shader)
  } else {

    let msg = ctx
      .get_shader_info_log(&shader)
      .unwrap_or_else(|| String::from("Unknown error creating shader"));

    Err(msg)
  }

}

pub fn link_program(
  ctx: &WebGl2RenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {

  let program = ctx
    .create_program()
    .ok_or_else(|| String::from("Unable to create program"))?;

  ctx.attach_shader(&program, vert_shader);
  ctx.attach_shader(&program, frag_shader);
  ctx.link_program(&program);

  let link_status = ctx
    .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false);

  if link_status {
    Ok(program)
  } else {

    let msg = ctx
      .get_program_info_log(&program)
      .unwrap_or_else(|| String::from("Unknown error linking program"));

    Err(msg)

  }



}

pub fn get_context() -> Result<WebGl2RenderingContext, JsValue> {

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

  Ok(ctx)
}
