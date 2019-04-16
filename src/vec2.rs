use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct Vec2<S> {
    pub x: S,
    pub y: S,
}

impl<S> Vec2<S> where S: FloatType<S> {
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

    pub fn new<U>(x: U, y: U) -> Vec2<S> where U: InputType {
        Vec2 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> Vec2<S> where U: InputType {
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

    pub fn reflect(i: Vec2<S>, n: Vec2<S>) -> Vec2<S> {
        assert!(n.is_unit());
        n * S::from(2).unwrap() * n.dot(i) - i
    }

    pub fn refract(i: Vec2<S>, n: Vec2<S>, eta: S) -> Vec2<S> {
        assert!(n.is_unit());
        let k = S::one() - eta * eta * (S::one() - n.dot(i) * n.dot(i));
        if k < S::zero() {
            Vec2::zero()
        } else {
            (i * eta) - n * (eta * n.dot(i) + k.sqrt())
        }
    }

    pub fn get_largest(&self) -> S {
        self.x.max(self.y)
    }

    pub fn get_smallest(&self) -> S {
        self.x.min(self.y)
    }

    pub fn get_largest_index(&self) -> usize {
        let mut i = 0;
        let mut largest = self.x;
        if self.y > largest {
            i = 1;
        }
        i
    }

    pub fn get_smallest_index(&self) -> usize {
        let mut i = 0;
        let mut smallest = self.x;
        if self.y < smallest {
            i = 1;
        }
        i
    }

    pub fn distance(a: Vec2<S>, b: Vec2<S>) -> S {
        (b - a).length()
    }

    pub fn mix(a: Vec2<S>, b: Vec2<S>, t: Vec2<S>) -> Vec2<S> {
        (b - a) * (Vec2::one() - t)
    }

    pub fn mix_all(a: Vec2<S>, b: Vec2<S>, t: S) -> Vec2<S> {
        (b - a) * (S::one() - t)
    }

    pub fn clamp(a: Vec2<S>, min: Vec2<S>, max: Vec2<S>) -> Vec2<S> {
        Vec2 {
            x: num::clamp(a.x, min.x, max.x),
            y: num::clamp(a.y, min.y, max.y),
        }
    }

    pub fn clamp_all(a: Vec2<S>, min: S, max: S) -> Vec2<S> {
        Vec2 {
            x: num::clamp(a.x, min, max),
            y: num::clamp(a.y, min, max),
        }
    }
}

impl<S> Index<usize> for Vec2<S> where S: FloatType<S> {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for Vec2<S> where S: FloatType<S> {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index)
        }
    }
}

impl<S> Neg for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<S> Add<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Vec2<S> {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<S> AddAssign<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    fn add_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<S> Sub<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn sub(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<S> SubAssign<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    fn sub_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<S> Mul<S> for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<S> MulAssign<S> for Vec2<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<S> Mul<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<S> MulAssign<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

//GLSL-like reversed multiplication rule where it is vec * transpose
impl<S> Mul<Mat2<S>> for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: Mat2<S>) -> Self::Output {
        let rhs = rhs.transpose();
        Vec2 {
            x: self.dot(rhs.r0),
            y: self.dot(rhs.r1),
        }
    }
}

impl<S> Div<S> for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        Vec2 {
            x: self.x,
            y: self.y,
        } * inv
    }
}

impl<S> Div<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn div(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}


impl<S> DivAssign<S> for Vec2<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
    }
}

impl<S> DivAssign<Vec2<S>> for Vec2<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: Vec2<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl<S> PartialEq for Vec2<S> where S: FloatType<S> {
    fn eq(&self, other: &Vec2<S>) -> bool {
        self.x.approx_eq(other.x, S::DEF_EPSILON) &&
            self.y.approx_eq(other.y, S::DEF_EPSILON)
    }
}


impl<S> fmt::Display for Vec2<S> where S: FloatType<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3} {:.3})", self.x, self.y)
    }
}

impl<S, U> From<(U, U)> for Vec2<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (U, U)) -> Vec2<S> {
        Vec2 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
        }
    }
}

impl<S, U> From<[U; 2]> for Vec2<S> where S: FloatType<S>, U: InputType {
    fn from(arr: [U; 2]) -> Vec2<S> {
        Vec2 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
        }
    }
}


impl<S, U> From<Vec3<U>> for Vec2<S> where S: FloatType<S>, U: InputType {
    fn from(vec3: Vec3<U>) -> Vec2<S> {
        Vec2 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
        }
    }
}


impl<S, U> From<Vec4<U>> for Vec2<S> where S: FloatType<S>, U: InputType {
    fn from(vec4: Vec4<U>) -> Vec2<S> {
        Vec2 {
            x: num::cast(vec4.x).unwrap(),
            y: num::cast(vec4.y).unwrap(),
        }
    }
}

impl<S> Default for Vec2<S> where S: FloatType<S> {
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

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for Vec2<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}