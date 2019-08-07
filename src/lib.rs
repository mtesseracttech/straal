use core::fmt;

pub use glium;
pub use num;

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

pub type Vec2n = Vec2<f32>;
pub type Vec2h = Vec2<f64>;

pub type Vec3n = Vec3<f32>;
pub type Vec3h = Vec3<f64>;

pub type Vec4n = Vec4<f32>;
pub type Vec4h = Vec4<f64>;

pub type Mat2n = Mat2<f32>;
pub type Mat2h = Mat2<f64>;

pub type Mat3n = Mat3<f32>;
pub type Mat3h = Mat3<f64>;

pub type Mat4n = Mat4<f32>;
pub type Mat4h = Mat4<f64>;

pub type Quatn = Quat<f32>;
pub type Quath = Quat<f64>;

pub type IVec2n = IVec2<i32>;
pub type IVec2h = IVec2<i64>;

pub type IVec3n = IVec3<i32>;
pub type IVec3h = IVec3<i64>;

pub type IVec4n = IVec4<i32>;
pub type IVec4h = IVec4<i64>;

pub enum RotationOrder {
    PHB,
    PBH,
    HPB,
    HBP,
    BPH,
    BHP,
}

pub trait FloatType<T>: num::Float + DefaultEpsilon<T> + fmt::Display {}

pub trait IntegerType: num::Integer + num::NumCast + num::Signed + Copy + fmt::Display {}

pub trait InputType: num::Num + num::NumCast + Copy + fmt::Display {}


impl<T: num::Float + DefaultEpsilon<T> + fmt::Display> FloatType<T> for T {}

impl<T: num::Integer + num::NumCast + num::Signed + Copy + fmt::Display> IntegerType for T {}

impl<T: num::Num + num::NumCast + Copy + fmt::Display> InputType for T {}


pub trait DefaultEpsilon<S> {
    const DEF_EPSILON: S;
}

impl DefaultEpsilon<f32> for f32 {
    const DEF_EPSILON: f32 = 1e-5;
}

impl DefaultEpsilon<f64> for f64 {
    const DEF_EPSILON: f64 = 1e-13;
}

pub trait ApproxEqual<S> where S: num::Float + DefaultEpsilon<S> {
    fn approx_eq(self, rhs: Self, epsilon: S) -> bool;
}

impl<S> ApproxEqual<S> for S where S: num::Float + DefaultEpsilon<S> {
    fn approx_eq(self, rhs: S, epsilon: S) -> bool {
        let abs_a = self.abs();
        let abs_b = rhs.abs();
        let diff = (self - rhs).abs();
        if self == rhs {
            true
        } else if self == S::zero() || rhs == S::zero() || diff < S::min_positive_value() {
            diff < epsilon
        } else {
            diff / S::min(abs_a + abs_b, S::max_value()) < epsilon
        }
    }
}