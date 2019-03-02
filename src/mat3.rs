use std::fmt;
use std::ops::*;

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

    pub fn angles_to_axes_zxy(angles: Vec3) -> Mat3 {
        const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
        let angles = angles * DEG_TO_RAD;
        let sx = angles.x.sin();
        let cx = angles.x.cos();
        let sy = angles.y.sin();
        let cy = angles.y.cos();
        let sz = angles.z.sin();
        let cz = angles.z.cos();

        Mat3::new(sx * sy * sz + cy * cz, cx * sz, sx * cy * sz - sy * cz,
                  sx * sy * cz - cy * sz, cx * cz, sy * sz + sx * cy * cz,
                  cx * sy, -sx, cx * cy)
    }
}

impl Not for Mat3 {
    type Output = Mat3;

    fn not(self) -> Self::Output {
        self.inverse()
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

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            Vec3::dot(&self.r0, &rhs),
            Vec3::dot(&self.r1, &rhs),
            Vec3::dot(&self.r2, &rhs),
        )
    }
}

impl Mul<Scalar> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut output = self.clone();
        output.r0 *= rhs;
        output.r1 *= rhs;
        output.r2 *= rhs;
        output
    }
}

impl From<[[Scalar; 3]; 3]> for Mat3 {
    fn from(mat: [[f32; 3]; 3]) -> Self {
        Self::new_from_arrs(mat[0], mat[1], mat[2])
    }
}

impl From<Quat> for Mat3 {
    fn from(q: Quat) -> Self {
        let a2 = q.w * q.w;
        let b2 = q.x * q.x;
        let c2 = q.y * q.y;
        let d2 = q.z * q.z;

        //Normalizes the quaternion, to be sure
        let inv = 1.0 / (a2 + b2 + c2 + d2);

        let r0c0 = (a2 + b2 - c2 - d2) * inv;
        let r1c1 = (a2 - b2 + c2 - d2) * inv;
        let r2c2 = (a2 - b2 - c2 + d2) * inv;

        let t1 = q.x * q.y;
        let t2 = q.z * q.w;

        let r1c0 = 2.0 * (t1 + t2) * inv;
        let r0c1 = 2.0 * (t1 - t2) * inv;

        let t1 = q.x * q.z;
        let t2 = q.y * q.w;

        let r2c0 = 2.0 * (t1 - t2) * inv;
        let r0c2 = 2.0 * (t1 + t2) * inv;

        let t1 = q.y * q.z;
        let t2 = q.x * q.w;

        let r2c1 = 2.0 * (t1 + t2) * inv;
        let r1c2 = 2.0 * (t1 - t2) * inv;

        Self::new(r0c0, r0c1, r0c2,
                  r1c0, r1c1, r1c2,
                  r2c0, r2c1, r2c2)
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