use std::fmt;
use std::ops::*;

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
    pub fn new(r0c0: Real, r0c1: Real, r0c2: Real, r0c3: Real,
               r1c0: Real, r1c1: Real, r1c2: Real, r1c3: Real,
               r2c0: Real, r2c1: Real, r2c2: Real, r2c3: Real,
               r3c0: Real, r3c1: Real, r3c2: Real, r3c3: Real) -> Self {
        Self::new_from_vec4s(Vec4::new(r0c0, r0c1, r0c2, r0c3),
                             Vec4::new(r1c0, r1c1, r1c2, r1c3),
                             Vec4::new(r2c0, r2c1, r2c2, r2c3),
                             Vec4::new(r3c0, r3c1, r3c2, r3c3))
    }

    pub fn new_from_vec4s(r0: Vec4, r1: Vec4, r2: Vec4, r3: Vec4) -> Self {
        Mat4 { r0, r1, r2, r3 }
    }

    pub fn new_from_arrs(r0: [Real; 4], r1: [Real; 4], r2: [Real; 4], r3: [Real; 4]) -> Self {
        Self::new_from_vec4s(Vec4::from(r0), Vec4::from(r1), Vec4::from(r2), Vec4::from(r3))
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0,
                  0.0, 1.0, 0.0, 0.0,
                  0.0, 0.0, 1.0, 0.0,
                  0.0, 0.0, 0.0, 1.0)
    }

    pub fn determinant(&self) -> Real {
        //https://github.com/g-truc/glm/blob/7590260cf81f3e49f492e992f60dd88cd3265d14/glm/detail/func_matrix.inl#L222
        //Calculating the subfactors that will be reused (they all appear twice in the next step)
        let sf_00 = self[2][2] * self[3][3] - self[2][3] * self[3][2];
        let sf_01 = self[1][2] * self[3][3] - self[1][3] * self[3][2];
        let sf_02 = self[1][2] * self[2][3] - self[1][3] * self[2][2];
        let sf_03 = self[0][2] * self[3][3] - self[0][3] * self[3][2];
        let sf_04 = self[0][2] * self[2][3] - self[0][3] * self[2][2];
        let sf_05 = self[0][2] * self[1][3] - self[0][3] * self[1][2];

        let det_cof = Vec4::new(
            self[1][1] * sf_00 - self[2][1] * sf_01 + self[3][1] * sf_02,
            -(self[0][1] * sf_00 - self[2][1] * sf_03 + self[3][1] * sf_04),
            self[0][1] * sf_01 - self[1][1] * sf_03 + self[3][1] * sf_05,
            -(self[0][1] * sf_02 - self[1][1] * sf_04 + self[2][1] * sf_05),
        );

        self[0][0] * det_cof[0] + self[1][0] * det_cof[1] + self[2][0] * det_cof[2] + self[3][0] * det_cof[3]
    }


    pub fn adjoint(&self) -> Mat4 {
        //Pre-calculating sub-factors, since all of them are used 4 times
        let sf00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let sf01 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let sf02 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let sf03 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
        let sf04 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
        let sf05 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let sf06 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let sf07 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let sf08 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let sf09 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
        let sf10 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
        let sf11 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
        let sf12 = self[1][2] * self[2][3] - self[2][2] * self[1][3];
        let sf13 = self[1][1] * self[2][3] - self[2][1] * self[1][3];
        let sf14 = self[1][1] * self[2][2] - self[2][1] * self[1][2];
        let sf15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];
        let sf16 = self[1][0] * self[2][2] - self[2][0] * self[1][2];
        let sf17 = self[1][0] * self[2][1] - self[2][0] * self[1][1];


        let r0 = Vec4::new(self[1][1] * sf00 - self[1][2] * sf01 + self[1][3] * sf02,
                           -(self[0][1] * sf00 - self[0][2] * sf01 + self[0][3] * sf02),
                           self[0][1] * sf06 - self[0][2] * sf07 + self[0][3] * sf08,
                           -(self[0][1] * sf12 - self[0][2] * sf13 + self[0][3] * sf14));

        let r1 = Vec4::new(-(self[1][0] * sf00 - self[1][2] * sf03 + self[1][3] * sf04),
                           self[0][0] * sf00 - self[0][2] * sf03 + self[0][3] * sf04,
                           -(self[0][0] * sf06 - self[0][2] * sf09 + self[0][3] * sf10),
                           self[0][0] * sf12 - self[0][2] * sf15 + self[0][3] * sf16);

        let r2 = Vec4::new(self[1][0] * sf01 - self[1][1] * sf03 + self[1][3] * sf05,
                           -(self[0][0] * sf01 - self[0][1] * sf03 + self[0][3] * sf05),
                           self[0][0] * sf07 - self[0][1] * sf09 + self[0][3] * sf11,
                           -(self[0][0] * sf13 - self[0][1] * sf15 + self[0][3] * sf17));

        let r3 = Vec4::new(-(self[1][0] * sf02 - self[1][1] * sf04 + self[1][2] * sf05),
                           self[0][0] * sf02 - self[0][1] * sf04 + self[0][2] * sf05,
                           -(self[0][0] * sf08 - self[0][1] * sf10 + self[0][2] * sf11),
                           self[0][0] * sf14 - self[0][1] * sf16 + self[0][2] * sf17);

        Mat4::new_from_vec4s(r0, r1, r2, r3)
    }

    //This version does not use the adjoint and determinant functions, because they share a bunch of calculations
    //that are best left un-abstracted for matrices of this size and up (for reduced memory usage, less redundant computation and potential compiler optimizations)
    pub fn inverse(&self) -> Self {
        //Pre-calculating sub-factors, since all of them are used 4 times
        let sf00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let sf01 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let sf02 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let sf03 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
        let sf04 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
        let sf05 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let sf06 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let sf07 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let sf08 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let sf09 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
        let sf10 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
        let sf11 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
        let sf12 = self[1][2] * self[2][3] - self[2][2] * self[1][3];
        let sf13 = self[1][1] * self[2][3] - self[2][1] * self[1][3];
        let sf14 = self[1][1] * self[2][2] - self[2][1] * self[1][2];
        let sf15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];
        let sf16 = self[1][0] * self[2][2] - self[2][0] * self[1][2];
        let sf17 = self[1][0] * self[2][1] - self[2][0] * self[1][1];


        let r0 = Vec4::new(self[1][1] * sf00 - self[1][2] * sf01 + self[1][3] * sf02,
                           -(self[0][1] * sf00 - self[0][2] * sf01 + self[0][3] * sf02),
                           self[0][1] * sf06 - self[0][2] * sf07 + self[0][3] * sf08,
                           -(self[0][1] * sf12 - self[0][2] * sf13 + self[0][3] * sf14));

        let r1 = Vec4::new(-(self[1][0] * sf00 - self[1][2] * sf03 + self[1][3] * sf04),
                           self[0][0] * sf00 - self[0][2] * sf03 + self[0][3] * sf04,
                           -(self[0][0] * sf06 - self[0][2] * sf09 + self[0][3] * sf10),
                           self[0][0] * sf12 - self[0][2] * sf15 + self[0][3] * sf16);

        let r2 = Vec4::new(self[1][0] * sf01 - self[1][1] * sf03 + self[1][3] * sf05,
                           -(self[0][0] * sf01 - self[0][1] * sf03 + self[0][3] * sf05),
                           self[0][0] * sf07 - self[0][1] * sf09 + self[0][3] * sf11,
                           -(self[0][0] * sf13 - self[0][1] * sf15 + self[0][3] * sf17));

        let r3 = Vec4::new(-(self[1][0] * sf02 - self[1][1] * sf04 + self[1][2] * sf05),
                           self[0][0] * sf02 - self[0][1] * sf04 + self[0][2] * sf05,
                           -(self[0][0] * sf08 - self[0][1] * sf10 + self[0][2] * sf11),
                           self[0][0] * sf14 - self[0][1] * sf16 + self[0][2] * sf17);

        let adj = Mat4::new_from_vec4s(r0, r1, r2, r3);

        let det = self[0][0] * adj[0][0] + self[1][0] * adj[0][1] + self[2][0] * adj[0][2] + self[3][0] * adj[0][3];

        adj / det
    }

    pub fn transpose(&self) -> Self {
        Self::new(self[0][0], self[1][0], self[2][0], self[3][0],
                  self[0][1], self[1][1], self[2][1], self[3][1],
                  self[0][2], self[1][2], self[2][2], self[3][2],
                  self[0][3], self[1][3], self[2][3], self[3][3])
    }

    //Performs a rotation around the cardinal axes, in the order ZXY (handy for camera rotation)
    pub fn angles_to_axes_zxy(angles: Vec3) -> Mat4 {
        Self::from(Mat3::angles_to_axes_zxy(angles))
    }

    pub fn get_angle_axis(n: Vec3, theta: Real) -> Mat4 {
        Self::from(Mat3::get_angle_axis(n, theta))
    }

    pub fn transform(pos: &Vec3) {}


    pub fn get_scale_mat(factors: Vec3) -> Mat4 {
        Self::from(Mat3::get_scale(factors))
    }

    pub fn get_scale_along_axis_mat(n: Vec3, s: Real) -> Mat4 {
        Self::from(Mat3::get_scale_along_axis(n, s))
    }
}

