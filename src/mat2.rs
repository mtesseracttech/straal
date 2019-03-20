use std::fmt;
use std::ops::*;

use super::*;

//going with row-major, since column major is the absolute worst to work with.

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat2 {
    pub r0: Vec2,
    pub r1: Vec2,
}

impl Mat2 {
    pub fn new(r0c0: Real, r0c1: Real,
               r1c0: Real, r1c1: Real) -> Self {
        Self::new_from_vec2s(Vec2::new(r0c0, r0c1),
                             Vec2::new(r1c0, r1c1))
    }

    pub fn new_from_vec2s(r0: Vec2, r1: Vec2) -> Self {
        Mat2 { r0, r1 }
    }

    pub fn new_from_arrs(r0: [Real; 2], r1: [Real; 2]) -> Self {
        Self::new_from_vec2s(Vec2::from(r0), Vec2::from(r1))
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0,
                  0.0, 1.0)
    }

    pub fn determinant(&self) -> Real {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pub fn adjoint(&self) -> Mat2 {
        Self::new(self[1][1], -self[0][1],
                  -self[1][0], self[0][0])
    }

    pub fn inverse(&self) -> Self {
        self.adjoint() / self.determinant()
    }

    pub fn transpose(&self) -> Self {
        Self::new(self[0][0], self[1][0],
                  self[0][1], self[1][1])
    }

    pub fn rotation(theta: Real) -> Mat2 {
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

impl Mul<Real> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        let mut output = self.clone();
        output.r0 *= rhs;
        output.r1 *= rhs;
        output
    }
}

impl MulAssign<Mat2> for Mat2 {
    fn mul_assign(&mut self, rhs: Mat2) {
        let new = *self * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
    }
}

impl Div<Real> for Mat2 {
    type Output = Mat2;

    fn div(self, rhs: f32) -> Self::Output {
        let inv_scale = 1.0 / rhs;
        self * inv_scale
    }
}

impl From<[[Real; 2]; 2]> for Mat2 {
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

impl Default for Mat2 {
    fn default() -> Self {
        Mat2::identity()
    }
}

impl glium::uniforms::AsUniformValue for Mat2 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Mat2(
                std::mem::transmute::<Self, [[f32; 2]; 2]>(self.transpose()))
        }
    }
}

unsafe impl glium::vertex::Attribute for Mat2 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x2x2
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}