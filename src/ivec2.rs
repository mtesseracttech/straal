use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IVec2 {
    pub x: Integer,
    pub y: Integer,
}

impl IVec2 {
    pub const ZERO: IVec2 = IVec2 { x: 0, y: 0 };
    pub const ONE: IVec2 = IVec2 { x: 1, y: 1 };

    pub fn new(x: Integer, y: Integer) -> IVec2 {
        IVec2 { x, y }
    }

    pub fn all(t: Integer) -> IVec2 {
        IVec2 { x: t, y: t }
    }

    pub fn dot(lhs: &IVec2, rhs: &IVec2) -> Integer {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    pub fn length_squared(&self) -> Integer {
        IVec2::dot(self, self)
    }

    pub fn length(&self) -> Integer {
        (self.length_squared() as Real).sqrt() as Integer
    }

    pub fn size() -> usize {
        2
    }
}

impl Index<usize> for IVec2 {
    type Output = Integer;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Requested an invalid index on an IVec2: {}", index)
        }
    }
}

impl IndexMut<usize> for IVec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Requested an invalid index on an IVec2: {}", index)
        }
    }
}

impl Neg for IVec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x,
                  -self.y)
    }
}

impl Add<IVec2> for IVec2 {
    type Output = Self;

    fn add(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x + rhs.x,
                  self.y + rhs.y)
    }
}

impl AddAssign<IVec2> for IVec2 {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<IVec2> for IVec2 {
    type Output = Self;

    fn sub(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x - rhs.x,
                  self.y - rhs.y)
    }
}

impl SubAssign<IVec2> for IVec2 {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}


impl Mul<Integer> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: Integer) -> Self::Output {
        Self::new(self.x * rhs,
                  self.y * rhs)
    }
}


impl MulAssign<Integer> for IVec2 {
    fn mul_assign(&mut self, rhs: Integer) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<IVec2> for IVec2 {
    type Output = Self;

    fn mul(self, rhs: IVec2) -> Self::Output {
        Self::new(self.x * rhs.x,
                  self.y * rhs.y)
    }
}

impl MulAssign<IVec2> for IVec2 {
    fn mul_assign(&mut self, rhs: IVec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}


impl PartialEq for IVec2 {
    fn eq(&self, other: &IVec2) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl fmt::Display for IVec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

impl From<(Integer, Integer, Integer, Integer)> for IVec2 {
    fn from(tuple: (Integer, Integer, Integer, Integer)) -> Self {
        Self::new(tuple.0,
                  tuple.1)
    }
}

impl From<[Integer; 2]> for IVec2 {
    fn from(arr: [Integer; 2]) -> Self {
        Self::new(arr[0],
                  arr[1])
    }
}

impl From<IVec3> for IVec2 {
    fn from(vec3: IVec3) -> Self {
        Self::new(vec3.x,
                  vec3.y)
    }
}

impl From<IVec4> for IVec2 {
    fn from(vec4: IVec4) -> Self {
        Self::new(vec4.x,
                  vec4.y)
    }
}

impl Default for IVec2 {
    fn default() -> Self {
        IVec2::zero()
    }
}

impl glium::uniforms::AsUniformValue for IVec2 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::IntVec2(std::mem::transmute::<Self, [Integer; 2]>(*self))
        }
    }
}

unsafe impl glium::vertex::Attribute for IVec2 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}