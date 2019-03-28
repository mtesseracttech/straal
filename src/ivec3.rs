use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct IVec3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S> IVec3<S> where S: IntegerType {
    pub fn zero() -> IVec3<S> {
        IVec3 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
        }
    }

    pub fn one() -> IVec3<S> {
        IVec3 {
            x: S::one(),
            y: S::one(),
            z: S::one(),
        }
    }

    pub fn right() -> IVec3<S> {
        IVec3 {
            x: S::one(),
            y: S::zero(),
            z: S::zero(),
        }
    }

    pub fn up() -> IVec3<S> {
        IVec3 {
            x: S::zero(),
            y: S::one(),
            z: S::zero(),
        }
    }

    pub fn forward() -> IVec3<S> {
        IVec3 {
            x: S::zero(),
            y: S::zero(),
            z: S::one(),
        }
    }

    pub fn new<U>(x: U, y: U, z: U) -> IVec3<S> where U: InputType {
        IVec3 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
            z: num::cast(z).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> IVec3<S> where U: InputType {
        let t = num::cast(t).unwrap();
        IVec3 {
            x: t,
            y: t,
            z: t,
        }
    }

    pub fn dot(self, rhs: IVec3<S>) -> S {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: IVec3<S>) -> IVec3<S> {
        IVec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length_squared(self) -> S {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().to_f64().unwrap().sqrt()
    }

    pub fn is_unit(&self) -> bool {
        self.length_squared() == S::one()
    }

    pub fn size() -> usize {
        3
    }
}


impl<S> Index<usize> for IVec3<S> where S: IntegerType {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Requested an invalid index on a IVec3: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for IVec3<S> where S: IntegerType {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Requested an invalid index on a IVec3: {}", index)
        }
    }
}

impl<S> Neg for IVec3<S> where S: IntegerType {
    type Output = IVec3<S>;

    fn neg(self) -> Self::Output {
        IVec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<S> Add<IVec3<S>> for IVec3<S> where S: IntegerType {
    type Output = IVec3<S>;

    fn add(self, rhs: IVec3<S>) -> IVec3<S> {
        IVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<S> AddAssign<IVec3<S>> for IVec3<S> where S: IntegerType {
    fn add_assign(&mut self, rhs: IVec3<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<S> Sub<IVec3<S>> for IVec3<S> where S: IntegerType {
    type Output = IVec3<S>;

    fn sub(self, rhs: IVec3<S>) -> Self::Output {
        IVec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<S> SubAssign<IVec3<S>> for IVec3<S> where S: IntegerType {
    fn sub_assign(&mut self, rhs: IVec3<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<S> Mul<S> for IVec3<S> where S: IntegerType {
    type Output = IVec3<S>;

    fn mul(self, rhs: S) -> Self::Output {
        IVec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<S> MulAssign<S> for IVec3<S> where S: IntegerType {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<S> Mul<IVec3<S>> for IVec3<S> where S: IntegerType {
    type Output = IVec3<S>;

    fn mul(self, rhs: IVec3<S>) -> Self::Output {
        IVec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<S> MulAssign<IVec3<S>> for IVec3<S> where S: IntegerType {
    fn mul_assign(&mut self, rhs: IVec3<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}


impl<S> Div<S> for IVec3<S> where S: IntegerType {
    type Output = IVec3<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        IVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        } * inv
    }
}

impl<S> Div<IVec3<S>> for IVec3<S> where S: IntegerType {
    type Output = IVec3<S>;

    fn div(self, rhs: IVec3<S>) -> Self::Output {
        IVec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}


impl<S> DivAssign<S> for IVec3<S> where S: IntegerType {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
        self.z = self.z * inv;
    }
}

impl<S> DivAssign<IVec3<S>> for IVec3<S> where S: IntegerType {
    fn div_assign(&mut self, rhs: IVec3<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}


impl<S> PartialEq for IVec3<S> where S: IntegerType {
    fn eq(&self, other: &IVec3<S>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<S> fmt::Display for IVec3<S> where S: IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}


impl<S, U> From<(U, U, U)> for IVec3<S> where S: IntegerType, U: InputType {
    fn from(tuple: (U, U, U)) -> IVec3<S> {
        IVec3 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2).unwrap(),
        }
    }
}

impl<S, U> From<[U; 3]> for IVec3<S> where S: IntegerType, U: InputType {
    fn from(arr: [U; 3]) -> IVec3<S> {
        IVec3 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
            z: num::cast(arr[2]).unwrap(),
        }
    }
}


impl<S, U> From<IVec2<U>> for IVec3<S> where S: IntegerType, U: InputType {
    fn from(vec2: IVec2<U>) -> IVec3<S> {
        IVec3 {
            x: num::cast(vec2.x).unwrap(),
            y: num::cast(vec2.y).unwrap(),
            z: S::zero(),
        }
    }
}

impl<S, U> From<IVec4<U>> for IVec3<S> where S: IntegerType, U: InputType {
    fn from(vec4: IVec4<U>) -> IVec3<S> {
        IVec3 {
            x: num::cast(vec4.x).unwrap(),
            y: num::cast(vec4.y).unwrap(),
            z: num::cast(vec4.z).unwrap(),
        }
    }
}


impl<S> Default for IVec3<S> where S: IntegerType {
    fn default() -> IVec3<S> {
        IVec3::zero()
    }
}


impl glium::uniforms::AsUniformValue for IVec3<i32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::IntVec3(std::mem::transmute::<IVec3<i32>, [i32; 3]>(*self))
        }
    }
}

impl glium::uniforms::AsUniformValue for IVec3<i64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Int64Vec3(std::mem::transmute::<IVec3<i64>, [i64; 3]>(*self))
        }
    }
}

unsafe impl glium::vertex::Attribute for IVec3<i32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32I32
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for IVec3<i64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I64I64I64
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}