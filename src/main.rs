#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb_derive;

mod webgl_rendering_context;

use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, IHtmlElement, IParentNode, TypedArray};
use webgl_rendering_context::*;

type gl = WebGLRenderingContext;

const VERTEX_SOURCE: &'static str = include_str!("shaders/vertex.glsl");
const FRAGMENT_SOURCE: &'static str = include_str!("shaders/fragment.glsl");
const VERTICIES: [f32; 12] = [
    1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0, 1.0, 0.0,
];
const TEXTURE_COORDINATE: [f32; 8] = [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
const INDICIES: [u32; 6] = [0, 1, 3, 1, 2, 3];

// draws a rectangle on the screen and binds a texture
// to that rectangle
fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: gl = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    context.clear_color(1.0, 0.0, 0.0, 1.0);
    context.clear(gl::COLOR_BUFFER_BIT);

    // create the verticies buffer
    let verticies = TypedArray::<f32>::from(&VERTICIES[..]).buffer();
    let vertex_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&verticies), gl::STATIC_DRAW);

    // create the texture coordinate buffer
    let textures = TypedArray::<f32>::from(&TEXTURE_COORDINATE[..]).buffer();
    let texture_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&textures), gl::STATIC_DRAW);

    let indicies = TypedArray::<u32>::from(&INDICIES[..]).buffer();
    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indicies), gl::STATIC_DRAW);

    // compile the vertex shader
    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&vert_shader, VERTEX_SOURCE);
    context.compile_shader(&vert_shader);

    // check if the compilation was successful
    let compiled = context.get_shader_parameter(&vert_shader, gl::COMPILE_STATUS);

    if compiled == stdweb::Value::Bool(false) {
        let error = context.get_shader_info_log(&vert_shader);
        if let Some(e) = error {
            console!(log, e);
        }
    }

    // compile the fragment shader
    let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&frag_shader, FRAGMENT_SOURCE);
    context.compile_shader(&frag_shader);

    // check if the compilation was successful
    let compiled = context.get_shader_parameter(&frag_shader, gl::COMPILE_STATUS);

    if compiled == stdweb::Value::Bool(false) {
        let error = context.get_shader_info_log(&frag_shader);
        if let Some(e) = error {
            console!(log, e);
        }
    }

    // create the shader program and link the fragment and vertex shaders
    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vert_shader);
    context.attach_shader(&shader_program, &frag_shader);
    context.link_program(&shader_program);

    // associate the vertex position buffer to the position attribute in the vertex shader
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    let pos_attr = context.get_attrib_location(&shader_program, "aVertexPosition") as u32;
    context.vertex_attrib_pointer(pos_attr, 3, gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(pos_attr);

    // associate the texture position buffer to the texture coordinate attribute in the vertex shader
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_buffer));
    let tex_attr = context.get_attrib_location(&shader_program, "aTextureCoord") as u32;
    context.vertex_attrib_pointer(tex_attr, 2, gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(tex_attr);

    // create the texture object and add its parameters
    let texture = context.create_texture().unwrap();
    context.bind_texture(gl::TEXTURE_2D, Some(&texture));

    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

    // initialize pixels to be an array of zeros, this should create a black texture
    let pixels = [0u8; 144 * 160 * 4];

    context.bind_texture(gl::TEXTURE_2D, Some(&texture));

    // load the data onto our texture object
    context.tex_image2_d(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        160,
        144,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        Some(pixels.as_ref()),
    );

    context.generate_mipmap(gl::TEXTURE_2D);
    context.active_texture(gl::TEXTURE0);

    context.use_program(Some(&shader_program));

    // get the texture sampler from the fragment shader and upload the
    // texture object onto it
    let screen_uniform = context
        .get_uniform_location(&shader_program, "uSampler")
        .unwrap();
    context.uniform1i(Some(&screen_uniform), 0);

    context.draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0);
}
