#![feature(local_key_cell_methods)]
use std::{cell::Cell, f32::consts::PI};

use cgmath::{Matrix4, PerspectiveFov, Rad, Vector3};
use gl_types::GLenum;
use wasm_bindgen::prelude::*;

use crate::{
    gl_types::{
        GLint, GLintptr, GLsizei, GL_ARRAY_BUFFER, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT,
        GL_DEPTH_TEST, GL_FLOAT, GL_FRAGMENT_SHADER, GL_LEQUAL, GL_STATIC_DRAW, GL_TRIANGLE_STRIP,
        GL_VERTEX_SHADER,
    },
    js::{clearToBlue, setupCanvas},
    util::Mat4ToSliceExt,
};

pub mod canvas;
pub mod gl_types;
pub mod js;
pub mod util;
pub mod voronoi_2d;

thread_local! {
    static SHADER_PROGRAM: Cell<JsValue> = Cell::new(build_program());
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    log!("Inside start");
    setupCanvas();
    log!("Clearing to blue");
    clearToBlue();

    draw(0.0);
}

fn build_program() -> JsValue {
    let vertex_shader_text = include_str!("shader.vert");
    let vertex_shader = js::createShader(GL_VERTEX_SHADER);
    js::shaderSource(&vertex_shader, vertex_shader_text);
    js::compileShader(&vertex_shader);

    let fragment_shader_text = include_str!("shader.frag");
    let fragment_shader = js::createShader(GL_FRAGMENT_SHADER);
    js::shaderSource(&fragment_shader, fragment_shader_text);
    js::compileShader(&fragment_shader);

    let shader_program = js::createProgram();
    js::attachShader(&shader_program, &vertex_shader);
    js::attachShader(&shader_program, &fragment_shader);
    js::linkProgram(&shader_program);
    shader_program
}

fn draw(time: f32) {
    // ############
    let shader_program = SHADER_PROGRAM.take();
    js::useProgram(&shader_program);
    let vertex_position = js::getAttribLocation(&shader_program, "aVertexPosition");
    let vertex_color = js::getAttribLocation(&shader_program, "aVertexColor");
    let projection_matrix_u = js::getUniformLocation(&shader_program, "uProjectionMatrix");
    let model_view_matrix_u = js::getUniformLocation(&shader_program, "uModelViewMatrix");

    let position_buffer = js::createBuffer();
    js::bindBuffer(GL_ARRAY_BUFFER, &position_buffer);
    const POSITIONS: [f32; 8] = [1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0];
    js::bufferDataF32(GL_ARRAY_BUFFER, &POSITIONS, GL_STATIC_DRAW);

    let color_buffer = js::createBuffer();
    js::bindBuffer(GL_ARRAY_BUFFER, &color_buffer);
    const COLORS: [f32; 16] = [
        1.0, 1.0, 1.0, 1.0, // white
        1.0, 0.0, 0.0, 1.0, // red
        0.0, 1.0, 0.0, 1.0, // green
        0.0, 0.0, 1.0, 1.0, // blue
    ];
    js::bufferDataF32(GL_ARRAY_BUFFER, &COLORS, GL_STATIC_DRAW);

    js::clearColor(0.0, 0.0, 0.0, 1.0); // Clear to black, fully opaque
    js::clearDepth(1.0); // Clear everything
    js::enable(GL_DEPTH_TEST); // Enable depth testing
    js::depthFunc(GL_LEQUAL); // Near things obscure far things

    js::clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    let fovy = Rad(45. * PI / 180.); // in radians
    let aspect = js::getClientWidth() as f32 / js::getClientHeight() as f32;
    let near = 0.1;
    let far = 100.0;
    let projection_matrix: Matrix4<_> = PerspectiveFov {
        fovy,
        aspect,
        near,
        far,
    }
    .into();
    let model_view_matrix = Matrix4::from_translation(Vector3::new(-0.0f32, 0.0, -6.0))
        * Matrix4::from_angle_y(Rad(time));

    // Tell WebGL how to pull out the positions from the position
    // buffer into the vertexPosition attribute
    {
        const NUM_COMPONENTS: GLint = 2;
        const TYPE: GLenum = GL_FLOAT;
        const NORMALIZE: bool = false;
        const STRIDE: GLsizei = 0;
        const OFFSET: GLintptr = 0;
        js::bindBuffer(GL_ARRAY_BUFFER, &position_buffer);
        js::vertexAttribPointer(
            vertex_position,
            NUM_COMPONENTS,
            TYPE,
            NORMALIZE,
            STRIDE,
            OFFSET,
        );
        js::enableVertexAttribArray(vertex_position);
    }

    // Tell WebGL how to pull out the colors from the color buffer
    // into the vertexColor attribute.
    {
        const NUM_COMPONENTS: GLint = 4;
        const TYPE: GLenum = GL_FLOAT;
        const NORMALIZE: bool = false;
        const STRIDE: GLsizei = 0;
        const OFFSET: GLintptr = 0;
        js::bindBuffer(GL_ARRAY_BUFFER, &color_buffer);
        js::vertexAttribPointer(
            vertex_color,
            NUM_COMPONENTS,
            TYPE,
            NORMALIZE,
            STRIDE,
            OFFSET,
        );
        js::enableVertexAttribArray(vertex_color);
    }

    js::useProgram(&shader_program);

    js::uniformMatrix4fv(projection_matrix_u, false, projection_matrix.as_slice());
    js::uniformMatrix4fv(model_view_matrix_u, false, model_view_matrix.as_slice());

    {
        const OFFSET: GLint = 0;
        const VERTEX_COUNT: GLsizei = 4;
        js::drawArrays(GL_TRIANGLE_STRIP, OFFSET, VERTEX_COUNT);
    }
    SHADER_PROGRAM.set(shader_program);

    // ############
}

#[wasm_bindgen]
pub fn redraw(width: u32, height: u32, time: f32) {
    draw(time);
}
