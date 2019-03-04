#![allow(dead_code)]

extern crate rand;

pub use ivec2::*;
pub use ivec3::*;
pub use ivec4::*;
pub use mat2::*;
pub use mat3::*;
pub use mat4::*;
pub use quat::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

pub mod ivec2;
pub mod ivec3;
pub mod ivec4;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod quat;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub type Scalar = f32;
pub type Integer = i32;

