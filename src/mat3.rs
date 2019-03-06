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
    pub fn new(r0c0: Real, r0c1: Real, r0c2: Real,
               r1c0: Real, r1c1: Real, r1c2: Real,
               r2c0: Real, r2c1: Real, r2c2: Real, ) -> Self {
        Self::new_from_vec3s(Vec3::new(r0c0, r0c1, r0c2),
                             Vec3::new(r1c0, r1c1, r1c2),
                             Vec3::new(r2c0, r2c1, r2c2))
    }

    pub fn new_from_vec3s(r0: Vec3, r1: Vec3, r2: Vec3) -> Self {
        Mat3 { r0, r1, r2 }
    }

    pub fn new_from_arrs(r0: [Real; 3], r1: [Real; 3], r2: [Real; 3]) -> Self {
        Self::new_from_vec3s(Vec3::from(r0), Vec3::from(r1), Vec3::from(r2))
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0,
                  0.0, 1.0, 0.0,
                  0.0, 0.0, 1.0)
    }

    pub fn determinant(&self) -> Real {
        self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
            self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
            self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
    }

    pub fn adjoint(&self) -> Mat3 {
        let r0 = Vec3::new(self[1][1] * self[2][2] - self[1][2] * self[2][1],
                           -(self[0][1] * self[2][2] - self[0][2] * self[2][1]),
                           self[0][1] * self[1][2] - self[0][2] * self[1][1]);
        let r1 = Vec3::new(-(self[1][0] * self[2][2] - self[1][2] * self[2][0]),
                           self[0][0] * self[2][2] - self[0][2] * self[2][0],
                           -(self[0][0] * self[1][2] - self[0][2] * self[1][0]));
        let r2 = Vec3::new(self[1][0] * self[2][1] - self[1][1] * self[2][0],
                           -(self[0][0] * self[2][1] - self[0][1] * self[2][0]),
                           self[0][0] * self[1][1] - self[0][1] * self[1][0]);

        Self::new_from_vec3s(r0, r1, r2)
    }

    pub fn inverse(&self) -> Mat3 {
        self.adjoint() / self.determinant()
    }

    pub fn transpose(&self) -> Self {
        Self::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }

    //Performs a rotation around the cardinal axes, in the order ZXY (handy for camera rotation)
    pub fn angles_to_axes_zxy(angles: Vec3) -> Mat3 {
        const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
        let angles = angles * DEG_TO_RAD;
        let sx = angles.x.sin();
        let cx = angles.x.cos();
        let sy = angles.y.sin();
        let cy = angles.y.cos();
        let sz = angles.z.sin();
        let cz = angles.z.cos();

        Self::new(sx * sy * sz + cy * cz, cx * sz, sx * cy * sz - sy * cz,
                  sx * sy * cz - cy * sz, cx * cz, sy * sz + sx * cy * cz,
                  cx * sy, -sx, cx * cy)
    }

    pub fn get_euler_angles(&self) -> Vec3 {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        let sp = -self[3][2];
        if sp <= -1.0 {
            x = -std::f32::consts::FRAC_PI_2
        } else if sp >= 1.0 {
            x = std::f32::consts::FRAC_PI_2
        } else {
            x = sp.asin()
        }

        if sp.abs() > 0.9999 {
            y = -self[3][1].atan2(self[1][1]);
            z = 0.0;
        } else {
            y = self[1][3].atan2(self[3][3]);
            z = self[2][1].atan2(self[2][2]);
        }
        Vec3::new(x, y, z)
    }

    //Performs a rotation around an arbitary unit axis
    pub fn get_angle_axis(n: Vec3, theta: Real) -> Mat3 {
        debug_assert!(n.is_unit());
        let ct = theta.cos();
        let st = theta.sin();
        let p_cos = 1.0 - ct; //1.0 - cos(theta), so basically a cosine from 0 to 2

        Self::new(n.x * n.x * p_cos + ct, n.x * n.y * p_cos + n.z * st, n.x * n.z * p_cos - n.y * st,
                  n.x * n.y * p_cos - n.z * st, n.y * n.y * p_cos + ct, n.y * n.z * p_cos + n.x * st,
                  n.x * n.z * p_cos + n.y * st, n.y * n.z * p_cos - n.x * st, n.z * n.z * p_cos + ct)
    }

    pub fn scale(&mut self, factors: Vec3) {
        *self *= Self::get_scale(factors);
    }

    pub fn get_scale(factors: Vec3) -> Mat3 {
        Self::new(factors.x, 0.0, 0.0,
                  0.0, factors.y, 0.0,
                  0.0, 0.0, factors.z)
    }

    pub fn get_scale_along_axis(n: Vec3, s: Real) -> Mat3 {
        debug_assert!(n.is_unit());

        let s_min_one = s - 1.0;

        Self::new(1.0 + s_min_one * n.x * n.x, s_min_one * n.x * n.y, s_min_one * n.x * n.z,
                  s_min_one * n.x * n.y, 1.0 + s_min_one * n.y * n.y, s_min_one * n.y * n.z,
                  s_min_one * n.x * n.z, s_min_one * n.z * n.y, 1.0 + s_min_one * n.z * n.z)
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

impl Mul<Real> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Real) -> Self::Output {
        let mut output = self.clone();
        output.r0 *= rhs;
        output.r1 *= rhs;
        output.r2 *= rhs;
        output
    }
}

impl MulAssign<Mat3> for Mat3 {
    fn mul_assign(&mut self, rhs: Mat3) {
        let new = *self * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
    }
}

impl Div<Real> for Mat3 {
    type Output = Mat3;

    fn div(self, rhs: f32) -> Self::Output {
        let inv_scale = 1.0 / rhs;
        self * inv_scale
    }
}

impl From<[[Real; 3]; 3]> for Mat3 {
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

unsafe impl glium::vertex::Attribute for Mat3 {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x3x3
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}