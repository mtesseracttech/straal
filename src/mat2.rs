use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat2<S> {
    pub r0: Vec2<S>,
    pub r1: Vec2<S>,
}


impl<S> Mat2<S> where S: FloatType<S> {
    pub fn identity() -> Mat2<S> {
        Mat2 {
            r0: Vec2 { x: S::one(), y: S::zero() },
            r1: Vec2 { x: S::zero(), y: S::one() },
        }
    }

    pub fn empty() -> Mat2<S> {
        Mat2 {
            r0: Vec2::zero(),
            r1: Vec2::zero(),
        }
    }

    pub fn new<U>(r0c0: U, r0c1: U,
                  r1c0: U, r1c1: U) -> Mat2<S> where U: InputType {
        Mat2 {
            r0: Vec2 { x: num::cast(r0c0).unwrap(), y: num::cast(r0c1).unwrap() },
            r1: Vec2 { x: num::cast(r1c0).unwrap(), y: num::cast(r1c1).unwrap() },
        }
    }

    pub fn new_from_vec2s(r0: Vec2<S>, r1: Vec2<S>) -> Mat2<S> {
        Mat2 { r0, r1 }
    }

    pub fn new_from_arrs(r0: [S; 2], r1: [S; 2]) -> Mat2<S> {
        Mat2 {
            r0: Vec2::from(r0),
            r1: Vec2::from(r1),
        }
    }

    pub fn determinant(&self) -> S {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pub fn adjoint(&self) -> Mat2<S> {
        Mat2 {
            r0: Vec2 { x: self[1][1], y: -self[0][1] },
            r1: Vec2 { x: -self[1][0], y: self[0][0] },
        }
    }

    pub fn inverse(&self) -> Mat2<S> {
        self.adjoint() / self.determinant()
    }

    pub fn transpose(&self) -> Mat2<S> {
        Mat2 {
            r0: Vec2 { x: self[0][0], y: self[1][0] },
            r1: Vec2 { x: self[0][1], y: self[1][1] },
        }
    }

    pub fn get_rotation_base(theta: S) -> Mat2<S> {
        let s = theta.sin();
        let c = theta.cos();

        Mat2 {
            r0: Vec2 { x: c, y: s },
            r1: Vec2 { x: -s, y: c },
        }
    }
}

impl<S> Index<usize> for Mat2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            _ => panic!("Requested an invalid row of a Mat2: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for Mat2<S> where S: FloatType<S> {
    fn index_mut(&mut self, index: usize) -> &mut Vec2<S> {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            _ => panic!("Requested an invalid row of a Mat2: {}", index)
        }
    }
}

impl<S> Neg for Mat2<S> where S: FloatType<S> {
    type Output = Mat2<S>;

    fn neg(self) -> Self::Output {
        Mat2 {
            r0: -self.r0,
            r1: -self.r1,
        }
    }
}

impl<S> Not for Mat2<S> where S: FloatType<S> {
    type Output = Mat2<S>;

    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl<S> Mul<Mat2<S>> for Mat2<S> where S: FloatType<S> {
    type Output = Mat2<S>;

    fn mul(self, rhs: Mat2<S>) -> Self::Output {
        let rhs = rhs.transpose();
        Mat2 {
            r0: Vec2 { x: self.r0.dot(rhs.r0), y: self.r0.dot(rhs.r1) },
            r1: Vec2 { x: self.r1.dot(rhs.r0), y: self.r1.dot(rhs.r1) },
        }
    }
}

impl<S> Mul<Vec2<S>> for Mat2<S> where S: FloatType<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.r0.dot(rhs),
            y: self.r1.dot(rhs),
        }
    }
}

impl<S> Mul<S> for Mat2<S> where S: FloatType<S> {
    type Output = Mat2<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Mat2 {
            r0: self.r0 * rhs,
            r1: self.r1 * rhs,
        }
    }
}

impl<S> MulAssign<Mat2<S>> for Mat2<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: Mat2<S>) {
        let new = self.clone() * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
    }
}

impl<S> MulAssign<S> for Mat2<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: S) {
        let new = self.clone() * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
    }
}


impl<S> Div<S> for Mat2<S> where S: FloatType<S> {
    type Output = Mat2<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv_scale = S::one() / rhs;
        self * inv_scale
    }
}

impl<S> Div<Mat2<S>> for Mat2<S> where S: FloatType<S> {
    type Output = Mat2<S>;

    fn div(self, rhs: Mat2<S>) -> Self::Output {
        let inv_mat = rhs.inverse();
        self * inv_mat
    }
}

impl<S> DivAssign<S> for Mat2<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: S) {
        let new = self.clone() / rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
    }
}

impl<S> DivAssign<Mat2<S>> for Mat2<S> where S: FloatType<S> {
    fn div_assign(&mut self, rhs: Mat2<S>) {
        let new = self.clone() / rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
    }
}


impl<S> From<[[S; 2]; 2]> for Mat2<S> where S: FloatType<S> {
    fn from(arr_mat: [[S; 2]; 2]) -> Mat2<S> {
        Mat2 {
            r0: Vec2::from(arr_mat[0]),
            r1: Vec2::from(arr_mat[1]),
        }
    }
}

impl<S> PartialEq for Mat2<S> where S: FloatType<S> {
    fn eq(&self, other: &Mat2<S>) -> bool {
        self.r0 == other.r0 && self.r1 == other.r1
    }
}


impl<S> fmt::Display for Mat2<S> where S: FloatType<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "⌈{:.2} {:.2}⌉\n\
                   ⌊{:.2} {:.2}⌋",
               self.r0.x, self.r0.y,
               self.r1.x, self.r1.y)
    }
}

impl<S> Default for Mat2<S> where S: FloatType<S> {
    fn default() -> Mat2<S> {
        Mat2::identity()
    }
}


impl glium::uniforms::AsUniformValue for Mat2<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Mat2(std::mem::transmute::<Mat2<f32>, [[f32; 2]; 2]>(self.transpose()))
        }
    }
}

impl glium::uniforms::AsUniformValue for Mat2<f64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::DoubleMat2(std::mem::transmute::<Mat2<f64>, [[f64; 2]; 2]>(self.transpose()))
        }
    }
}

unsafe impl glium::vertex::Attribute for Mat2<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x2x2
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}

unsafe impl glium::vertex::Attribute for Mat2<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64x2x2
    }

    fn is_supported<C: ?Sized>(_caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}