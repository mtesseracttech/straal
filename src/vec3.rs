use std::fmt;
use std::fmt::Display;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S> Vec3<S> where S: num::Float + DefaultEpsilon<S>,
{
    pub fn zero() -> Vec3<S> {
        Vec3 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
        }
    }

    pub fn one() -> Vec3<S> {
        Vec3 {
            x: S::one(),
            y: S::one(),
            z: S::one(),
        }
    }

    pub fn right() -> Vec3<S> {
        Vec3 {
            x: S::one(),
            y: S::zero(),
            z: S::zero(),
        }
    }

    pub fn up() -> Vec3<S> {
        Vec3 {
            x: S::zero(),
            y: S::one(),
            z: S::zero(),
        }
    }

    pub fn forward() -> Vec3<S> {
        Vec3 {
            x: S::zero(),
            y: S::zero(),
            z: S::one(),
        }
    }

    pub fn new<U>(x: U, y: U, z: U) -> Vec3<S> where U: num::Num + num::NumCast + Copy {
        Vec3 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
            z: num::cast(z).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> Vec3<S> where U: num::Num + num::NumCast + Copy {
        Vec3 {
            x: num::cast(t).unwrap(),
            y: num::cast(t).unwrap(),
            z: num::cast(t).unwrap(),
        }
    }

    pub fn dot(self, rhs: Vec3<S>) -> S {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3<S>) -> Vec3<S> {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
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
        3
    }

    pub fn normalized(&self) -> Vec3<S> {
        let scale = S::one() / self.length();
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        } * scale
    }

    pub fn normalize(&mut self) {
        let scale = S::one() / self.length();
        self.x = self.x * scale;
        self.y = self.y * scale;
        self.z = self.z * scale;
    }
}


impl<S> Index<usize> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Requested an invalid index on a Vec3: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Requested an invalid index on a Vec3: {}", index)
        }
    }
}

impl<S> Neg for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<S> Add<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn add(self, rhs: Vec3<S>) -> Vec3<S> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<S> AddAssign<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn add_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<S> Sub<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn sub(self, rhs: Vec3<S>) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<S> SubAssign<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn sub_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}


impl<S> Mul<S> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl<S> MulAssign<S> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<S> Mul<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: Vec3<S>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<S> MulAssign<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}


impl<S> Div<S> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        } * inv
    }
}

impl<S> Div<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn div(self, rhs: Vec3<S>) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}


impl<S> DivAssign<S> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
        self.z = self.z * inv;
    }
}

impl<S> DivAssign<Vec3<S>> for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}


impl<S> PartialEq for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn eq(&self, other: &Vec3<S>) -> bool {
        self.x.approx_eq(other.x, S::DEF_EPSILON) &&
            self.y.approx_eq(other.y, S::DEF_EPSILON) &&
            self.z.approx_eq(other.z, S::DEF_EPSILON)
    }
}

impl<S> fmt::Display for Vec3<S> where S: num::Float + DefaultEpsilon<S> + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3} {:.3} {:.3})", self.x, self.y, self.z)
    }
}

impl<S, U> From<(U, U, U)> for Vec3<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (U, U, U)) -> Vec3<S> {
        Vec3 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2).unwrap(),
        }
    }
}

impl<S, U> From<[U; 3]> for Vec3<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(arr: [U; 3]) -> Vec3<S> {
        Vec3 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
            z: num::cast(arr[2]).unwrap(),
        }
    }
}

impl<S, U> From<Vec2<U>> for Vec3<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(vec3: Vec2<U>) -> Vec3<S> {
        Vec3 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
            z: S::zero(),
        }
    }
}


impl<S, U> From<Vec4<U>> for Vec3<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(vec4: Vec4<U>) -> Vec3<S> {
        Vec3 {
            x: num::cast(vec4.x).unwrap(),
            y: num::cast(vec4.y).unwrap(),
            z: num::cast(vec4.z).unwrap(),
        }
    }
}

impl<S, U> From<(U, Vec2<U>)> for Vec3<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (U, Vec2<U>)) -> Vec3<S> {
        Vec3 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1.x).unwrap(),
            z: num::cast(tuple.1.y).unwrap(),
        }
    }
}

impl<S, U> From<(Vec2<U>, U)> for Vec3<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (Vec2<U>, U)) -> Vec3<S> {
        Vec3 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.1).unwrap(),
        }
    }
}


impl<S> Default for Vec3<S> where S: num::Float + DefaultEpsilon<S> {
    fn default() -> Vec3<S> {
        Vec3::zero()
    }
}

impl glium::uniforms::AsUniformValue for Vec3<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Vec3(std::mem::transmute::<Vec3<f32>, [f32; 3]>(*self))
        }
    }
}


impl glium::uniforms::AsUniformValue for Vec3<f64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::DoubleVec3(std::mem::transmute::<Vec3<f64>, [f64; 3]>(*self))
        }
    }
}


unsafe impl glium::vertex::Attribute for Vec3<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for Vec3<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64F64
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}