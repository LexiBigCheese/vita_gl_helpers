use std::ffi::c_void;

use derive_more::From;
use gl::types::{GLsizei, GLuint};

use crate::program::Program;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Attribute(pub GLuint);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AttributeSize {
    ONE = 1,
    TWO,
    THREE,
    FOUR,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum AttributeType {
    Byte = gl::BYTE,
    UnsignedByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Fixed = gl::FIXED,
    Float = gl::FLOAT,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttributeFormat {
    pub size: AttributeSize,
    pub type_: AttributeType,
    pub normalized: bool,
}

impl Attribute {
    ///Note on vitaGL, the only recognised divisors are 0 and 1
    pub fn divisor(&self, divisor: GLuint) {
        unsafe {
            gl::VertexAttribDivisor(self.0, divisor);
        }
    }
    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.0);
        }
    }
    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.0);
        }
    }
    pub unsafe fn pointer(&self, format: AttributeFormat, stride: GLsizei, pointer: *const c_void) {
        unsafe {
            gl::VertexAttribPointer(
                self.0,
                format.size as _,
                format.type_ as _,
                if format.normalized {
                    gl::TRUE
                } else {
                    gl::FALSE
                },
                stride,
                pointer,
            );
        }
    }
}

#[derive(Debug, From)]
pub struct MissingAttributes(pub Vec<&'static str>);

impl std::fmt::Display for MissingAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing Attributes: [{}]", self.0.join(","))
    }
}

impl std::error::Error for MissingAttributes {}

pub trait AttributeTable: Sized {
    fn with_locations_from(p: &Program) -> Result<Self, MissingAttributes>;
    fn attributes(&self) -> impl Iterator<Item = &Attribute>;
    fn enable_all(&self) {
        self.attributes().for_each(Attribute::enable);
    }
    fn disable_all(&self) {
        self.attributes().for_each(Attribute::disable);
    }
}

#[macro_export]
macro_rules! attribute_table {
    ($sname:ident,$($lname:ident => $lstr:expr),*) => {
        pub struct $sname {
            $($lname: $crate::attribute::Attribute),*
        }
        impl $crate::attribute::AttributeTable for $sname {
            fn with_locations_from(p: &$crate::program::Program) -> Result<Self,$crate::attribute::MissingAttributes> {
                let to_check = [$($lstr),*];
                let locations = [$(p.get_attrib_location($lstr)),*];
                let errors: Vec<&'static str> = to_check.into_iter().zip(locations.iter()).filter_map(|(n,&l)| if l < 0 {Some(n)} else {None}).collect();
                if !errors.is_empty() {
                    return Err($crate::attribute::MissingAttributes(errors));
                }
                let mut locations_iter = locations.into_iter();
                Ok($sname {
                    $($lname: $crate::attribute::Attribute(locations_iter.next().unwrap() as u32)),*
                })
            }
            fn attributes(&self) -> impl Iterator<Item=&$crate::attribute::Attribute> {
                [
                    $(&self.$lname),*
                ].into_iter()
            }
        }
    }
}
