use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::gl_types::{GLbitmask, GLenum, GLint, GLintptr, GLsizei};

#[wasm_bindgen]
extern "C" {
    pub fn alert(msg: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(msg: &str);
}

#[wasm_bindgen(module = "/www/imports.js")]
extern "C" {
    pub fn setupCanvas();
    pub fn clearToBlue();
    pub fn attachShader(program: &JsValue, shader: &JsValue);
    pub fn bindBuffer(target: GLenum, id: &JsValue);
    pub fn bufferDataF32(target: GLenum, data: &[f32], usage: GLenum);
    pub fn bufferDataU16(target: GLenum, data: &[u16], usage: GLenum);
    pub fn uniformMatrix4fv(attr: JsValue, transpose: bool, value: &[f32]);
    pub fn getClientWidth() -> u32;
    pub fn getClientHeight() -> u32;
    pub fn clear(mask: GLbitmask);
    pub fn clearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn clearDepth(d: f32);
    pub fn enable(flag: u32);
    pub fn depthFunc(func: u32);
    pub fn compileShader(program: &JsValue);
    pub fn createBuffer() -> JsValue;
    pub fn createProgram() -> JsValue;
    pub fn createShader(shader_type: GLenum) -> JsValue;
    pub fn drawElements(mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr);
    pub fn drawArrays(mode: GLenum, offset: GLint, count: GLsizei);
    pub fn enableVertexAttribArray(index: GLint);
    pub fn getAttribLocation(program: &JsValue, name: &str) -> GLint;
    pub fn getUniformLocation(program: &JsValue, name: &str) -> JsValue;
    pub fn linkProgram(program: &JsValue);
    pub fn shaderSource(shader: &JsValue, source: &str);
    pub fn useProgram(program: &JsValue);
    pub fn vertexAttribPointer(
        index: GLint,
        size: GLint,
        type_: GLenum,
        normalized: bool,
        stride: GLsizei,
        pointer: GLintptr,
    );
}

#[macro_export]
macro_rules! alert {
    ($($args:expr),*) => {
        $crate::js::alert(&format!($($args),*))
    };
}

#[macro_export]
macro_rules! log {
    ($($args:expr),*) => {
        $crate::js::log(&format!($($args),*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($args:expr),*) => {
        $crate::js::error(&format!($($args),*))
    };
}
