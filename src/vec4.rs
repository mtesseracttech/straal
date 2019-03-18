use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub w: Real,
}

impl Vec4 {
    //Build a new vector4 from 4 scalar (floating point) components
    pub fn new(x: Real, y: Real, z: Real, w: Real) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    pub fn zero() -> Vec4 {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn dot(lhs: &Vec4, rhs: &Vec4) -> Real {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }


    pub fn length_squared(&self) -> Real {
        Vec4::dot(self, self)
    }

    //Returns the euclidean distance of the vector
    pub fn length(&self) -> Real {
        self.length_squared().sqrt()
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared().approx_eq(1.0, DEF_F32_EPSILON)
    }


    pub fn size() -> usize {
        4
    }

    pub fn normalized(&self) -> Vec4 {
        let scale = 1.0 / self.length();
        Self::new(self.x * scale, self.y * scale, self.z * scale, self.w * scale)
    }

    pub fn normalize(&mut self) {
        let scale = 1.0 / self.length();
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
        self.w *= scale;
    }
}

impl Index<usize> for Vec4 {
    type Output = Real;
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

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Requested an invalid index on a Vec4: {}", index)
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self::new(self.x + rhs.x,
                  self.y + rhs.y,
                  self.z + rhs.z,
                  self.w + rhs.w)
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Self::new(self.x - rhs.x,
                  self.y - rhs.y,
                  self.z - rhs.z,
                  self.w - rhs.w)
    }
}

impl SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}


impl Mul<Real> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        Self::new(self.x * rhs,
                  self.y * rhs,
                  self.z * rhs,
                  self.w * rhs)
    }
}


impl MulAssign<Real> for Vec4 {
    fn mul_assign(&mut self, rhs: Real) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::new(self.x * rhs.x,
                  self.y * rhs.y,
                  self.z * rhs.z,
                  self.w * rhs.w)
    }
}

impl MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, rhs: Vec4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}


impl Div<Real> for Vec4 {
    type Output = Self;

    fn div(self, rhs: Real) -> Self::Output {
        let inv = 1.0 / rhs;
        Self::new(self.x * inv,
                  self.y * inv,
                  self.z * inv,
                  self.w * inv)
    }
}

impl DivAssign<Real> for Vec4 {
    fn div_assign(&mut self, rhs: Real) {
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
        self.w *= inv;
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Self;

    fn div(self, rhs: Vec4) -> Self::Output {
        Self::new(self.x / rhs.x,
                  self.y / rhs.y,
                  self.z / rhs.z,
                  self.w / rhs.w)
    }
}

impl DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, rhs: Vec4) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}


impl PartialEq for Vec4 {
    fn eq(&self, other: &Vec4) -> bool {
        self.x.approx_eq(other.x, DEF_F32_EPSILON) && self.y.approx_eq(other.y, DEF_F32_EPSILON) && self.z.approx_eq(other.z, DEF_F32_EPSILON) && self.w.approx_eq(other.w, DEF_F32_EPSILON)
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2} {:.2} {:.2} {:.2})", self.x, self.y, self.z, self.w)
    }
}

impl From<(Real, Real, Real, Real)> for Vec4 {
    fn from(tuple: (Real, Real, Real, Real)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl From<[Real; 4]> for Vec4 {
    fn from(arr: [Real; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<Vec3> for Vec4 {
    fn from(vec3: Vec3) -> Self {
        Self::new(vec3.x, vec3.y, vec3.z, 0.0)
    }
}

impl From<(Vec3, Real)> for Vec4 {
    fn from(other: (Vec3, Real)) -> Self {
        Self::new(other.0.x, other.0.y, other.0.z, other.1)
    }
}

impl From<(Real, Vec3)> for Vec4 {
    fn from(other: (Real, Vec3)) -> Self {
        Self::new(other.0, other.1.x, other.1.y, other.1.z)
    }
}

impl From<Vec2> for Vec4 {
    fn from(vec2: Vec2) -> Self {
        Self::new(vec2.x, vec2.y, 0.0, 0.0)
    }
}

impl From<(Vec2, Vec2)> for Vec4 {
    fn from(other: (Vec2, Vec2)) -> Self {
        Self::new(other.0.x, other.0.y, other.1.x, other.1.y)
    }
}

impl From<(Real, Real, Vec2)> for Vec4 {
    fn from(other: (Real, Real, Vec2)) -> Self {
        Self::new(other.0, other.1, other.2.x, other.2.y)
    }
}

impl From<(Vec2, Real, Real)> for Vec4 {
    fn from(other: (Vec2, Real, Real)) -> Self {
        Self::new(other.0.x, other.0.y, other.1, other.2)
    }
}

impl From<(Real, Vec2, Real)> for Vec4 {
    fn from(other: (Real, Vec2, Real)) -> Self {
        Self::new(other.0, other.1.x, other.1.y, other.2)
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4::zero()
    }
}

impl glium::uniforms::AsUniformValue for Vec4 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Vec4(std::mem::transmute::<Self, [f32; 4]>(*self))
        }
    }
}

unsafe impl glium::vertex::Attribute for Vec4 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32F32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}