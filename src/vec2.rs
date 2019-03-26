use std::fmt;
use std::fmt::Display;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec2<S> {
    pub x: S,
    pub y: S,
}

impl<S> Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    pub fn zero() -> Vec2<S> {
        Vec2 {
            x: S::zero(),
            y: S::zero(),
        }
    }

    pub fn one() -> Vec2<S> {
        Vec2 {
            x: S::one(),
            y: S::one(),
        }
    }

    pub fn right() -> Vec2<S> {
        Vec2 {
            x: S::one(),
            y: S::zero(),
        }
    }

    pub fn up() -> Vec2<S> {
        Vec2 {
            x: S::zero(),
            y: S::one(),
        }
    }

    pub fn new<U>(x: U, y: U) -> Vec2<S> where U: num::Num + num::NumCast + Copy {
        Vec2 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> Vec2<S> where U: num::Num + num::NumCast + Copy {
        Vec2 {
            x: num::cast(t).unwrap(),
            y: num::cast(t).unwrap(),
        }
    }

    pub fn dot(self, rhs: Vec2<S>) -> S {
        self.x * rhs.x + self.y * rhs.y
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
        2
    }

    pub fn normalized(&self) -> Vec2<S> {
        let scale = S::one() / self.length();
        Vec2 {
            x: self.x,
            y: self.y,
        } * scale
    }

    pub fn normalize(&mut self) {
        let scale = S::one() / self.length();
        self.x = self.x * scale;
        self.y = self.y * scale;
    }
}

impl<S> Index<usize> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index)
        }
    }
}

impl<S> Neg for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<S> Add<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Vec2<S> {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<S> AddAssign<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn add_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<S> Sub<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn sub(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<S> SubAssign<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn sub_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<S> Mul<S> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<S> MulAssign<S> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<S> Mul<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<S> MulAssign<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

//GLSL-like reversed multiplication rule where it is vec * transpose
impl<S> Mul<Mat2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: Mat2<S>) -> Self::Output {
        let rhs = rhs.transpose();
        Vec2 {
            x: self.dot(rhs.r0),
            y: self.dot(rhs.r1),
        }
    }
}

impl<S> Div<S> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        Vec2 {
            x: self.x,
            y: self.y,
        } * inv
    }
}

impl<S> Div<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec2<S>;

    fn div(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}


impl<S> DivAssign<S> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
    }
}

impl<S> DivAssign<Vec2<S>> for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl<S> PartialEq for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn eq(&self, other: &Vec2<S>) -> bool {
        self.x.approx_eq(other.x, S::DEF_EPSILON) &&
            self.y.approx_eq(other.y, S::DEF_EPSILON)
    }
}


impl<S> fmt::Display for Vec2<S> where S: num::Float + DefaultEpsilon<S> + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3} {:.3})", self.x, self.y)
    }
}

impl<S, U> From<(U, U)> for Vec2<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(tuple: (U, U)) -> Vec2<S> {
        Vec2 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
        }
    }
}

impl<S, U> From<[U; 2]> for Vec2<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(arr: [U; 2]) -> Vec2<S> {
        Vec2 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
        }
    }
}


impl<S, U> From<Vec3<U>> for Vec2<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(vec3: Vec3<U>) -> Vec2<S> {
        Vec2 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
        }
    }
}


impl<S, U> From<Vec4<U>> for Vec2<S> where S: num::Float + DefaultEpsilon<S>, U: num::Num + num::NumCast + Copy {
    fn from(vec4: Vec4<U>) -> Vec2<S> {
        Vec2 {
            x: num::cast(vec4.x).unwrap(),
            y: num::cast(vec4.y).unwrap(),
        }
    }
}

impl<S> Default for Vec2<S> where S: num::Float + DefaultEpsilon<S> {
    fn default() -> Vec2<S> {
        Vec2::zero()
    }
}

impl glium::uniforms::AsUniformValue for Vec2<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Vec2(std::mem::transmute::<Vec2<f32>, [f32; 2]>(*self))
        }
    }
}


impl glium::uniforms::AsUniformValue for Vec2<f64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::DoubleVec2(std::mem::transmute::<Vec2<f64>, [f64; 2]>(*self))
        }
    }
}


unsafe impl glium::vertex::Attribute for Vec2<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for Vec2<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}