use derive_more::TryFrom;

#[derive(TryFrom, Debug, PartialEq, Eq)]
#[try_from(repr)]
#[repr(u32)]
pub enum GlError {
    NoError = gl::NO_ERROR,
    InvalidEnum = gl::INVALID_ENUM,
    InvalidValue = gl::INVALID_VALUE,
    InvalidOperation = gl::INVALID_OPERATION,
    InvalidFramebufferOperation = gl::INVALID_FRAMEBUFFER_OPERATION,
    OutOfMemory = gl::OUT_OF_MEMORY,
    StackUnderflow = gl::STACK_UNDERFLOW,
    StackOverflow = gl::STACK_OVERFLOW,
}

impl std::fmt::Display for GlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GlError::NoError => "GL_NO_ERROR",
                GlError::InvalidEnum => "GL_INVALID_ENUM",
                GlError::InvalidValue => "GL_INVALID_VALUE",
                GlError::InvalidOperation => "GL_INVALID_OPERATION",
                GlError::InvalidFramebufferOperation => "GL_INVALID_FRAMEBUFFER_OPERATION",
                GlError::OutOfMemory => "GL_OUT_OF_MEMORY",
                GlError::StackUnderflow => "GL_STACK_UNDERFLOW",
                GlError::StackOverflow => "GL_STACK_OVERFLOW",
            }
        )
    }
}

pub fn get_error() -> GlError {
    unsafe {
        gl::GetError()
            .try_into()
            .expect("Unexpected Error in glGetError")
    }
}

pub struct Errors;

impl Iterator for Errors {
    type Item = GlError;

    fn next(&mut self) -> Option<Self::Item> {
        let next_error = get_error();
        if next_error == GlError::NoError {
            None
        } else {
            Some(next_error)
        }
    }
}

pub fn eprintln_errors() {
    for error in Errors {
        eprintln!("GL ERROR: {error}");
    }
}