impl Not for Mat4 {
    type Output = Mat4;

    fn not(self) -> Self::Output {
        self.inverse()
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

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            Vec4::dot(&self.r0, &rhs),
            Vec4::dot(&self.r1, &rhs),
            Vec4::dot(&self.r2, &rhs),
            Vec4::dot(&self.r3, &rhs),
        )
    }
}

impl Mul<Real> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        let mut output = self.clone();
        output.r0 *= rhs;
        output.r1 *= rhs;
        output.r2 *= rhs;
        output.r3 *= rhs;
        output
    }
}

impl MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        let new = *self * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
        self.r3 = new.r3;
    }
}

impl Div<Real> for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: f32) -> Self::Output {
        let inv_scale = 1.0 / rhs;
        self * inv_scale
    }
}

impl From<[[Real; 4]; 4]> for Mat4 {
    fn from(mat: [[f32; 4]; 4]) -> Self {
        Self::new_from_arrs(mat[0], mat[1], mat[2], mat[3])
    }
}

impl From<Quat> for Mat4 {
    fn from(q: Quat) -> Self {
        let a2 = q.w * q.w;
        let b2 = q.x * q.x;
        let c2 = q.y * q.y;
        let d2 = q.z * q.z;

        let inv = 1.0 / (a2 + b2 + c2 + d2);

        let r0c0 = (a2 + b2 - c2 - d2) * inv;
        let r1c1 = (a2 - b2 + c2 - d2) * inv;
        let r2c2 = (a2 - b2 - c2 + d2) * inv;

        let t0 = q.x * q.y;
        let t1 = q.z * q.w;

        let r1c0 = 2.0 * (t0 + t1) * inv;
        let r0c1 = 2.0 * (t0 - t1) * inv;

        let t0 = q.x * q.z;
        let t1 = q.y * q.w;

        let r2c0 = 2.0 * (t0 - t1) * inv;
        let r0c2 = 2.0 * (t0 + t1) * inv;

        let t0 = q.y * q.z;
        let t1 = q.x * q.w;

        let r2c1 = 2.0 * (t0 + t1) * inv;
        let r1c2 = 2.0 * (t0 - t1) * inv;

        Self::new(r0c0, r0c1, r0c2, 0.0,
                  r1c0, r1c1, r1c2, 0.0,
                  r2c0, r2c1, r2c2, 0.0,
                  0.0, 0.0, 0.0, 1.0)
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
               self[0][0], self[0][1], self[0][2], self[0][3],
               self[1][0], self[1][1], self[1][2], self[1][3],
               self[2][0], self[2][1], self[2][2], self[2][3],
               self[3][0], self[3][1], self[3][2], self[3][3])
    }
}

impl From<Mat2> for Mat4 {
    fn from(mat: Mat2) -> Self {
        Self::new_from_vec4s(Vec4::from(mat[0]),
                             Vec4::from(mat[1]),
                             Vec4::new(0.0, 0.0, 1.0, 0.0),
                             Vec4::new(0.0, 0.0, 0.0, 1.0))
    }
}

impl From<Mat3> for Mat4 {
    fn from(mat: Mat3) -> Self {
        Self::new_from_vec4s(Vec4::from(mat[0]),
                             Vec4::from(mat[1]),
                             Vec4::from(mat[2]),
                             Vec4::new(0.0, 0.0, 0.0, 1.0))
    }
}

impl glium::uniforms::AsUniformValue for Mat4 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe { glium::uniforms::UniformValue::Mat4(std::mem::transmute::<Self, [[f32; 4]; 4]>(self.transpose())) }
    }
}

unsafe impl glium::vertex::Attribute for Mat4 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x4x4
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}