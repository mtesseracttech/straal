use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vec3 {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn up() -> Vec3 {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn right() -> Vec3 {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn forward() -> Vec3 {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> Scalar {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Self {
        Self::new(lhs.y * rhs.z - lhs.z * rhs.y,
                  lhs.z * rhs.x - lhs.x * rhs.z,
                  lhs.x * rhs.y - lhs.y * rhs.x)
    }

    pub fn length_squared(&self) -> Scalar {
        Vec3::dot(self, self)
    }

    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared() == 1.0
    }

    pub fn size() -> usize {
        3
    }

    pub fn normalized(&self) -> Vec3 {
        let scale = 1.0 / self.length();
        Self::new(self.x * scale, self.y * scale, self.z * scale)
    }

    pub fn normalize(&mut self) {
        let scale = 1.0 / self.length();
        self.x *= scale;
        self.y *= scale;
    }
}

impl Index<usize> for Vec3 {
    type Output = Scalar;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Requested an invalid index on a Vec3: {}", index)
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Requested an invalid index on a Vec3: {}", index)
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y, -self.z) }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output { Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z) }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output { Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z) }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}


impl Mul<Scalar> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output { Self::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}


impl MulAssign<Scalar> for Vec3 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}


impl Div<Scalar> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Scalar) -> Self::Output {
        let inv = 1.0 / rhs;
        Self::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

impl DivAssign<Scalar> for Vec3 {
    fn div_assign(&mut self, rhs: Scalar) {
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self::Output { Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z) }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}


impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x.approx_eq(other.x, DEF_F32_EPSILON) && self.y.approx_eq(other.y, DEF_F32_EPSILON) && self.z.approx_eq(other.z, DEF_F32_EPSILON)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({:.2} {:.2} {:.2})", self.x, self.y, self.z) }
}

impl From<(Scalar, Scalar, Scalar)> for Vec3 {
    fn from(tuple: (Scalar, Scalar, Scalar)) -> Self { Self::new(tuple.0, tuple.1, tuple.2) }
}

impl From<[f32; 3]> for Vec3 {
    fn from(arr: [f32; 3]) -> Self { Self::new(arr[0], arr[1], arr[2]) }
}

impl From<Vec2> for Vec3 {
    fn from(vec2: Vec2) -> Self { Self::new(vec2.x, vec2.y, 0.0) }
}

impl From<(Scalar, Vec2)> for Vec3 {
    fn from(other: (Scalar, Vec2)) -> Self {
        Self::new(other.0, other.1.x, other.1.y)
    }
}

impl From<(Vec2, Scalar)> for Vec3 {
    fn from(other: (Vec2, Scalar)) -> Self {
        Self::new(other.0.x, other.0.y, other.1)
    }
}


impl From<Vec4> for Vec3 {
    fn from(vec4: Vec4) -> Self { Self::new(vec4.x, vec4.y, vec4.z) }
}

impl glium::uniforms::AsUniformValue for Vec3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe { glium::uniforms::UniformValue::Vec3(std::mem::transmute::<Self, [f32; 3]>(*self)) }
    }
}
