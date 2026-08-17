// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_double, c_float, c_int, c_schar, c_short, c_uchar, c_uint, c_ushort, c_void};
//48
pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLbyte = c_schar;
pub type GLshort = c_short;
pub type GLint = c_int;
pub type GLsizei = c_int;
pub type GLubyte = c_uchar;
pub type GLushort = c_ushort;
pub type GLuint = c_uint;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;
pub type GLvoid = c_void;
//63
//68
//AccumOp
pub const GL_ACCUM: GLenum = 0x0100;
pub const GL_LOAD: GLenum = 0x0101;
pub const GL_RETURN: GLenum = 0x0102;
pub const GL_MULT: GLenum = 0x0103;
pub const GL_ADD: GLenum = 0x0104;
//AlphaFunction
pub const GL_NEVER: GLenum = 0x0200;
pub const GL_LESS: GLenum = 0x0201;
pub const GL_EQUAL: GLenum = 0x0202;
pub const GL_LEQUAL: GLenum = 0x0203;
pub const GL_GREATER: GLenum = 0x0204;
pub const GL_NOTEQUAL: GLenum = 0x0205;
pub const GL_GEQUAL: GLenum = 0x0206;
pub const GL_ALWAYS: GLenum = 0x0207;
// TODO: we're missing about 1500 lines of defines and methods
// until that time, you can use the excellent GL crate
// https://github.com/brendanzab/gl-rs
extern "system" {
    pub fn glAccum(
        op: GLenum,
        value: GLfloat,
    );
    pub fn glAlphaFunc(
        func: GLenum,
        reference: GLclampf,
    );
}
