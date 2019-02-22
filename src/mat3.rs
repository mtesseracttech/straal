use std::fmt;
use std::ops::*;
use std::str;

use glium::uniforms::AsUniformValue;

use super::*;

//going with row-major, since column major is the absolute worst to work with.

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
    r0: Vec3,
    r1: Vec3,
    r2: Vec3,
}

impl Mat3 {
    pub fn new(r0c0: Scalar, r0c1: Scalar, r0c2: Scalar,
               r1c0: Scalar, r1c1: Scalar, r1c2: Scalar,
               r2c0: Scalar, r2c1: Scalar, r2c2: Scalar, ) -> Self {
        Self::new_from_vec3s(Vec3::new(r0c0, r0c1, r0c2),
                             Vec3::new(r1c0, r1c1, r1c2),
                             Vec3::new(r2c0, r2c1, r2c2))
    }

    pub fn new_from_vec3s(r0: Vec3, r1: Vec3, r2: Vec3) -> Self {
        Mat3 { r0, r1, r2 }
    }

    pub fn new_from_arrs(r0: [Scalar; 3], r1: [Scalar; 3], r2: [Scalar; 3]) -> Self {
        Self::new_from_vec3s(Vec3::from(r0), Vec3::from(r1), Vec3::from(r2))
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0,
                  0.0, 1.0, 0.0,
                  0.0, 0.0, 1.0)
    }

    pub fn determinant(&self) -> Scalar {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1]) -
            self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0]) +
            self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }

    pub fn inverse(&self) -> Self {
        let inv_det = 1.0 / self.determinant();
        Self::new_from_vec3s(self.r0 * inv_det,
                             self.r1 * inv_det,
                             self.r2 * inv_det)
    }

    pub fn transpose(&self) -> Self {
        Self::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let rhs = rhs.transpose();
        Mat3::new(Vec3::dot(&self[0], &rhs[0]), Vec3::dot(&self[0], &rhs[1]), Vec3::dot(&self[0], &rhs[2]),
                  Vec3::dot(&self[1], &rhs[0]), Vec3::dot(&self[1], &rhs[1]), Vec3::dot(&self[1], &rhs[2]),
                  Vec3::dot(&self[2], &rhs[0]), Vec3::dot(&self[2], &rhs[1]), Vec3::dot(&self[2], &rhs[2]))
    }
}

impl Mul<Scalar> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let output = self.clone();
        output.r0 * rhs;
        output.r1 * rhs;
        output.r2 * rhs;
        output
    }
}

impl From<[[Scalar; 3]; 3]> for Mat3 {
    fn from(mat: [[f32; 3]; 3]) -> Self {
        Self::new_from_arrs(mat[0], mat[1], mat[2])
    }
}

impl Index<usize> for Mat3 {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            _ => panic!("Requested an invalid row of a Mat3: {}", index)
        }
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            _ => panic!("Requested an invalid row of a Mat3: {}", index)
        }
    }
}

impl PartialEq for Mat3 {
    fn eq(&self, other: &Mat3) -> bool {
        self.r0 == other.r0 && self.r1 == other.r1 && self.r2 == other.r2
    }
}

impl fmt::Display for Mat3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "⌈{:.2} {:.2} {:.2}⌉\n\
                   |{:.2} {:.2} {:.2}|\n\
                   ⌊{:.2} {:.2} {:.2}⌋",
               self.r0.x, self.r0.y, self.r0.z,
               self.r1.x, self.r1.y, self.r1.z,
               self.r2.x, self.r2.y, self.r2.z)
    }
}

impl glium::uniforms::AsUniformValue for Mat3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe { glium::uniforms::UniformValue::Mat3(std::mem::transmute::<Self, [[f32; 3]; 3]>(self.transpose())) }
    }
}