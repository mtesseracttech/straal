use std::fmt;
use std::fmt::Display;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat3<S> {
    pub r0: Vec3<S>,
    pub r1: Vec3<S>,
    pub r2: Vec3<S>,
}


impl<S> Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    pub fn identity() -> Mat3<S> {
        Mat3 {
            r0: Vec3 { x: S::one(), y: S::zero(), z: S::zero() },
            r1: Vec3 { x: S::zero(), y: S::one(), z: S::zero() },
            r2: Vec3 { x: S::zero(), y: S::zero(), z: S::one() },
        }
    }

    pub fn empty() -> Mat3<S> {
        Mat3 {
            r0: Vec3::zero(),
            r1: Vec3::zero(),
            r2: Vec3::zero(),
        }
    }

    pub fn new(r0c0: S, r0c1: S, r0c2: S,
               r1c0: S, r1c1: S, r1c2: S,
               r2c0: S, r2c1: S, r2c2: S, ) -> Mat3<S> {
        Mat3 {
            r0: Vec3 { x: r0c0, y: r0c1, z: r0c2 },
            r1: Vec3 { x: r1c0, y: r1c1, z: r1c2 },
            r2: Vec3 { x: r2c0, y: r2c1, z: r2c2 },
        }
    }

    pub fn new_from_vec3s(r0: Vec3<S>, r1: Vec3<S>, r2: Vec3<S>) -> Mat3<S> {
        Mat3 { r0, r1, r2 }
    }

    pub fn new_from_arrs(r0: [S; 3], r1: [S; 3], r2: [S; 3]) -> Mat3<S> {
        Mat3 {
            r0: Vec3::from(r0),
            r1: Vec3::from(r1),
            r2: Vec3::from(r2),
        }
    }

    pub fn determinant(&self) -> S {
        self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
            self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
            self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
    }

    pub fn adjoint(&self) -> Mat3<S> {
        Mat3 {
            r0: Vec3 {
                x: self[1][1] * self[2][2] - self[1][2] * self[2][1],
                y: -(self[0][1] * self[2][2] - self[0][2] * self[2][1]),
                z: self[0][1] * self[1][2] - self[0][2] * self[1][1],
            },
            r1: Vec3 {
                x: -(self[1][0] * self[2][2] - self[1][2] * self[2][0]),
                y: self[0][0] * self[2][2] - self[0][2] * self[2][0],
                z: -(self[0][0] * self[1][2] - self[0][2] * self[1][0]),
            },
            r2: Vec3 {
                x: self[1][0] * self[2][1] - self[1][1] * self[2][0],
                y: -(self[0][0] * self[2][1] - self[0][1] * self[2][0]),
                z: self[0][0] * self[1][1] - self[0][1] * self[1][0],
            },
        }
    }

    pub fn inverse(&self) -> Mat3<S> {
        self.adjoint() / self.determinant()
    }

    pub fn transpose(&self) -> Mat3<S> {
        Mat3 {
            r0: Vec3 { x: self[0][0], y: self[1][0], z: self[2][0] },
            r1: Vec3 { x: self[0][1], y: self[1][1], z: self[2][1] },
            r2: Vec3 { x: self[0][2], y: self[1][2], z: self[2][2] },
        }
    }

    pub fn get_rotation_mat_flex_euler_deg(angles: Vec3<S>, order: RotationOrder) -> Mat3<S> {
        Mat3::get_rotation_mat_flex_euler_rad(angles * S::to_radians(S::one()), order)
    }

    pub fn get_rotation_mat_flex_euler_rad(angles: Vec3<S>, order: RotationOrder) -> Mat3<S> {
        let sin_pitch = angles.x.sin();
        let cos_pitch = angles.x.cos();
        let sin_heading = angles.y.sin();
        let cos_heading = angles.y.cos();
        let sin_bank = angles.z.sin();
        let cos_bank = angles.z.cos();

        let p = Mat3 {
            r0: Vec3 { x: S::one(), y: S::zero(), z: S::zero() },
            r1: Vec3 { x: S::zero(), y: cos_pitch, z: sin_pitch },
            r2: Vec3 { x: S::zero(), y: -sin_pitch, z: cos_pitch },
        };

        let h = Mat3 {
            r0: Vec3 { x: cos_heading, y: S::zero(), z: -sin_heading },
            r1: Vec3 { x: S::zero(), y: S::one(), z: S::zero() },
            r2: Vec3 { x: sin_heading, y: S::zero(), z: cos_heading },
        };

        let b = Mat3 {
            r0: Vec3 { x: cos_bank, y: sin_bank, z: S::zero() },
            r1: Vec3 { x: -sin_bank, y: cos_bank, z: S::zero() },
            r2: Vec3 { x: S::zero(), y: S::zero(), z: S::one() },
        };

        match order {
            RotationOrder::PHB => p * h * b,
            RotationOrder::PBH => p * b * h,
            RotationOrder::HPB => h * p * b,
            RotationOrder::HBP => b * b * p,
            RotationOrder::BPH => b * p * h,
            RotationOrder::BHP => b * h * p,
        }
    }

    pub fn get_rotation_mat_euler_upr_obj_deg(pitch: S, heading: S, bank: S) -> Mat3<S> {
        Mat3::get_rotation_mat_euler_upr_obj_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }

    //Performs a rotation around the cardinal axes, in the order BPH (handy for camera rotation)
    pub fn get_rotation_mat_euler_upr_obj_rad(pitch: S, heading: S, bank: S) -> Mat3<S> {
        let sp = pitch.sin();
        let cp = pitch.cos();
        let sh = heading.sin();
        let ch = heading.cos();
        let sb = bank.sin();
        let cb = bank.cos();

        Mat3 {
            r0: Vec3 { x: ch * cb + sh * sp * sb, y: sb * cp, z: -sh * cb + ch * sp * sb },
            r1: Vec3 { x: -ch * sb + sh * sp * cb, y: cb * cp, z: sb * sh + ch * sp * cb },
            r2: Vec3 { x: sh * cp, y: -sp, z: ch * cp },
        }
    }

    pub fn get_rotation_mat_euler_obj_upr_deg(pitch: S, heading: S, bank: S) -> Mat3<S> {
        Mat3::get_rotation_mat_euler_obj_upr_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }

    //Performs a rotation around the cardinal axes, in the order HPB
    pub fn get_rotation_mat_euler_obj_upr_rad(pitch: S, heading: S, bank: S) -> Mat3<S> {
        let sp = pitch.sin();
        let cp = pitch.cos();
        let sh = heading.sin();
        let ch = heading.cos();
        let sb = bank.sin();
        let cb = bank.cos();

        Mat3 {
            r0: Vec3 { x: ch * cb + sh * sp * sb, y: -ch * sb + sh * sp * cb, z: sh * cp },
            r1: Vec3 { x: sb * cp, y: cb * cp, z: -sp },
            r2: Vec3 { x: -sh * cb + ch * sp * sb, y: sb * sh + ch * sp * cb, z: ch * cp },
        }
    }

    pub fn get_euler_angles_obj_upr_deg(&self) -> Vec3<S> {
        self.get_euler_angles_obj_upr_rad() * S::one().to_degrees()
    }

    pub fn get_euler_angles_obj_upr_rad(&self) -> Vec3<S> {
        let sp = -self[2][1];
        let pitch = if sp <= -S::one() {
            -S::from(std::f64::consts::FRAC_PI_2).unwrap()
        } else if sp >= S::one() {
            S::from(std::f64::consts::FRAC_PI_2).unwrap()
        } else {
            sp.asin()
        };

        let mut bank = S::zero();
        let mut heading = S::zero();

        if sp.abs() > S::from(0.9999).unwrap() {
            heading = -self[0][2].atan2(self[0][0]);
            bank = S::zero();
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

    //TODO: get euler angles upr obj

    pub fn get_angle_axis_mat_deg(n: Vec3<S>, theta: S) -> Mat3<S> {
        Mat3::get_angle_axis_mat_rad(n, theta.to_radians())
    }

    //Performs a rotation around an arbitary unit axis
    pub fn get_angle_axis_mat_rad(n: Vec3<S>, theta: S) -> Mat3<S> {
        debug_assert!(n.is_unit());
        let ct = theta.cos();
        let st = theta.sin();
        let p_cos = S::one() - ct; //1.0 - cos(theta), so basically a cosine from 0 to 2

        Mat3 {
            r0: Vec3 { x: n.x * n.x * p_cos + ct, y: n.x * n.y * p_cos + n.z * st, z: n.x * n.z * p_cos - n.y * st },
            r1: Vec3 { x: n.x * n.y * p_cos - n.z * st, y: n.y * n.y * p_cos + ct, z: n.y * n.z * p_cos + n.x * st },
            r2: Vec3 { x: n.x * n.z * p_cos + n.y * st, y: n.y * n.z * p_cos - n.x * st, z: n.z * n.z * p_cos + ct },
        }
    }

    pub fn get_uniform_scale_mat(factors: Vec3<S>) -> Mat3<S> {
        Mat3 {
            r0: Vec3 { x: factors.x, y: S::zero(), z: S::zero() },
            r1: Vec3 { x: S::zero(), y: factors.y, z: S::zero() },
            r2: Vec3 { x: S::zero(), y: S::zero(), z: factors.z },
        }
    }

    pub fn get_scale_along_axis_mat(n: Vec3<S>, s: S) -> Mat3<S> {
        debug_assert!(n.is_unit());

        let s_min_one = s - S::one();

        Mat3 {
            r0: Vec3 { x: S::one() + s_min_one * n.x * n.x, y: s_min_one * n.x * n.y, z: s_min_one * n.x * n.z },
            r1: Vec3 { x: s_min_one * n.x * n.y, y: S::one() + s_min_one * n.y * n.y, z: s_min_one * n.y * n.z },
            r2: Vec3 { x: s_min_one * n.x * n.z, y: s_min_one * n.z * n.y, z: S::one() + s_min_one * n.z * n.z },
        }
    }

    pub fn rotate_by_euler_flex_deg(&mut self, angles: Vec3<S>, order: RotationOrder) {
        *self *= Mat3::get_rotation_mat_flex_euler_deg(angles, order)
    }

    pub fn rotate_by_euler_flex_rad(&mut self, angles: Vec3<S>, order: RotationOrder) {
        *self *= Mat3::get_rotation_mat_flex_euler_rad(angles, order)
    }

    pub fn rotate_by_euler_upr_obj_deg(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat3::get_rotation_mat_euler_upr_obj_deg(pitch, heading, bank)
    }

    pub fn rotate_by_euler_upr_obj_rad(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat3::get_rotation_mat_euler_upr_obj_rad(pitch, heading, bank)
    }

    pub fn rotate_by_euler_obj_upr_deg(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat3::get_rotation_mat_euler_obj_upr_deg(pitch, heading, bank)
    }

    pub fn rotate_by_euler_obj_upr_rad(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat3::get_rotation_mat_euler_obj_upr_rad(pitch, heading, bank)
    }

    pub fn rotate_around_axis_deg(&mut self, n: Vec3<S>, theta: S) {
        *self *= Mat3::get_angle_axis_mat_deg(n, theta);
    }

    pub fn rotate_around_axis_rad(&mut self, n: Vec3<S>, theta: S) {
        *self *= Mat3::get_angle_axis_mat_rad(n, theta);
    }

    pub fn scale_uniformly(&mut self, factors: Vec3<S>) {
        *self *= Mat3::get_uniform_scale_mat(factors);
    }

    pub fn scale_along_axis(&mut self, n: Vec3<S>, s: S) {
        *self *= Mat3::get_scale_along_axis_mat(n, s);
    }
}

impl<S> Index<usize> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0,
            1 => &self.r1,
            2 => &self.r2,
            _ => panic!("Requested an invalid row of a Mat3: {}", index)
        }
    }
}

