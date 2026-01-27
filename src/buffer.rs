use derive_more::{From, Into};
use gl::types::{GLenum, GLsizei, GLuint};

use crate::attribute::{Attribute, AttributeFormat};

/// To create and delete buffers:
/// ```rust
/// use vita_gl_helpers::buffer::GenDelBuffersExt;
/// let mut buffers = vec![Buffer::default();5];
/// buffers.gen_buffers();
/// // Do things with the buffers
/// buffers.del_buffers();
/// ```
#[derive(From, Into, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Buffer(GLuint);

impl Buffer {
    pub fn bind(&self, target: impl Into<GLenum>) {
        unsafe {
            gl::BindBuffer(target.into(), self.0);
        }
    }
    pub fn bind_then<R>(
        &self,
        target: impl Into<GLenum>,
        then: impl FnOnce(BoundBuffer) -> R,
    ) -> R {
        let target = target.into();
        self.bind(target);
        then(BoundBuffer(target))
    }
    pub fn data<T>(
        &self,
        target: impl Into<GLenum>,
        data: impl AsRef<[T]>,
        usage: impl Into<GLenum>,
    ) {
        self.bind_then(target, |b| b.data::<T>(data, usage));
    }
    pub fn bind_to(
        &self,
        attribute: Attribute,
        format: AttributeFormat,
        stride: GLsizei,
        offset: usize,
    ) {
        self.bind_then(gl::ARRAY_BUFFER, |b| {
            b.bind_to(attribute, format, stride, offset)
        })
    }
}

#[non_exhaustive]
pub struct BoundBuffer(GLenum);

impl BoundBuffer {
    pub fn data<T>(&self, data: impl AsRef<[T]>, usage: impl Into<GLenum>) {
        let data = data.as_ref();
        let n_bytes = size_of::<T>() * data.len();
        unsafe {
            gl::BufferData(self.0, n_bytes as _, data.as_ptr() as _, usage.into());
        }
    }
    pub fn bind_to(
        &self,
        attribute: Attribute,
        format: AttributeFormat,
        stride: GLsizei,
        offset: usize,
    ) {
        unsafe { attribute.pointer(format, stride, offset as _) }
    }
}

pub trait GenDelBuffersExt {
    fn gen_buffers(&mut self);
    fn del_buffers(&mut self);
}

impl<T: AsMut<[Buffer]>> GenDelBuffersExt for T {
    fn gen_buffers(&mut self) {
        let as_mut = self.as_mut();
        unsafe { gl::GenBuffers(as_mut.len() as i32, as_mut.as_mut_ptr() as _) }
    }

    fn del_buffers(&mut self) {
        let as_mut = self.as_mut();
        unsafe { gl::DeleteBuffers(as_mut.len() as i32, as_mut.as_mut_ptr() as _) }
    }
}
