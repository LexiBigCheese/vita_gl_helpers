use std::ffi::c_void;

use derive_more::{From, Into};
use gl::types::{GLenum, GLint, GLuint};

/// To create and delete textures:
/// ```rust
/// use vita_gl_helpers::texture::GenDelTexturesExt;
/// let mut textures = vec![Texture::default();5];
/// textures.gen_textures();
/// // Do things with the textures
/// textures.del_textures();
/// ```
#[derive(From, Into, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Texture(GLuint);

#[non_exhaustive]
pub struct BoundTexture(GLenum);

impl Texture {
    pub fn bind(&self, bindpoint: GLenum) {
        unsafe {
            gl::BindTexture(bindpoint, self.0);
        }
    }
    pub fn bind_then<R>(&self, bindpoint: GLenum, then: impl FnOnce(BoundTexture) -> R) -> R {
        self.bind(bindpoint);
        then(BoundTexture(bindpoint))
    }
}

impl BoundTexture {
    pub fn image_2d(
        &self,
        level: impl Into<GLint>,
        internalformat: impl Into<GLint>,
        width: impl Into<GLint>,
        height: impl Into<GLint>,
        format: impl Into<GLenum>,
        type_: impl Into<GLenum>,
        pixels: *const c_void,
    ) {
        unsafe {
            gl::TexImage2D(
                self.0,
                level.into(),
                internalformat.into(),
                width.into(),
                height.into(),
                0,
                format.into(),
                type_.into(),
                pixels,
            );
        }
    }
    pub fn gen_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(self.0);
        }
    }
    pub fn parameter_i(&self, pname: impl Into<GLenum>, param: impl Into<GLint>) {
        unsafe {
            gl::TexParameteri(self.0, pname.into(), param.into());
        }
    }
}

pub trait GenDelTexturesExt {
    fn gen_textures(&mut self);
    fn delete_textures(&mut self);
}

impl<T: AsMut<[Texture]>> GenDelTexturesExt for T {
    fn gen_textures(&mut self) {
        let as_mut = self.as_mut();
        unsafe { gl::GenTextures(as_mut.len() as i32, as_mut.as_mut_ptr() as _) }
    }

    fn delete_textures(&mut self) {
        let as_mut = self.as_mut();
        unsafe { gl::DeleteTextures(as_mut.len() as i32, as_mut.as_mut_ptr() as _) }
    }
}
