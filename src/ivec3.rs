use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IVec3 {
    pub x: Integer,
    pub y: Integer,
    pub z: Integer,
}

impl IVec3 {
    pub fn new(x: Integer, y: Integer, z: Integer) -> IVec3 {
        IVec3 { x, y, z }
    }

    pub fn zero() -> IVec3 {
        IVec3 {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn dot(lhs: &IVec3, rhs: &IVec3) -> Integer {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn length_squared(&self) -> Integer {
        IVec3::dot(self, self)
    }

    pub fn length(&self) -> Integer {
        (self.length_squared() as Real).sqrt() as Integer
    }

    pub fn size() -> usize {
        3
    }
}

impl Index<usize> for IVec3 {
    type Output = Integer;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Requested an invalid index on an IVec3: {}", index)
        }
    }
}

impl IndexMut<usize> for IVec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Requested an invalid index on an IVec3: {}", index)
        }
    }
}

impl Neg for IVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Add<IVec3> for IVec3 {
    type Output = Self;

    fn add(self, rhs: IVec3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<IVec3> for IVec3 {
    fn add_assign(&mut self, rhs: IVec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<IVec3> for IVec3 {
    type Output = Self;

    fn sub(self, rhs: IVec3) -> Self::Output {
        Self::new(self.x - rhs.x,
                  self.y - rhs.y,
                  self.z - rhs.z)
    }
}

impl SubAssign<IVec3> for IVec3 {
    fn sub_assign(&mut self, rhs: IVec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}


impl Mul<Integer> for IVec3 {
    type Output = Self;

    fn mul(self, rhs: Integer) -> Self::Output {
        Self::new(self.x * rhs,
                  self.y * rhs,
                  self.z * rhs)
    }
}


impl MulAssign<Integer> for IVec3 {
    fn mul_assign(&mut self, rhs: Integer) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<IVec3> for IVec3 {
    type Output = Self;

    fn mul(self, rhs: IVec3) -> Self::Output {
        Self::new(self.x * rhs.x,
                  self.y * rhs.y,
                  self.z * rhs.z)
    }
}

impl MulAssign<IVec3> for IVec3 {
    fn mul_assign(&mut self, rhs: IVec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}


impl PartialEq for IVec3 {
    fn eq(&self, other: &IVec3) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

impl fmt::Display for IVec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}

impl From<(Integer, Integer, Integer, Integer)> for IVec3 {
    fn from(tuple: (Integer, Integer, Integer, Integer)) -> Self {
        Self::new(tuple.0,
                  tuple.1,
                  tuple.2)
    }
}

impl From<[Integer; 3]> for IVec3 {
    fn from(arr: [Integer; 3]) -> Self {
        Self::new(arr[0],
                  arr[1],
                  arr[2])
    }
}

impl From<IVec2> for IVec3 {
    fn from(vec2: IVec2) -> Self {
        Self::new(vec2.x,
                  vec2.y,
                  0)
    }
}

impl From<IVec4> for IVec3 {
    fn from(vec4: IVec4) -> Self {
        Self::new(vec4.x,
                  vec4.y,
                  vec4.z)
    }
}

impl Default for IVec3 {
    fn default() -> Self {
        IVec3::zero()
    }
}

impl glium::uniforms::AsUniformValue for IVec3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::IntVec3(std::mem::transmute::<Self, [Integer; 3]>(*self))
        }
    }
}

unsafe impl glium::vertex::Attribute for IVec3 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32I32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}