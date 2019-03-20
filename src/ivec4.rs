use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IVec4 {
    pub x: Integer,
    pub y: Integer,
    pub z: Integer,
    pub w: Integer,
}

impl IVec4 {
    pub const ZERO: IVec4 = IVec4 { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: IVec4 = IVec4 { x: 1, y: 1, z: 1, w: 1 };
    pub const RIGHT: IVec4 = IVec4 { x: 1, y: 0, z: 0, w: 0 };
    pub const UP: IVec4 = IVec4 { x: 0, y: 1, z: 0, w: 0 };
    pub const FORWARD: IVec4 = IVec4 { x: 0, y: 0, z: 1, w: 0 };
    pub const W_ONLY: IVec4 = IVec4 { x: 0, y: 0, z: 0, w: 1 };

    pub fn new(x: Integer, y: Integer, z: Integer, w: Integer) -> IVec4 {
        IVec4 { x, y, z, w }
    }

    pub fn zero() -> IVec4 {
        IVec4 { x: 0, y: 0, z: 0, w: 0 }
    }

    pub fn one() -> IVec4 {
        IVec4 { x: 1, y: 1, z: 1, w: 1 }
    }

    pub fn all(t: Integer) -> IVec4 {
        IVec4 { x: t, y: t, z: t, w: t }
    }


    pub fn dot(lhs: &IVec4, rhs: &IVec4) -> Integer {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
    }

    pub fn length_squared(&self) -> Integer {
        IVec4::dot(self, self)
    }

    pub fn length(&self) -> Integer {
        (self.length_squared() as Real).sqrt() as Integer
    }

    pub fn size() -> usize {
        4
    }
}

impl Index<usize> for IVec4 {
    type Output = Integer;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Requested an invalid index on an IVec4: {}", index)
        }
    }
}

impl IndexMut<usize> for IVec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Requested an invalid index on an IVec4: {}", index)
        }
    }
}

impl Neg for IVec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add<IVec4> for IVec4 {
    type Output = Self;

    fn add(self, rhs: IVec4) -> Self::Output {
        Self::new(self.x + rhs.x,
                  self.y + rhs.y,
                  self.z + rhs.z,
                  self.w + rhs.w)
    }
}

impl AddAssign<IVec4> for IVec4 {
    fn add_assign(&mut self, rhs: IVec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub<IVec4> for IVec4 {
    type Output = Self;

    fn sub(self, rhs: IVec4) -> Self::Output {
        Self::new(self.x - rhs.x,
                  self.y - rhs.y,
                  self.z - rhs.z,
                  self.w - rhs.w)
    }
}

impl SubAssign<IVec4> for IVec4 {
    fn sub_assign(&mut self, rhs: IVec4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}


impl Mul<Integer> for IVec4 {
    type Output = Self;

    fn mul(self, rhs: Integer) -> Self::Output {
        Self::new(self.x * rhs,
                  self.y * rhs,
                  self.z * rhs,
                  self.w * rhs)
    }
}


impl MulAssign<Integer> for IVec4 {
    fn mul_assign(&mut self, rhs: Integer) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Mul<IVec4> for IVec4 {
    type Output = Self;

    fn mul(self, rhs: IVec4) -> Self::Output {
        Self::new(self.x * rhs.x,
                  self.y * rhs.y,
                  self.z * rhs.z,
                  self.w * rhs.w)
    }
}

impl MulAssign<IVec4> for IVec4 {
    fn mul_assign(&mut self, rhs: IVec4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}


impl PartialEq for IVec4 {
    fn eq(&self, other: &IVec4) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z) && (self.w == other.w)
    }
}

impl fmt::Display for IVec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {} {})", self.x, self.y, self.z, self.w)
    }
}

impl From<(Integer, Integer, Integer, Integer)> for IVec4 {
    fn from(tuple: (Integer, Integer, Integer, Integer)) -> Self {
        Self::new(tuple.0,
                  tuple.1,
                  tuple.2,
                  tuple.3)
    }
}

impl From<[Integer; 4]> for IVec4 {
    fn from(arr: [Integer; 4]) -> Self {
        Self::new(arr[0],
                  arr[1],
                  arr[2],
                  arr[3])
    }
}

impl From<IVec2> for IVec4 {
    fn from(vec2: IVec2) -> Self {
        Self::new(vec2.x,
                  vec2.y,
                  0,
                  0)
    }
}

impl From<IVec3> for IVec4 {
    fn from(vec3: IVec3) -> Self {
        Self::new(
            vec3.x,
            vec3.y,
            vec3.z,
            0)
    }
}

impl Default for IVec4 {
    fn default() -> Self {
        IVec4::zero()
    }
}

impl glium::uniforms::AsUniformValue for IVec4 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::IntVec4(std::mem::transmute::<Self, [Integer; 4]>(*self))
        }
    }
}

unsafe impl glium::vertex::Attribute for IVec4 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32I32I32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}