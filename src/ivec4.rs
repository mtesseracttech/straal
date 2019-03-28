use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct IVec4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

impl<S> IVec4<S> where S: IntegerType {
    pub fn zero() -> IVec4<S> {
        IVec4 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
            w: S::zero(),
        }
    }

    pub fn one() -> IVec4<S> {
        IVec4 {
            x: S::one(),
            y: S::one(),
            z: S::one(),
            w: S::one(),
        }
    }

    pub fn right() -> IVec4<S> {
        IVec4 {
            x: S::one(),
            y: S::zero(),
            z: S::zero(),
            w: S::zero(),
        }
    }

    pub fn up() -> IVec4<S> {
        IVec4 {
            x: S::zero(),
            y: S::one(),
            z: S::zero(),
            w: S::zero(),
        }
    }

    pub fn forward() -> IVec4<S> {
        IVec4 {
            x: S::zero(),
            y: S::zero(),
            z: S::one(),
            w: S::zero(),
        }
    }

    pub fn w_only() -> IVec4<S> {
        IVec4 {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
            w: S::one(),
        }
    }

    pub fn new<U>(x: U, y: U, z: U, w: U) -> IVec4<S> where U: InputType {
        IVec4 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
            z: num::cast(z).unwrap(),
            w: num::cast(w).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> IVec4<S> where U: InputType {
        let t = num::cast(t).unwrap();
        IVec4 {
            x: t,
            y: t,
            z: t,
            w: t,
        }
    }

    pub fn dot(self, rhs: IVec4<S>) -> S {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
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
        4
    }
}


impl<S> Index<usize> for IVec4<S> where S: IntegerType {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Requested an invalid index on a IVec4: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for IVec4<S> where S: IntegerType {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Requested an invalid index on a IVec4: {}", index)
        }
    }
}

impl<S> Neg for IVec4<S> where S: IntegerType {
    type Output = IVec4<S>;

    fn neg(self) -> Self::Output {
        IVec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<S> Add<IVec4<S>> for IVec4<S> where S: IntegerType {
    type Output = IVec4<S>;

    fn add(self, rhs: IVec4<S>) -> IVec4<S> {
        IVec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<S> AddAssign<IVec4<S>> for IVec4<S> where S: IntegerType {
    fn add_assign(&mut self, rhs: IVec4<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

impl<S> Sub<IVec4<S>> for IVec4<S> where S: IntegerType {
    type Output = IVec4<S>;

    fn sub(self, rhs: IVec4<S>) -> Self::Output {
        IVec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<S> SubAssign<IVec4<S>> for IVec4<S> where S: IntegerType {
    fn sub_assign(&mut self, rhs: IVec4<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<S> Mul<S> for IVec4<S> where S: IntegerType {
    type Output = IVec4<S>;

    fn mul(self, rhs: S) -> Self::Output {
        IVec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<S> MulAssign<S> for IVec4<S> where S: IntegerType {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

impl<S> Mul<IVec4<S>> for IVec4<S> where S: IntegerType {
    type Output = IVec4<S>;

    fn mul(self, rhs: IVec4<S>) -> Self::Output {
        IVec4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<S> MulAssign<IVec4<S>> for IVec4<S> where S: IntegerType {
    fn mul_assign(&mut self, rhs: IVec4<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self.w = self.w * rhs.w;
    }
}


impl<S> Div<S> for IVec4<S> where S: IntegerType {
    type Output = IVec4<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        IVec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        } * inv
    }
}

impl<S> Div<IVec4<S>> for IVec4<S> where S: IntegerType {
    type Output = IVec4<S>;

    fn div(self, rhs: IVec4<S>) -> Self::Output {
        IVec4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}


impl<S> DivAssign<S> for IVec4<S> where S: IntegerType {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
        self.z = self.z * inv;
        self.w = self.w * inv;
    }
}

impl<S> DivAssign<IVec4<S>> for IVec4<S> where S: IntegerType {
    fn div_assign(&mut self, rhs: IVec4<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self.w = self.w / rhs.w;
    }
}


impl<S> PartialEq for IVec4<S> where S: IntegerType {
    fn eq(&self, other: &IVec4<S>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<S> fmt::Display for IVec4<S> where S: IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {} {})", self.x, self.y, self.z, self.w)
    }
}


impl<S, U> From<(U, U, U, U)> for IVec4<S> where S: IntegerType, U: InputType {
    fn from(tuple: (U, U, U, U)) -> IVec4<S> {
        IVec4 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
            z: num::cast(tuple.2).unwrap(),
            w: num::cast(tuple.3).unwrap(),
        }
    }
}

impl<S, U> From<[U; 4]> for IVec4<S> where S: IntegerType, U: InputType {
    fn from(arr: [U; 4]) -> IVec4<S> {
        IVec4 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
            z: num::cast(arr[2]).unwrap(),
            w: num::cast(arr[3]).unwrap(),
        }
    }
}

impl<S, U> From<IVec2<U>> for IVec4<S> where S: IntegerType, U: InputType {
    fn from(vec2: IVec2<U>) -> IVec4<S> {
        IVec4 {
            x: num::cast(vec2.x).unwrap(),
            y: num::cast(vec2.y).unwrap(),
            z: S::zero(),
            w: S::zero(),
        }
    }
}

impl<S, U> From<IVec3<U>> for IVec4<S> where S: IntegerType, U: InputType {
    fn from(vec3: IVec3<U>) -> IVec4<S> {
        IVec4 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
            z: num::cast(vec3.z).unwrap(),
            w: S::zero(),
        }
    }
}

impl<S> Default for IVec4<S> where S: IntegerType {
    fn default() -> IVec4<S> {
        IVec4::zero()
    }
}

impl glium::uniforms::AsUniformValue for IVec4<i32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::IntVec4(std::mem::transmute::<IVec4<i32>, [i32; 4]>(*self))
        }
    }
}

impl glium::uniforms::AsUniformValue for IVec4<i64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Int64Vec4(std::mem::transmute::<IVec4<i64>, [i64; 4]>(*self))
        }
    }
}

unsafe impl glium::vertex::Attribute for IVec4<i32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32I32I32
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for IVec4<i64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I64I64I64I64
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}