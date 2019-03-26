use std::fmt;
use std::fmt::Display;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

impl<S> Vec4<S> where S: num::Float + DefaultEpsilon<S>,
{
    pub fn zero() -> Vec4<S> {
        Vec4 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
            w: S::zero(),
        }
    }

    pub fn one() -> Vec4<S> {
        Vec4 {
            x: S::one(),
            y: S::one(),
            z: S::one(),
            w: S::one(),
        }
    }

    pub fn right() -> Vec4<S> {
        Vec4 {
            x: S::one(),
            y: S::zero(),
            z: S::zero(),
            w: S::zero(),
        }
    }

    pub fn up() -> Vec4<S> {
        Vec4 {
            x: S::zero(),
            y: S::one(),
            z: S::zero(),
            w: S::zero(),
        }
    }

    pub fn forward() -> Vec4<S> {
        Vec4 {
            x: S::zero(),
            y: S::zero(),
            z: S::one(),
            w: S::zero(),
        }
    }

    pub fn w_only() -> Vec4<S> {
        Vec4 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
            w: S::one(),
        }
    }

    pub fn new<U>(x: U, y: U, z: U, w: U) -> Vec4<S> where U: num::Num + num::NumCast + Copy {
        Vec4 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
            z: num::cast(z).unwrap(),
            w: num::cast(w).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> Vec4<S> where U: num::Num + num::NumCast + Copy {
        Vec4 {
            x: num::cast(t).unwrap(),
            y: num::cast(t).unwrap(),
            z: num::cast(t).unwrap(),
            w: num::cast(t).unwrap(),
        }
    }

    pub fn dot(self, rhs: Vec4<S>) -> S {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn length_squared(self) -> S {
        self.dot(self)
    }

    pub fn length(self) -> S {
        self.length_squared().sqrt()
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared().approx_eq(S::one(), S::DEF_EPSILON)
    }

    pub fn size() -> usize {
        4
    }

    pub fn normalized(&self) -> Vec4<S> {
        let scale = S::one() / self.length();
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        } * scale
    }

    pub fn normalize(&mut self) {
        let scale = S::one() / self.length();
        self.x = self.x * scale;
        self.y = self.y * scale;
        self.z = self.z * scale;
        self.w = self.w * scale;
    }
}


impl<S> Index<usize> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Requested an invalid index on a Vec4: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Requested an invalid index on a Vec4: {}", index)
        }
    }
}

impl<S> Neg for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn neg(self) -> Self::Output {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<S> Add<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn add(self, rhs: Vec4<S>) -> Vec4<S> {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<S> AddAssign<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn add_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

impl<S> Sub<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn sub(self, rhs: Vec4<S>) -> Self::Output {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<S> SubAssign<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn sub_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}


impl<S> Mul<S> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<S> MulAssign<S> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

impl<S> Mul<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn mul(self, rhs: Vec4<S>) -> Self::Output {
        Vec4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<S> MulAssign<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self.w = self.w * rhs.w;
    }
}


impl<S> Div<S> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        } * inv
    }
}

impl<S> Div<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn div(self, rhs: Vec4<S>) -> Self::Output {
        Vec4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}


impl<S> DivAssign<S> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
        self.z = self.z * inv;
        self.w = self.w * inv;
    }
}

impl<S> DivAssign<Vec4<S>> for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self.w = self.w / rhs.w;
    }
}

impl<S> PartialEq for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn eq(&self, other: &Vec4<S>) -> bool {
        self.x.approx_eq(other.x, S::DEF_EPSILON) &&
            self.y.approx_eq(other.y, S::DEF_EPSILON) &&
            self.z.approx_eq(other.z, S::DEF_EPSILON) &&
            self.w.approx_eq(other.w, S::DEF_EPSILON)
    }
}

impl<S> fmt::Display for Vec4<S> where S: num::Float + DefaultEpsilon<S> + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3} {:.3} {:.3} {:.3})", self.x, self.y, self.z, self.w)
    }
}

impl<S, U> From<(U, U, U, U)> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (U, U, U, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2).unwrap(),
            w: num::cast(tuple.3).unwrap(),
        }
    }
}

impl<S, U> From<[U; 4]> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(arr: [U; 4]) -> Vec4<S> {
        Vec4 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
            z: num::cast(arr[2]).unwrap(),
            w: num::cast(arr[3]).unwrap(),
        }
    }
}

impl<S, U> From<Vec2<U>> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(vec3: Vec2<U>) -> Vec4<S> {
        Vec4 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
            z: S::zero(),
            w: S::zero(),
        }
    }
}


impl<S, U> From<Vec3<U>> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(vec3: Vec3<U>) -> Vec4<S> {
        Vec4 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
            z: num::cast(vec3.z).unwrap(),
            w: S::zero(),
        }
    }
}


impl<S, U> From<(Vec2<U>, Vec2<U>)> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (Vec2<U>, Vec2<U>)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.1.x).unwrap(),
            w: num::cast(tuple.1.y).unwrap(),
        }
    }
}

impl<S, U> From<(U, Vec2<U>, U)> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (U, Vec2<U>, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1.x).unwrap(),
            z: num::cast(tuple.1.y).unwrap(),
            w: num::cast(tuple.2).unwrap(),
        }
    }
}

impl<S, U> From<(U, U, Vec2<U>)> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (U, U, Vec2<U>)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2.x).unwrap(),
            w: num::cast(tuple.2.y).unwrap(),
        }
    }
}

impl<S, U> From<(Vec2<U>, U, U)> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (Vec2<U>, U, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.1).unwrap(),
            w: num::cast(tuple.2).unwrap(),
        }
    }
}

impl<S, U> From<(U, Vec3<U>)> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (U, Vec3<U>)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1.x).unwrap(),
            z: num::cast(tuple.1.y).unwrap(),
            w: num::cast(tuple.1.z).unwrap(),
        }
    }
}

impl<S, U> From<(Vec3<U>, U)> for Vec4<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (Vec3<U>, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.0.z).unwrap(),
            w: num::cast(tuple.1).unwrap(),
        }
    }
}

impl<S> Default for Vec4<S> where S: num::Float + DefaultEpsilon<S> {
    fn default() -> Vec4<S> {
        Vec4::zero()
    }
}

impl glium::uniforms::AsUniformValue for Vec4<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Vec4(std::mem::transmute::<Vec4<f32>, [f32; 4]>(*self))
        }
    }
}


impl glium::uniforms::AsUniformValue for Vec4<f64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::DoubleVec4(std::mem::transmute::<Vec4<f64>, [f64; 4]>(*self))
        }
    }
}


unsafe impl glium::vertex::Attribute for Vec4<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32F32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for Vec4<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64F64F64
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}