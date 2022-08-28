pub type GLenum = u32;
pub type GLbitmask = u32;
pub type GLuint = u32;
pub type GLint = i32;
pub type GLsizei = i32;
// Note(kettle11): GLintptr should be an i64, but those can't be properly passed
// between Wasm and Javascript, so for now just use an i32.
pub type GLintptr = i32;

pub use constants::*;
mod constants {
    //! Values taken from the [WebGL Constants page](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants).
    //!
    //! All names here have the `GL_` prefix added.

    use super::{GLbitmask, GLenum};

    pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
    pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
    pub const GL_FLOAT: GLenum = 0x1406;
    pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
    pub const GL_STATIC_DRAW: GLenum = 0x88E4;
    pub const GL_TRIANGLES: GLenum = 0x0004;
    pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
    pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
    pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
    pub const GL_DEPTH_TEST: GLenum = 0x0B71;
    pub const GL_LEQUAL: GLenum = 0x0203;

    pub const GL_COLOR_BUFFER_BIT: GLbitmask = 0x00004000;
    pub const GL_DEPTH_BUFFER_BIT: GLbitmask = 0x00000100;
}
