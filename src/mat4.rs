use std::fmt;
use std::ops::*;
use std::str;

use glium::uniforms::AsUniformValue;

use super::*;

//going with row-major, since column major is the absolute worst to work with.

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
    pub r0: Vec4,
    pub r1: Vec4,
    pub r2: Vec4,
    pub r3: Vec4,
}

impl Mat4 {
    pub fn new(r0c0: Scalar, r0c1: Scalar, r0c2: Scalar, r0c3: Scalar,
               r1c0: Scalar, r1c1: Scalar, r1c2: Scalar, r1c3: Scalar,
               r2c0: Scalar, r2c1: Scalar, r2c2: Scalar, r2c3: Scalar,
               r3c0: Scalar, r3c1: Scalar, r3c2: Scalar, r3c3: Scalar) -> Self {
        Self::new_from_vec4s(Vec4::new(r0c0, r0c1, r0c2, r0c3),
                             Vec4::new(r1c0, r1c1, r1c2, r1c3),
                             Vec4::new(r2c0, r2c1, r2c2, r2c3),
                             Vec4::new(r3c0, r3c1, r3c2, r3c3))
    }

    pub fn new_from_vec4s(r0: Vec4, r1: Vec4, r2: Vec4, r3: Vec4) -> Self {
        Mat4 { r0, r1, r2, r3 }
    }

    pub fn new_from_arrs(r0: [Scalar; 4], r1: [Scalar; 4], r2: [Scalar; 4], r3: [Scalar; 4]) -> Self {
        Self::new_from_vec4s(Vec4::from(r0), Vec4::from(r1), Vec4::from(r2), Vec4::from(r3))
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0,
                  0.0, 1.0, 0.0, 0.0,
                  0.0, 0.0, 1.0, 0.0,
                  0.0, 0.0, 0.0, 1.0)
    }

    pub fn determinant(&self) -> Scalar {
        //https://github.com/g-truc/glm/blob/7590260cf81f3e49f492e992f60dd88cd3265d14/glm/detail/func_matrix.inl#L222
        //Calculating the subfactors that will be reused (they all appear twice in the next step)
        let sf_00 = self[2][2] * self[3][3] - self[2][3] * self[3][2];
        let sf_01 = self[1][2] * self[3][3] - self[1][3] * self[3][2];
        let sf_02 = self[1][2] * self[2][3] - self[1][3] * self[2][2];
        let sf_03 = self[0][2] * self[3][3] - self[0][3] * self[3][2];
        let sf_04 = self[0][2] * self[2][3] - self[0][3] * self[2][2];
        let sf_05 = self[0][2] * self[1][3] - self[0][3] * self[1][2];

        //Creating the cofactor vector
        let det_cof = Vec4::new(
            (self[1][1] * sf_00 - self[2][1] * sf_01 + self[3][1] * sf_02),
            -(self[0][1] * sf_00 - self[2][1] * sf_03 + self[3][1] * sf_04),
            (self[0][1] * sf_01 - self[1][1] * sf_03 + self[3][1] * sf_05),
            -(self[0][1] * sf_02 - self[1][1] * sf_04 + self[2][1] * sf_05),
        );

        self[0][0] * det_cof[0] + self[1][0] * det_cof[1] + self[2][0] * det_cof[2] + self[3][0] * det_cof[3]
    }

    pub fn inverse(&self) -> Self {
        let inv_det = 1.0 / self.determinant();
        Self::new_from_vec4s(self.r0 * inv_det,
                             self.r1 * inv_det,
                             self.r2 * inv_det,
                             self.r3 * inv_det)
    }

    pub fn transpose(&self) -> Self {
        Self::new(self[0][0], self[1][0], self[2][0], self[3][0],
                  self[0][1], self[1][1], self[2][1], self[3][1],
                  self[0][2], self[1][2], self[2][2], self[3][2],
                  self[0][3], self[1][3], self[2][3], self[3][3])
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let rhs = rhs.transpose();
        Mat4::new(Vec4::dot(&self[0], &rhs[0]), Vec4::dot(&self[0], &rhs[1]), Vec4::dot(&self[0], &rhs[2]), Vec4::dot(&self[0], &rhs[3]),
                  Vec4::dot(&self[1], &rhs[0]), Vec4::dot(&self[1], &rhs[1]), Vec4::dot(&self[1], &rhs[2]), Vec4::dot(&self[1], &rhs[3]),
                  Vec4::dot(&self[2], &rhs[0]), Vec4::dot(&self[2], &rhs[1]), Vec4::dot(&self[2], &rhs[2]), Vec4::dot(&self[2], &rhs[3]),
                  Vec4::dot(&self[3], &rhs[0]), Vec4::dot(&self[3], &rhs[1]), Vec4::dot(&self[3], &rhs[2]), Vec4::dot(&self[3], &rhs[3]))
    }
}

impl Mul<Scalar> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let output = self.clone();
        output.r0 * rhs;
        output.r1 * rhs;
        output.r2 * rhs;
        output.r3 * rhs;
        output
    }
}

impl From<[[Scalar; 4]; 4]> for Mat4 {
    fn from(mat: [[f32; 4]; 4]) -> Self {
        Self::new_from_arrs(mat[0], mat[1], mat[2], mat[3])
    }
}


impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            3 => &self.r3,
            _ => panic!("Requested an invalid row of a Mat4: {}", index)
        }
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r2,
            _ => panic!("Requested an invalid row of a Mat4: {}", index)
        }
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Mat4) -> bool {
        self.r0 == other.r0 && self.r1 == other.r1 && self.r2 == other.r2 && self.r3 == other.r3
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "⌈{:.2} {:.2} {:.2} {:.2}⌉\n\
                   |{:.2} {:.2} {:.2} {:.2}|\n\
                   |{:.2} {:.2} {:.2} {:.2}|\n\
                   ⌊{:.2} {:.2} {:.2} {:.2}⌋",
               self.r0.x, self.r0.y, self.r0.z, self.r0.w,
               self.r1.x, self.r1.y, self.r1.z, self.r1.w,
               self.r2.x, self.r2.y, self.r2.z, self.r2.w,
               self.r3.x, self.r3.y, self.r3.z, self.r3.w)
    }
}

impl glium::uniforms::AsUniformValue for Mat4 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe { glium::uniforms::UniformValue::Mat4(std::mem::transmute::<Self, [[f32; 4]; 4]>(self.transpose())) }
    }
}