impl<S> IndexMut<usize> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn index_mut(&mut self, index: usize) -> &mut Vec3<S> {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            _ => panic!("Requested an invalid row of a Mat3: {}", index)
        }
    }
}

impl<S> Not for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat3<S>;

    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl<S> Neg for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat3<S>;

    fn neg(self) -> Self::Output {
        Mat3 {
            r0: -self.r0,
            r1: -self.r1,
            r2: -self.r2,
        }
    }
}

impl<S> Mul<Mat3<S>> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat3<S>;

    fn mul(self, rhs: Mat3<S>) -> Self::Output {
        let rhs = rhs.transpose();
        Mat3 {
            r0: Vec3 { x: self[0].dot(rhs[0]), y: self[0].dot(rhs[1]), z: self[0].dot(rhs[2]) },
            r1: Vec3 { x: self[1].dot(rhs[0]), y: self[1].dot(rhs[1]), z: self[1].dot(rhs[2]) },
            r2: Vec3 { x: self[2].dot(rhs[0]), y: self[2].dot(rhs[1]), z: self[2].dot(rhs[2]) },
        }
    }
}

impl<S> Mul<Vec3<S>> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: Vec3<S>) -> Self::Output {
        Vec3 {
            x: self.r0.dot(rhs),
            y: self.r1.dot(rhs),
            z: self.r2.dot(rhs),
        }
    }
}

