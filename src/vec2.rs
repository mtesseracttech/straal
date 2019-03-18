use std::borrow::Cow;
use std::fmt;
use std::ops::*;

use glium::CapabilitiesSource;
use glium::vertex::AttributeType;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: Real,
    pub y: Real,
}

impl Vec2 {
    pub fn new(x: Real, y: Real) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Vec2 {
        Vec2 { x: 1.0, y: 1.0 }
    }

    pub fn all(t: Real) -> Vec2 {
        Vec2 { x: t, y: t }
    }

    pub fn right() -> Vec2 {
        Self::new(1.0, 0.0)
    }

    pub fn up() -> Vec2 {
        Self::new(0.0, 1.0)
    }

    pub fn dot(lhs: &Vec2, rhs: &Vec2) -> Real {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    pub fn length_squared(&self) -> Real {
        Vec2::dot(self, self)
    }

    pub fn length(&self) -> Real {
        self.length_squared().sqrt()
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared().approx_eq(1.0, DEF_F32_EPSILON)
    }

    pub fn size() -> usize {
        2
    }

    pub fn normalized(&self) -> Vec2 {
        let scale = 1.0 / self.length();
        Self::new(self.x * scale, self.y * scale)
    }

    pub fn normalize(&mut self) {
        let scale = 1.0 / self.length();
        self.x *= scale;
        self.y *= scale;
    }
}

impl Index<usize> for Vec2 {
    type Output = Real;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index)
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Requested an invalid index on a Vec2: {}", index)
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y) }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output { Self::new(self.x + rhs.x, self.y + rhs.y) }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output { Self::new(self.x - rhs.x, self.y - rhs.y) }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}


impl Mul<Real> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output { Self::new(self.x * rhs, self.y * rhs) }
}


impl MulAssign<Real> for Vec2 {
    fn mul_assign(&mut self, rhs: Real) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}


impl Div<Real> for Vec2 {
    type Output = Self;

    fn div(self, rhs: Real) -> Self::Output {
        let inv = 1.0 / rhs;
        Self::new(self.x * inv, self.y * inv)
    }
}

impl DivAssign<Real> for Vec2 {
    fn div_assign(&mut self, rhs: Real) {
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
    }
}

impl DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;

    fn div(self, rhs: Vec2) -> Self::Output { Self::new(self.x / rhs.x, self.y / rhs.y) }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x.approx_eq(other.x, DEF_F32_EPSILON) && self.y.approx_eq(other.y, DEF_F32_EPSILON)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({:.2} {:.2})", self.x, self.y) }
}

impl From<(u32, u32)> for Vec2 {
    fn from(sizes: (u32, u32)) -> Self {
        Self::new(sizes.0 as Real, sizes.1 as Real)
    }
}

impl From<(Integer, Integer)> for Vec2 {
    fn from(ints: (i32, i32)) -> Self {
        Self::new(ints.0 as Real, ints.1 as Real)
    }
}

impl From<(Real, Real)> for Vec2 {
    fn from(tuple: (Real, Real)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(arr: [f32; 2]) -> Self { Self::new(arr[0], arr[1]) }
}

impl From<Vec3> for Vec2 {
    fn from(vec3: Vec3) -> Self { Self::new(vec3.x, vec3.y) }
}

impl From<Vec4> for Vec2 {
    fn from(vec4: Vec4) -> Self { Self::new(vec4.x, vec4.y) }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::zero()
    }
}

impl glium::uniforms::AsUniformValue for Vec2 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe { glium::uniforms::UniformValue::Vec2(std::mem::transmute::<Self, [f32; 2]>(*self)) }
    }
}

unsafe impl glium::vertex::Attribute for Vec2 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}