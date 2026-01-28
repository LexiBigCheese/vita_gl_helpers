use derive_more::From;

use crate::program::Program;

macro_rules! uniform_def {
    ($name:ident,$accept:ty) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub gl::types::GLint);
        impl $name {
            pub fn set(&self, to: $accept) {
                self.set_multi(&[to])
            }
            pub fn set_multi(&self, to: &[$accept]) {
                self.set_subrange(0, to)
            }
            pub fn set_subrange(&self, offset: usize, to: &[$accept]) {
                unsafe {
                    gl::$name(self.0 + offset as i32, to.len() as _, to.as_ptr() as _);
                }
            }
        }
    };
    ($name:ident,$accept:ty,mat) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub gl::types::GLint);
        impl $name {
            pub fn set(&self, to: $accept, transpose: bool) {
                self.set_multi(&[to], transpose)
            }
            pub fn set_multi(&self, to: &[$accept], transpose: bool) {
                self.set_subrange(0, to, transpose)
            }
            pub fn set_subrange(&self, offset: usize, to: &[$accept], transpose: bool) {
                unsafe {
                    gl::$name(
                        self.0 + offset as i32,
                        to.len() as _,
                        if transpose { gl::TRUE } else { gl::FALSE },
                        to.as_ptr() as _,
                    );
                }
            }
        }
    };
}

uniform_def!(Uniform1fv, f32);
uniform_def!(Uniform2fv, [f32; 2]);
uniform_def!(Uniform3fv, [f32; 3]);
uniform_def!(Uniform4fv, [f32; 4]);

uniform_def!(Uniform1iv, i32);
uniform_def!(Uniform2iv, [i32; 2]);
uniform_def!(Uniform3iv, [i32; 3]);
uniform_def!(Uniform4iv, [i32; 4]);

uniform_def!(UniformMatrix2fv, [f32; 4], mat);
uniform_def!(UniformMatrix3fv, [f32; 9], mat);
uniform_def!(UniformMatrix4fv, [f32; 16], mat);
// uniform_def!(UniformMatrix2x3fv, [f32; 6], mat); Sadly, we don't have these :(
// uniform_def!(UniformMatrix3x2fv, [f32; 6], mat);
// uniform_def!(UniformMatrix2x4fv, [f32; 8], mat);
// uniform_def!(UniformMatrix4x2fv, [f32; 8], mat);
// uniform_def!(UniformMatrix3x4fv, [f32; 12], mat);
// uniform_def!(UniformMatrix4x3fv, [f32; 12], mat);

#[derive(Debug, From)]
pub struct MissingUniforms(pub Vec<&'static str>);

impl std::fmt::Display for MissingUniforms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing Uniforms: [{}]", self.0.join(","))
    }
}

impl std::error::Error for MissingUniforms {}

pub trait UniformTable: Sized {
    fn with_locations_from(p: &Program) -> Result<Self, MissingUniforms>;
}

#[macro_export]
macro_rules! uniform_table {
    ($sname:ident,$($lname:ident : $t:ident => $lstr:expr),*) => {
        #[derive(Default,Clone,Copy,PartialEq,Eq)]
        pub struct $sname {
            $($lname: $crate::uniforms::$t),*
        }
        impl $crate::uniforms::UniformTable for $sname {
            fn with_locations_from(p: &$crate::program::Program) -> Result<Self, $crate::uniforms::MissingUniforms> {
                let to_check = [$($lstr),*];
                let locations = [$(p.get_uniform_location($lstr)),*];
                let errors: Vec<&'static str> = to_check.into_iter().zip(locations.iter()).filter_map(|(n,&l)| if l == -1 {Some(n)} else {None}).collect();
                if !errors.is_empty() {
                    return Err($crate::uniforms::MissingUniforms(errors));
                }
                let mut locations_iter = locations.into_iter();
                Ok($sname {
                    $($lname: $crate::uniforms::$t(locations_iter.next().unwrap())),*
                })
            }
        }
    };
}
