use std::ffi::c_void;

use gl::types::{GLenum, GLint, GLsizei};

use crate::buffer::Buffer;

#[repr(u32)]
pub enum Mode {
    Points = gl::POINTS,
    Lines = gl::LINES,
    Triangles = gl::TRIANGLES,
    Quads = gl::QUADS,
}

pub fn draw_arrays(mode: Mode, first: GLint, count: GLsizei) {
    unsafe { gl::DrawArrays(mode as _, first, count) }
}

pub struct ElementParams {
    pub count: GLsizei,
    pub type_: GLenum,
    pub indices: *const c_void,
}

pub trait Elements {
    fn use_me(&self) -> ElementParams;
    fn draw(&self, mode: Mode) {
        let params = self.use_me();
        unsafe {
            gl::DrawElements(mode as _, params.count, params.type_, params.indices);
        }
    }
    fn draw_instanced(&self, mode: Mode, primcount: GLsizei) {
        let params = self.use_me();
        unsafe {
            gl::DrawElementsInstanced(
                mode as _,
                params.count,
                params.type_,
                params.indices,
                primcount,
            );
        }
    }
}

pub struct ElementsU16<'a> {
    pub indices: &'a [u16],
}

impl<'a> Elements for ElementsU16<'a> {
    fn use_me(&self) -> ElementParams {
        Buffer::default().bind(gl::ELEMENT_ARRAY_BUFFER);
        ElementParams {
            count: self.indices.len() as _,
            type_: gl::UNSIGNED_SHORT,
            indices: self.indices.as_ptr() as _,
        }
    }
}

pub struct ElementsU32<'a> {
    pub indices: &'a [u32],
}

impl<'a> Elements for ElementsU32<'a> {
    fn use_me(&self) -> ElementParams {
        Buffer::default().bind(gl::ELEMENT_ARRAY_BUFFER);
        ElementParams {
            count: self.indices.len() as _,
            type_: gl::UNSIGNED_INT,
            indices: self.indices.as_ptr() as _,
        }
    }
}

pub struct ElementsBufU16 {
    pub indices: Buffer,
    pub len: u32,
}

impl Elements for ElementsBufU16 {
    fn use_me(&self) -> ElementParams {
        self.indices.bind(gl::ELEMENT_ARRAY_BUFFER);
        ElementParams {
            count: self.len as _,
            type_: gl::UNSIGNED_SHORT,
            indices: 0 as _,
        }
    }
}

pub struct ElementsBufU32 {
    pub indices: Buffer,
    pub len: u32,
}

impl Elements for ElementsBufU32 {
    fn use_me(&self) -> ElementParams {
        self.indices.bind(gl::ELEMENT_ARRAY_BUFFER);
        ElementParams {
            count: self.len as _,
            type_: gl::UNSIGNED_INT,
            indices: 0 as _,
        }
    }
}

// pub struct Elements<T: AsRef<[U]>,U> {
//     indices: T
// }