impl<S> Mul<S> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat3<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Mat3 {
            r0: self.r0 * rhs,
            r1: self.r1 * rhs,
            r2: self.r2 * rhs,
        }
    }
}

impl<S> MulAssign<Mat3<S>> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: Mat3<S>) {
        let new = self.clone() * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
    }
}

impl<S> MulAssign<S> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: S) {
        let new = self.clone() * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
    }
}

impl<S> Div<S> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat3<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv_scale = S::one() / rhs;
        self * inv_scale
    }
}

impl<S> Div<Mat3<S>> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat3<S>;

    fn div(self, rhs: Mat3<S>) -> Self::Output {
        let inv_mat = rhs.inverse();
        self * inv_mat
    }
}

impl<S> DivAssign<S> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: S) {
        let new = self.clone() / rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
    }
}

impl<S> DivAssign<Mat3<S>> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: Mat3<S>) {
        let new = self.clone() / rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
    }
}


impl<S> From<[[S; 3]; 3]> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn from(mat: [[S; 3]; 3]) -> Mat3<S> {
        Mat3 {
            r0: Vec3::from(mat[0]),
            r1: Vec3::from(mat[1]),
            r2: Vec3::from(mat[2]),
        }
    }
}


impl<S> From<Quat<S>> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn from(q: Quat<S>) -> Mat3<S> {
        let x2 = q.v.x * q.v.x;
        let y2 = q.v.y * q.v.y;
        let z2 = q.v.z * q.v.z;

