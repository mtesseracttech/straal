use std::fmt;
use std::ops::*;

use super::*;

//going with row-major, since column major is the absolute worst to work with.

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat3 {
    pub r0: Vec3,
    pub r1: Vec3,
    pub r2: Vec3,
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Mat3 {
        r0: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        r1: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        r2: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
    };

    pub const EMPTY: Mat3 = Mat3 {
        r0: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        r1: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        r2: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
    };


    pub fn new(r0c0: Real, r0c1: Real, r0c2: Real,
               r1c0: Real, r1c1: Real, r1c2: Real,
               r2c0: Real, r2c1: Real, r2c2: Real, ) -> Self {
        Mat3 {
            r0: Vec3 { x: r0c0, y: r0c1, z: r0c2 },
            r1: Vec3 { x: r1c0, y: r1c1, z: r1c2 },
            r2: Vec3 { x: r2c0, y: r2c1, z: r2c2 },
        }
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
        let det = self.determinant();
        if det.approx_eq(0.0, DEF_F32_EPSILON) {
            self.adjoint() / det
        } else {
            Mat3::IDENTITY
        }
    }

    pub fn transpose(&self) -> Self {
        Self::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }


    //Performs a rotation around the cardinal axes, in the order BPH (handy for camera rotation)
    pub fn get_rotation_mat_euler_upr_obj_rad(pitch: Real, heading: Real, bank: Real) -> Mat3 {
        let sp = pitch.sin();
        let cp = pitch.cos();
        let sh = heading.sin();
        let ch = heading.cos();
        let sb = bank.sin();
        let cb = bank.cos();

        Self::new(ch * cb + sh * sp * sb, sb * cp, -sh * cb + ch * sp * sb,
                  -ch * sb + sh * sp * cb, cb * cp, sb * sh + ch * sp * cb,
                  sh * cp, -sp, ch * cp)
    }

    pub fn get_rotation_mat_euler_upr_obj_deg(pitch: Real, heading: Real, bank: Real) -> Mat3 {
        Self::get_rotation_mat_euler_upr_obj_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }

    //Performs a rotation around the cardinal axes, in the order HPB
    pub fn get_rotation_mat_euler_obj_upr_rad(pitch: Real, heading: Real, bank: Real) -> Mat3 {
        let sp = pitch.sin();
        let cp = pitch.cos();
        let sh = heading.sin();
        let ch = heading.cos();
        let sb = bank.sin();
        let cb = bank.cos();

        Self::new(ch * cb + sh * sp * sb, -ch * sb + sh * sp * cb, sh * cp,
                  sb * cp, cb * cp, -sp,
                  -sh * cb + ch * sp * sb, sb * sh + ch * sp * cb, ch * cp)
    }

    pub fn get_rotation_mat_euler_obj_upr_deg(pitch: Real, heading: Real, bank: Real) -> Mat3 {
        Self::get_rotation_mat_euler_obj_upr_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }


    pub fn get_euler_angles_obj_upr(&self) -> Vec3 {
        let mut angles = Vec3::ZERO;

        let sp = -self[2][1];
        let pitch = if sp <= -1.0 {
            -std::f32::consts::FRAC_PI_2
        } else if sp >= 1.0 {
            std::f32::consts::FRAC_PI_2
        } else {
            sp.asin()
        };

        let mut bank = 0.0;
        let mut heading = 0.0;

        if sp.abs() > 0.9999 {
            heading = -self[0][2].atan2(self[0][0]);
            bank = 0.0;
        } else {
            heading = self[0][2].atan2(self[2][2]);
            bank = self[0][1].atan2(self[1][1]);
        }
        Vec3 {
            x: pitch,
            y: heading,
            z: bank,
        }
    }

    //Rotates the matrix around an arbitrary axis
    pub fn rotate_around(&mut self, n: Vec3, theta: Real) {
        *self *= Self::get_angle_axis(n, theta);
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
        *self *= Self::get_scale_mat(factors);
    }

    pub fn get_scale_mat(factors: Vec3) -> Mat3 {
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
        Mat3::new(Vec3::dot(self[0], rhs[0]), Vec3::dot(self[0], rhs[1]), Vec3::dot(self[0], rhs[2]),
                  Vec3::dot(self[1], rhs[0]), Vec3::dot(self[1], rhs[1]), Vec3::dot(self[1], rhs[2]),
                  Vec3::dot(self[2], rhs[0]), Vec3::dot(self[2], rhs[1]), Vec3::dot(self[2], rhs[2]))
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: Vec3::dot(self.r0, rhs),
            y: Vec3::dot(self.r1, rhs),
            z: Vec3::dot(self.r2, rhs),
        }
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
        let x2 = q.v.x * q.v.x;
        let y2 = q.v.y * q.v.y;
        let z2 = q.v.z * q.v.z;

        //Credits to https://github.com/Duckfan77 for helping remind me that basic arithmetic is
        //to be taken seriously
        Self::new(1.0 - 2.0 * (y2 + z2), 2.0 * (q.v.x * q.v.y + q.w * q.v.z), 2.0 * (q.v.x * q.v.z - q.w * q.v.y),
                  2.0 * (q.v.x * q.v.y - q.w * q.v.z), 1.0 - 2.0 * (x2 + z2), 2.0 * (q.v.y * q.v.z + q.w * q.v.x),
                  2.0 * (q.v.x * q.v.z + q.w * q.v.y), 2.0 * (q.v.y * q.v.z - q.w * q.v.x), 1.0 - 2.0 * (x2 + y2))
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

impl Default for Mat3 {
    fn default() -> Self {
        Mat3::identity()
    }
}

impl glium::uniforms::AsUniformValue for Mat3 {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Mat3(
                std::mem::transmute::<Self, [[f32; 3]; 3]>(self.transpose()))
        }
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