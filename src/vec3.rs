use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct Vec3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S> Vec3<S> where S: FloatType<S>,
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

    pub fn new<U>(x: U, y: U, z: U) -> Vec3<S> where U: InputType {
        Vec3 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
            z: num::cast(z).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> Vec3<S> where U: InputType {
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

    pub fn reflect(i: Vec3<S>, n: Vec3<S>) -> Vec3<S> {
        assert!(n.is_unit());
        n * S::from(2).unwrap() * n.dot(i) - i
    }

    pub fn refract(i: Vec3<S>, n: Vec3<S>, refraction_index: S) -> Vec3<S> {
        assert!(n.is_unit());
        let k = S::one() - refraction_index * refraction_index * (S::one() - n.dot(i) * n.dot(i));
        if k < S::zero() {
            Vec3::zero()
        } else {
            (i * refraction_index) - n * (refraction_index * n.dot(i) + k.sqrt())
        }
    }

    pub fn get_largest(&self) -> S {
        self.x.max(self.y.max(self.z))
    }

    pub fn get_smallest(&self) -> S {
        self.x.min(self.y.min(self.z))
    }

    pub fn get_largest_index(&self) -> usize {
        let mut i = 0;
        let mut largest = self.x;
        if self.y > largest {
            i = 1;
            largest = self.y;
        }
        if self.z > largest {
            i = 2;
            largest = self.z;
        }
        i
    }

    pub fn get_smallest_index(&self) -> usize {
        let mut i = 0;
        let mut smallest = self.x;
        if self.y < smallest {
            i = 1;
            smallest = self.y;
        }
        if self.z < smallest {
            i = 2;
            smallest = self.z;
        }
        i
    }

    pub fn distance(a: Vec3<S>, b: Vec3<S>) -> S {
        (b - a).length()
    }

    pub fn mix(a: Vec3<S>, b: Vec3<S>, t: Vec3<S>) -> Vec3<S> {
        (b - a) * (Vec3::one() - t)
    }

    pub fn mix_all(a: Vec3<S>, b: Vec3<S>, t: S) -> Vec3<S> {
        (b - a) * (S::one() - t)
    }

    pub fn clamp(a: Vec3<S>, min: Vec3<S>, max: Vec3<S>) -> Vec3<S> {
        Vec3 {
            x: num::clamp(a.x, min.x, max.x),
            y: num::clamp(a.y, min.y, max.y),
            z: num::clamp(a.z, min.z, max.z),
        }
    }

    pub fn clamp_all(a: Vec3<S>, min: S, max: S) -> Vec3<S> {
        Vec3 {
            x: num::clamp(a.x, min, max),
            y: num::clamp(a.y, min, max),
            z: num::clamp(a.z, min, max),
        }
    }
}


impl<S> Index<usize> for Vec3<S> where S: FloatType<S> {
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

impl<S> IndexMut<usize> for Vec3<S> where S: FloatType<S> {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Requested an invalid index on a Vec3: {}", index)
        }
    }
}

impl<S> Neg for Vec3<S> where S: FloatType<S> {
    type Output = Vec3<S>;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<S> Add<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    type Output = Vec3<S>;

    fn add(self, rhs: Vec3<S>) -> Vec3<S> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<S> AddAssign<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    fn add_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<S> Sub<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    type Output = Vec3<S>;

    fn sub(self, rhs: Vec3<S>) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<S> SubAssign<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    fn sub_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}


impl<S> Mul<S> for Vec3<S> where S: FloatType<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}


impl<S> MulAssign<S> for Vec3<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<S> Mul<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: Vec3<S>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<S> MulAssign<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}


impl<S> Div<S> for Vec3<S> where S: FloatType<S> {
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

impl<S> Div<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    type Output = Vec3<S>;

    fn div(self, rhs: Vec3<S>) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}


impl<S> DivAssign<S> for Vec3<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
        self.z = self.z * inv;
    }
}

impl<S> DivAssign<Vec3<S>> for Vec3<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: Vec3<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}


impl<S> PartialEq for Vec3<S> where S: FloatType<S> {
    fn eq(&self, other: &Vec3<S>) -> bool {
        self.x.approx_eq(other.x, S::DEF_EPSILON) & &
            self.y.approx_eq(other.y, S::DEF_EPSILON) & &
            self.z.approx_eq(other.z, S::DEF_EPSILON)
    }
}

impl<S> fmt::Display for Vec3<S> where S: FloatType<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3} {:.3} {:.3})", self.x, self.y, self.z)
    }
}

impl<S, U> From<(U, U, U)> for Vec3<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (U, U, U)) -> Vec3<S> {
        Vec3 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2).unwrap(),
        }
    }
}

impl<S, U> From<[U; 3]> for Vec3<S> where S: FloatType<S>, U: InputType {
    fn from(arr: [U; 3]) -> Vec3<S> {
        Vec3 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
            z: num::cast(arr[2]).unwrap(),
        }
    }
}

impl<S, U> From<Vec2<U>> for Vec3<S> where S: FloatType<S>, U: InputType {
    fn from(vec3: Vec2<U>) -> Vec3<S> {
        Vec3 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
            z: S::zero(),
        }
    }
}


impl<S, U> From<Vec4<U>> for Vec3<S> where S: FloatType<S>, U: InputType {
    fn from(vec4: Vec4<U>) -> Vec3<S> {
        Vec3 {
            x: num::cast(vec4.x).unwrap(),
            y: num::cast(vec4.y).unwrap(),
            z: num::cast(vec4.z).unwrap(),
        }
    }
}

impl<S, U> From<(U, Vec2<U>)> for Vec3<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (U, Vec2<U>)) -> Vec3<S> {
        Vec3 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1.x).unwrap(),
            z: num::cast(tuple.1.y).unwrap(),
        }
    }
}

impl<S, U> From<(Vec2<U>, U)> for Vec3<S> where S: FloatType<S>, U: InputType {
    fn from(tuple: (Vec2<U>, U)) -> Vec3<S> {
        Vec3 {
            x: num::cast(tuple.0.x).unwrap(),
            y: num::cast(tuple.0.y).unwrap(),
            z: num::cast(tuple.1).unwrap(),
        }
    }
}


impl<S> Default for Vec3<S> where S: FloatType<S> {
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

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for Vec3<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64F64F64
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}