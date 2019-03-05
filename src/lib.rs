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

const DEF_F32_EPSILON: f32 = 0.00001;

trait ApproxEqual {
    fn approx_eq(self, rhs: Self, epsilon: f32) -> bool;
}

impl ApproxEqual for Scalar {
    fn approx_eq(self, rhs: f32, epsilon: f32) -> bool {
        let abs_a = self.abs();
        let abs_b = rhs.abs();
        let diff = (self - rhs).abs();

        //print!("{:.10} =? {:.10}", self, rhs);
        // shortcut, handles infinities
        if self == rhs {
            //println!("Taking the infinities/100% equal path");
            true
        } else if self == 0.0 || rhs == 0.0 || diff < std::f32::MIN_POSITIVE {
            // a or b is zero or both are extremely close to it
            // relative error is less meaningful here
            //diff < (epsilon * std::f32::MIN_POSITIVE) //idk about this bit, it's waaay too small, even with a relatively big epsilon
            diff < epsilon
        } else { // use relative error
            //println!("Taking the relative error path");
            //println!("Relative error: {}, Epsilon: {}, Answer: {}", diff / f32::min((abs_a + abs_b), std::f32::MAX), epsilon, diff / f32::min((abs_a + abs_b), std::f32::MAX) < epsilon);

            diff / f32::min((abs_a + abs_b), std::f32::MAX) < epsilon
        }
    }
}