        let two = S::one() + S::one();

        //Credits to https://github.com/Duckfan77 for helping remind me that basic arithmetic is
        //to be taken seriously
        Self::new(S::one() - two * (y2 + z2), two * (q.v.x * q.v.y + q.w * q.v.z), two * (q.v.x * q.v.z - q.w * q.v.y),
                  two * (q.v.x * q.v.y - q.w * q.v.z), S::one() - two * (x2 + z2), two * (q.v.y * q.v.z + q.w * q.v.x),
                  two * (q.v.x * q.v.z + q.w * q.v.y), two * (q.v.y * q.v.z - q.w * q.v.x), S::one() - two * (x2 + y2))
    }
}


impl<S> PartialEq for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn eq(&self, other: &Mat3<S>) -> bool {
        self.r0 == other.r0 &&
            self.r1 == other.r1 &&
            self.r2 == other.r2
    }
}


impl<S> fmt::Display for Mat3<S> where S: num::Float + DefaultEpsilon<S> + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "⌈{:.2} {:.2} {:.2}⌉\n\
                   |{:.2} {:.2} {:.2}|\n\
                   ⌊{:.2} {:.2} {:.2}⌋",
               self.r0.x, self.r0.y, self.r0.z,
               self.r1.x, self.r1.y, self.r1.z,
               self.r2.x, self.r2.y, self.r2.z)
    }
}

impl<S> From<Mat2<S>> for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn from(mat: Mat2<S>) -> Mat3<S> {
        Mat3 {
            r0: Vec3::from(mat.r0),
            r1: Vec3::from(mat.r1),
            r2: Vec3 { x: S::zero(), y: S::zero(), z: S::one() },
        }
    }
}

impl<S> Default for Mat3<S> where S: num::Float + DefaultEpsilon<S> {
    fn default() -> Mat3<S> {
        Mat3::identity()
    }
}

impl glium::uniforms::AsUniformValue for Mat3<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Mat3(std::mem::transmute::<Mat3<f32>, [[f32; 3]; 3]>(self.transpose()))
        }
    }
}

impl glium::uniforms::AsUniformValue for Mat3<f64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::DoubleMat3(std::mem::transmute::<Mat3<f64>, [[f64; 3]; 3]>(self.transpose()))
        }
    }
}

unsafe impl glium::vertex::Attribute for Mat3<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x3x3
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}

unsafe impl glium::vertex::Attribute for Mat3<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64x3x3
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}