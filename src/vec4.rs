use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct Vec4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

impl<S> Vec4<S> where S: FloatType<S>,
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

    pub fn new<U>(x: U, y: U, z: U, w: U) -> Vec4<S> where U: InputType {
        Vec4 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
            z: num::cast(z).unwrap(),
            w: num::cast(w).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> Vec4<S> where U: InputType {
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

    pub fn reflect(i: Vec4<S>, n: Vec4<S>) -> Vec4<S> {
        assert!(n.is_unit());
        n * S::from(2).unwrap() * n.dot(i) - i
    }

    pub fn refract(i: Vec4<S>, n: Vec4<S>, eta: S) -> Vec4<S> {
        assert!(n.is_unit());
        let k = S::one() - eta * eta * (S::one() - n.dot(i) * n.dot(i));
        if k < S::zero() {
            Vec4::zero()
        } else {
            (i * eta) - n * (eta * n.dot(i) + k.sqrt())
        }
    }

    pub fn get_largest(&self) -> S {
        self.x.max(self.y.max(self.z.max(self.w)))
    }

    pub fn get_smallest(&self) -> S {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    pub fn get_largest_index(&self) -> usize {
        let mut i = 0;
        let mut largest = self.x;
        if self.y > largest {
            i = 1;
        }
        if self.z > largest {
            i = 2
        }
        if self.w > largest {
            i = 3
        }
        i
    }

    pub fn get_smallest_index(&self) -> usize {
        let mut i = 0;
        let mut smallest = self.x;
        if self.y < smallest {
            i = 1;
        }
        if self.z < smallest {
            i = 2
        }
        if self.w < smallest {
            i = 3
        }
        i
    }

    pub fn distance(a: Vec4<S>, b: Vec4<S>) -> S {
        (b - a).length()
    }

    pub fn mix(a: Vec4<S>, b: Vec4<S>, t: Vec4<S>) -> Vec4<S> {
        (b - a) * (Vec4::one() - t)
    }

    pub fn mix_all(a: Vec4<S>, b: Vec4<S>, t: S) -> Vec4<S> {
        (b - a) * (S::one() - t)
    }

    pub fn clamp(a: Vec4<S>, min: Vec4<S>, max: Vec4<S>) -> Vec4<S> {
        Vec4 {
            x: num::clamp(a.x, min.x, max.x),
            y: num::clamp(a.y, min.y, max.y),
            z: num::clamp(a.z, min.z, max.z),
            w: num::clamp(a.w, min.w, max.w),
        }
    }

    pub fn clamp_all(a: Vec4<S>, min: S, max: S) -> Vec4<S> {
        Vec4 {
            x: num::clamp(a.x, min, max),
            y: num::clamp(a.y, min, max),
            z: num::clamp(a.z, min, max),
            w: num::clamp(a.w, min, max),
        }
    }
}


impl<S> Index<usize> for Vec4<S> where S: FloatType<S> {
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

impl<S> IndexMut<usize> for Vec4<S> where S: FloatType<S> {
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

impl<S> Neg for Vec4<S> where S: FloatType<S> {
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

impl<S> Add<Vec4<S>> for Vec4<S> where S: FloatType<S> {
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

impl<S> AddAssign<Vec4<S>> for Vec4<S> where S: FloatType<S> {
    fn add_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

impl<S> Sub<Vec4<S>> for Vec4<S> where S: FloatType<S> {
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

impl<S> SubAssign<Vec4<S>> for Vec4<S> where S: FloatType<S> {
    fn sub_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}


impl<S> Mul<S> for Vec4<S> where S: FloatType<S> {
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

impl<S> MulAssign<S> for Vec4<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

impl<S> Mul<Vec4<S>> for Vec4<S> where S: FloatType<S> {
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

impl<S> MulAssign<Vec4<S>> for Vec4<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self.w = self.w * rhs.w;
    }
}


impl<S> Div<S> for Vec4<S> where S: FloatType<S> {
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

impl<S> Div<Vec4<S>> for Vec4<S> where S: FloatType<S> {
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


impl<S> DivAssign<S> for Vec4<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
        self.z = self.z * inv;
        self.w = self.w * inv;
    }
}

impl<S> DivAssign<Vec4<S>> for Vec4<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: Vec4<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self.w = self.w / rhs.w;
    }
}

impl<S> PartialEq for Vec4<S> where S: FloatType<S> {
    fn eq(&self, other: &Vec4<S>) -> bool {
        self.x.approx_eq(other.x, S::DEF_EPSILON) &&
            self.y.approx_eq(other.y, S::DEF_EPSILON) &&
            self.z.approx_eq(other.z, S::DEF_EPSILON) &&
            self.w.approx_eq(other.w, S::DEF_EPSILON)
    }
}

impl<S> fmt::Display for Vec4<S> where S: FloatType<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3} {:.3} {:.3} {:.3})", self.x, self.y, self.z, self.w)
    }
}

impl<S, U> From<(U, U, U, U)> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (U, U, U, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2).unwrap(),
            w: num::cast(tuple.3).unwrap(),
        }
    }
}

impl<S, U> From<[U; 4]> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(arr: [U; 4]) -> Vec4<S> {
        Vec4 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
            z: num::cast(arr[2]).unwrap(),
            w: num::cast(arr[3]).unwrap(),
        }
    }
}

impl<S, U> From<Vec2<U>> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(vec3: Vec2<U>) -> Vec4<S> {
        Vec4 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
            z: S::zero(),
            w: S::zero(),
        }
    }
}


impl<S, U> From<Vec3<U>> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(vec3: Vec3<U>) -> Vec4<S> {
        Vec4 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
            z: num::cast(vec3.z).unwrap(),
            w: S::zero(),
        }
    }
}


impl<S, U> From<(Vec2<U>, Vec2<U>)> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (Vec2<U>, Vec2<U>)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.1.x).unwrap(),
            w: num::cast(tuple.1.y).unwrap(),
        }
    }
}

impl<S, U> From<(U, Vec2<U>, U)> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (U, Vec2<U>, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1.x).unwrap(),
            z: num::cast(tuple.1.y).unwrap(),
            w: num::cast(tuple.2).unwrap(),
        }
    }
}

impl<S, U> From<(U, U, Vec2<U>)> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (U, U, Vec2<U>)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2.x).unwrap(),
            w: num::cast(tuple.2.y).unwrap(),
        }
    }
}

impl<S, U> From<(Vec2<U>, U, U)> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (Vec2<U>, U, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.1).unwrap(),
            w: num::cast(tuple.2).unwrap(),
        }
    }
}

impl<S, U> From<(U, Vec3<U>)> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (U, Vec3<U>)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1.x).unwrap(),
            z: num::cast(tuple.1.y).unwrap(),
            w: num::cast(tuple.1.z).unwrap(),
        }
    }
}

impl<S, U> From<(Vec3<U>, U)> for Vec4<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (Vec3<U>, U)) -> Vec4<S> {
        Vec4 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.0.z).unwrap(),
            w: num::cast(tuple.1).unwrap(),
        }
    }
}

impl<S> Default for Vec4<S> where S: FloatType<S> {
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

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for Vec4<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64F64F64
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}