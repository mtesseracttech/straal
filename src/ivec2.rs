use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IVec2<S> {
    pub x: S,
    pub y: S,
}

impl<S> IVec2<S> where S: IntegerType {
    pub fn zero() -> IVec2<S> {
        IVec2 {
            x: S::zero(),
            y: S::zero(),
        }
    }

    pub fn one() -> IVec2<S> {
        IVec2 {
            x: S::one(),
            y: S::one(),
        }
    }

    pub fn right() -> IVec2<S> {
        IVec2 {
            x: S::one(),
            y: S::zero(),
        }
    }

    pub fn up() -> IVec2<S> {
        IVec2 {
            x: S::zero(),
            y: S::one(),
        }
    }

    pub fn new<U>(x: U, y: U) -> IVec2<S> where U: InputType {
        IVec2 {
            x: num::cast(x).unwrap(),
            y: num::cast(y).unwrap(),
        }
    }

    pub fn all<U>(t: U) -> IVec2<S> where U: InputType {
        let t = num::cast(t).unwrap();
        IVec2 {
            x: t,
            y: t,
        }
    }

    pub fn dot(self, rhs: IVec2<S>) -> S {
        self.x * rhs.x + self.y * rhs.y
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
        2
    }
}

impl<S> Index<usize> for IVec2<S> where S: IntegerType {
    type Output = S;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Requested an invalid index on a IVec2: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for IVec2<S> where S: IntegerType {
    fn index_mut(&mut self, index: usize) -> &mut S {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Requested an invalid index on a IVec2: {}", index)
        }
    }
}

impl<S> Neg for IVec2<S> where S: IntegerType {
    type Output = IVec2<S>;

    fn neg(self) -> Self::Output {
        IVec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<S> Add<IVec2<S>> for IVec2<S> where S: IntegerType {
    type Output = IVec2<S>;

    fn add(self, rhs: IVec2<S>) -> IVec2<S> {
        IVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<S> AddAssign<IVec2<S>> for IVec2<S> where S: IntegerType {
    fn add_assign(&mut self, rhs: IVec2<S>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<S> Sub<IVec2<S>> for IVec2<S> where S: IntegerType {
    type Output = IVec2<S>;

    fn sub(self, rhs: IVec2<S>) -> Self::Output {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<S> SubAssign<IVec2<S>> for IVec2<S> where S: IntegerType {
    fn sub_assign(&mut self, rhs: IVec2<S>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<S> Mul<S> for IVec2<S> where S: IntegerType {
    type Output = IVec2<S>;

    fn mul(self, rhs: S) -> Self::Output {
        IVec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<S> MulAssign<S> for IVec2<S> where S: IntegerType {
    fn mul_assign(&mut self, rhs: S) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<S> Mul<IVec2<S>> for IVec2<S> where S: IntegerType {
    type Output = IVec2<S>;

    fn mul(self, rhs: IVec2<S>) -> Self::Output {
        IVec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<S> MulAssign<IVec2<S>> for IVec2<S> where S: IntegerType {
    fn mul_assign(&mut self, rhs: IVec2<S>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}


impl<S> Div<S> for IVec2<S> where S: IntegerType {
    type Output = IVec2<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        IVec2 {
            x: self.x,
            y: self.y,
        } * inv
    }
}

impl<S> Div<IVec2<S>> for IVec2<S> where S: IntegerType {
    type Output = IVec2<S>;

    fn div(self, rhs: IVec2<S>) -> Self::Output {
        IVec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}


impl<S> DivAssign<S> for IVec2<S> where S: IntegerType {
    fn div_assign(&mut self, rhs: S) {
        let inv = S::one() / rhs;
        self.x = self.x * inv;
        self.y = self.y * inv;
    }
}

impl<S> DivAssign<IVec2<S>> for IVec2<S> where S: IntegerType {
    fn div_assign(&mut self, rhs: IVec2<S>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}


impl<S> PartialEq for IVec2<S> where S: IntegerType {
    fn eq(&self, other: &IVec2<S>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<S> fmt::Display for IVec2<S> where S: IntegerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}


impl<S, U> From<(U, U)> for IVec2<S> where S: IntegerType, U: InputType {
    fn from(tuple: (U, U)) -> IVec2<S> {
        IVec2 {
            x: num::cast(tuple.0).unwrap(),
            y: num::cast(tuple.1).unwrap(),
        }
    }
}

impl<S, U> From<[U; 2]> for IVec2<S> where S: IntegerType, U: InputType {
    fn from(arr: [U; 2]) -> IVec2<S> {
        IVec2 {
            x: num::cast(arr[0]).unwrap(),
            y: num::cast(arr[1]).unwrap(),
        }
    }
}

impl<S, U> From<IVec3<U>> for IVec2<S> where S: IntegerType, U: InputType {
    fn from(vec3: IVec3<U>) -> IVec2<S> {
        IVec2 {
            x: num::cast(vec3.x).unwrap(),
            y: num::cast(vec3.y).unwrap(),
        }
    }
}

impl<S, U> From<IVec4<U>> for IVec2<S> where S: IntegerType, U: InputType {
    fn from(vec4: IVec4<U>) -> IVec2<S> {
        IVec2 {
            x: num::cast(vec4.x).unwrap(),
            y: num::cast(vec4.y).unwrap(),
        }
    }
}

impl<S> Default for IVec2<S> where S: IntegerType {
    fn default() -> IVec2<S> {
        IVec2::zero()
    }
}

impl glium::uniforms::AsUniformValue for IVec2<i32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::IntVec2(std::mem::transmute::<IVec2<i32>, [i32; 2]>(*self))
        }
    }
}

impl glium::uniforms::AsUniformValue for IVec2<i64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Int64Vec2(std::mem::transmute::<IVec2<i64>, [i64; 2]>(*self))
        }
    }
}

unsafe impl glium::vertex::Attribute for Vec2<i32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I32I32
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}


unsafe impl glium::vertex::Attribute for Vec2<i64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::I64I64
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}