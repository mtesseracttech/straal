use std::fmt;
use std::ops::*;
use std::str;

use glium::uniforms::AsUniformValue;

use super::*;

//going with row-major, since column major is the absolute worst to work with.

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat2 {
    r0: Vec2,
    r1: Vec2,
}

impl Mat2 {
    pub fn new(r0c0: Scalar, r0c1: Scalar,
               r1c0: Scalar, r1c1: Scalar) -> Self {
        Self::new_from_vec2s(Vec2::new(r0c0, r0c1),
                             Vec2::new(r1c0, r1c1))
    }

    pub fn new_from_vec2s(r0: Vec2, r1: Vec2) -> Self {
        Mat2 { r0, r1 }
    }

    pub fn new_from_arrs(r0: [Scalar; 2], r1: [Scalar; 2]) -> Self {
        Self::new_from_vec2s(Vec2::from(r0), Vec2::from(r1))
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0,
                  0.0, 1.0)
    }

    pub fn determinant(&self) -> Scalar {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pub fn inverse(&self) -> Self {
        let inv_det = 1.0 / self.determinant();
        Self::new_from_vec2s(self.r0 * inv_det,
                             self.r1 * inv_det)
    }

    pub fn transpose(&self) -> Self {
        Self::new(self[0][0], self[1][0],
                  self[0][1], self[1][1])
    }

    pub fn rotation(theta: Scalar) -> Mat2 {
        let s = theta.sin();
        let c = theta.cos();

        Mat2::new(c, s,
                  -s, c)
    }
}

impl Not for Mat2 {
    type Output = Mat2;

    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: Mat2) -> Self::Output {
        let rhs = rhs.transpose();
        Mat2::new(Vec2::dot(&self.r0, &rhs.r0), Vec2::dot(&self.r0, &rhs.r1),
                  Vec2::dot(&self.r1, &rhs.r0), Vec2::dot(&self.r1, &rhs.r1))
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(
            Vec2::dot(&self.r0, &rhs),
            Vec2::dot(&self.r1, &rhs),
        )
    }
}

impl Mul<Scalar> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let output = self.clone();
        output.r0 * rhs;
        output.r1 * rhs;
        output
    }
}

impl From<[[Scalar; 2]; 2]> for Mat2 {
    fn from(mat: [[f32; 2]; 2]) -> Self {
        Self::new_from_arrs(mat[0], mat[1])
    }
}


impl Index<usize> for Mat2 {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            _ => panic!("Requested an invalid row of a Mat2: {}", index)
        }
    }
}

impl IndexMut<usize> for Mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            _ => panic!("Requested an invalid row of a Mat2: {}", index)
        }
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, other: &Mat2) -> bool {
        self.r0 == other.r0 && self.r1 == other.r1
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "⌈{:.2} {:.2}⌉\n\
                   ⌊{:.2} {:.2}⌋",
               self.r0.x, self.r0.y,
               self.r1.x, self.r1.y)
    }
}

impl glium::uniforms::AsUniformValue for Mat2 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe { glium::uniforms::UniformValue::Mat2(std::mem::transmute::<Self, [[f32; 2]; 2]>(self.transpose())) }
    }
}