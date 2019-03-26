use std::fmt;
use std::fmt::Display;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat4<S> {
    pub r0: Vec4<S>,
    pub r1: Vec4<S>,
    pub r2: Vec4<S>,
    pub r3: Vec4<S>,
}


impl<S> Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    pub fn identity() -> Mat4<S> {
        Mat4 {
            r0: Vec4 { x: S::one(), y: S::zero(), z: S::zero(), w: S::zero() },
            r1: Vec4 { x: S::zero(), y: S::one(), z: S::zero(), w: S::zero() },
            r2: Vec4 { x: S::zero(), y: S::zero(), z: S::one(), w: S::zero() },
            r3: Vec4 { x: S::zero(), y: S::zero(), z: S::zero(), w: S::one() },
        }
    }

    pub fn empty() -> Mat4<S> {
        Mat4 {
            r0: Vec4::zero(),
            r1: Vec4::zero(),
            r2: Vec4::zero(),
            r3: Vec4::zero(),
        }
    }

    pub fn new(r0c0: S, r0c1: S, r0c2: S, r0c3: S,
               r1c0: S, r1c1: S, r1c2: S, r1c3: S,
               r2c0: S, r2c1: S, r2c2: S, r2c3: S,
               r3c0: S, r3c1: S, r3c2: S, r3c3: S) -> Mat4<S> {
        Mat4 {
            r0: Vec4 { x: r0c0, y: r0c1, z: r0c2, w: r0c3 },
            r1: Vec4 { x: r1c0, y: r1c1, z: r1c2, w: r1c3 },
            r2: Vec4 { x: r2c0, y: r2c1, z: r2c2, w: r2c3 },
            r3: Vec4 { x: r3c0, y: r3c1, z: r3c2, w: r3c3 },
        }
    }

    pub fn new_from_vec4s(r0: Vec4<S>, r1: Vec4<S>, r2: Vec4<S>, r3: Vec4<S>) -> Mat4<S> {
        Mat4 { r0, r1, r2, r3 }
    }

    pub fn new_from_arrs(r0: [S; 4], r1: [S; 4], r2: [S; 4], r3: [S; 4]) -> Mat4<S> {
        Mat4 {
            r0: Vec4::from(r0),
            r1: Vec4::from(r1),
            r2: Vec4::from(r2),
            r3: Vec4::from(r3),
        }
    }

    pub fn determinant(&self) -> S {
        //https://github.com/g-truc/glm/blob/7590260cf81f3e49f492e992f60dd88cd3265d14/glm/detail/func_matrix.inl#L222
        //Calculating the subfactors that will be reused (they all appear twice in the next step)
        let sf_00 = self[2][2] * self[3][3] - self[2][3] * self[3][2];
        let sf_01 = self[1][2] * self[3][3] - self[1][3] * self[3][2];
        let sf_02 = self[1][2] * self[2][3] - self[1][3] * self[2][2];
        let sf_03 = self[0][2] * self[3][3] - self[0][3] * self[3][2];
        let sf_04 = self[0][2] * self[2][3] - self[0][3] * self[2][2];
        let sf_05 = self[0][2] * self[1][3] - self[0][3] * self[1][2];

        let det_conf = Vec4 {
            x: self[1][1] * sf_00 - self[2][1] * sf_01 + self[3][1] * sf_02,
            y: -(self[0][1] * sf_00 - self[2][1] * sf_03 + self[3][1] * sf_04),
            z: self[0][1] * sf_01 - self[1][1] * sf_03 + self[3][1] * sf_05,
            w: -(self[0][1] * sf_02 - self[1][1] * sf_04 + self[2][1] * sf_05),
        };

        self[0][0] * det_conf.x + self[1][0] * det_conf.y + self[2][0] * det_conf.z + self[3][0] * det_conf.w
    }

    pub fn adjoint(&self) -> Mat4<S> {
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


        let r0 = Vec4 {
            x: self[1][1] * sf00 - self[1][2] * sf01 + self[1][3] * sf02,
            y: -(self[0][1] * sf00 - self[0][2] * sf01 + self[0][3] * sf02),
            z: self[0][1] * sf06 - self[0][2] * sf07 + self[0][3] * sf08,
            w: -(self[0][1] * sf12 - self[0][2] * sf13 + self[0][3] * sf14),
        };

        let r1 = Vec4 {
            x: -(self[1][0] * sf00 - self[1][2] * sf03 + self[1][3] * sf04),
            y: self[0][0] * sf00 - self[0][2] * sf03 + self[0][3] * sf04,
            z: -(self[0][0] * sf06 - self[0][2] * sf09 + self[0][3] * sf10),
            w: self[0][0] * sf12 - self[0][2] * sf15 + self[0][3] * sf16,
        };

        let r2 = Vec4 {
            x: self[1][0] * sf01 - self[1][1] * sf03 + self[1][3] * sf05,
            y: -(self[0][0] * sf01 - self[0][1] * sf03 + self[0][3] * sf05),
            z: self[0][0] * sf07 - self[0][1] * sf09 + self[0][3] * sf11,
            w: -(self[0][0] * sf13 - self[0][1] * sf15 + self[0][3] * sf17),
        };

        let r3 = Vec4 {
            x: -(self[1][0] * sf02 - self[1][1] * sf04 + self[1][2] * sf05),
            y: self[0][0] * sf02 - self[0][1] * sf04 + self[0][2] * sf05,
            z: -(self[0][0] * sf08 - self[0][1] * sf10 + self[0][2] * sf11),
            w: self[0][0] * sf14 - self[0][1] * sf16 + self[0][2] * sf17,
        };

        Mat4 { r0, r1, r2, r3 }
    }

    //This version does not use the adjoint and determinant functions, because they share a bunch of calculations
    //that are best left un-abstracted for matrices of this size and up (for reduced memory usage, less redundant computation and potential compiler optimizations)
    pub fn inverse(&self) -> Mat4<S> {
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


        let r0 = Vec4 {
            x: self[1][1] * sf00 - self[1][2] * sf01 + self[1][3] * sf02,
            y: -(self[0][1] * sf00 - self[0][2] * sf01 + self[0][3] * sf02),
            z: self[0][1] * sf06 - self[0][2] * sf07 + self[0][3] * sf08,
            w: -(self[0][1] * sf12 - self[0][2] * sf13 + self[0][3] * sf14),
        };

        let r1 = Vec4 {
            x: -(self[1][0] * sf00 - self[1][2] * sf03 + self[1][3] * sf04),
            y: self[0][0] * sf00 - self[0][2] * sf03 + self[0][3] * sf04,
            z: -(self[0][0] * sf06 - self[0][2] * sf09 + self[0][3] * sf10),
            w: self[0][0] * sf12 - self[0][2] * sf15 + self[0][3] * sf16,
        };

        let r2 = Vec4 {
            x: self[1][0] * sf01 - self[1][1] * sf03 + self[1][3] * sf05,
            y: -(self[0][0] * sf01 - self[0][1] * sf03 + self[0][3] * sf05),
            z: self[0][0] * sf07 - self[0][1] * sf09 + self[0][3] * sf11,
            w: -(self[0][0] * sf13 - self[0][1] * sf15 + self[0][3] * sf17),
        };

        let r3 = Vec4 {
            x: -(self[1][0] * sf02 - self[1][1] * sf04 + self[1][2] * sf05),
            y: self[0][0] * sf02 - self[0][1] * sf04 + self[0][2] * sf05,
            z: -(self[0][0] * sf08 - self[0][1] * sf10 + self[0][2] * sf11),
            w: self[0][0] * sf14 - self[0][1] * sf16 + self[0][2] * sf17,
        };

        let adj = Mat4 { r0, r1, r2, r3 };

        let det = self[0][0] * adj[0][0] + self[1][0] * adj[0][1] + self[2][0] * adj[0][2] + self[3][0] * adj[0][3];

        adj / det
    }

    //Transposes the matrix (swaps the elements over the diagonal)
    pub fn transpose(&self) -> Mat4<S> {
        Mat4 {
            r0: Vec4 { x: self[0][0], y: self[1][0], z: self[2][0], w: self[3][0] },
            r1: Vec4 { x: self[0][1], y: self[1][1], z: self[2][1], w: self[3][1] },
            r2: Vec4 { x: self[0][2], y: self[1][2], z: self[2][2], w: self[3][2] },
            r3: Vec4 { x: self[0][3], y: self[1][3], z: self[2][3], w: self[3][3] },
        }
    }

    //From base matrices

    pub fn get_rotation_mat_flex_euler_deg(angles: Vec3<S>, order: RotationOrder) -> Mat4<S> {
        Mat4::from(Mat3::get_rotation_mat_flex_euler_deg(angles, order))
    }

    pub fn get_rotation_mat_flex_euler_rad(angles: Vec3<S>, order: RotationOrder) -> Mat4<S> {
        Mat4::from(Mat3::get_rotation_mat_flex_euler_rad(angles, order))
    }

    pub fn get_rotation_mat_euler_upr_obj_deg(pitch: S, heading: S, bank: S) -> Mat4<S> {
        Mat4::from(Mat3::get_rotation_mat_euler_upr_obj_deg(pitch, heading, bank))
    }

    pub fn get_rotation_mat_euler_upr_obj_rad(pitch: S, heading: S, bank: S) -> Mat4<S> {
        Mat4::from(Mat3::get_rotation_mat_euler_upr_obj_rad(pitch, heading, bank))
    }

    pub fn get_rotation_mat_euler_obj_upr_deg(pitch: S, heading: S, bank: S) -> Mat4<S> {
        Mat4::from(Mat3::get_rotation_mat_euler_obj_upr_deg(pitch, heading, bank))
    }

    pub fn get_rotation_mat_euler_obj_upr_rad(pitch: S, heading: S, bank: S) -> Mat4<S> {
        Mat4::from(Mat3::get_rotation_mat_euler_obj_upr_rad(pitch, heading, bank))
    }

    pub fn get_angle_axis_mat_deg(n: Vec3<S>, theta: S) -> Mat4<S> {
        Mat4::from(Mat3::get_angle_axis_mat_deg(n, theta))
    }

    pub fn get_angle_axis_mat_rad(n: Vec3<S>, theta: S) -> Mat4<S> {
        Mat4::from(Mat3::get_angle_axis_mat_rad(n, theta))
    }

    pub fn get_uniform_scale_mat(factors: Vec3<S>) -> Mat4<S> {
        Mat4::from(Mat3::get_uniform_scale_mat(factors))
    }

    pub fn get_scale_along_axis_mat(n: Vec3<S>, s: S) -> Mat4<S> {
        Mat4::from(Mat3::get_scale_along_axis_mat(n, s))
    }

    //Mat4 creation

    pub fn get_translation_mat(pos: Vec3<S>) -> Mat4<S> {
        let mut trans = Mat4::identity();
        trans[0][3] = pos.x;
        trans[1][3] = pos.y;
        trans[2][3] = pos.z;
        trans
    }

    //Direct operations on Mat4

    pub fn rotate_by_euler_flex_deg(&mut self, angles: Vec3<S>, order: RotationOrder) {
        *self *= Mat4::get_rotation_mat_flex_euler_deg(angles, order)
    }

    pub fn rotate_by_euler_flex_rad(&mut self, angles: Vec3<S>, order: RotationOrder) {
        *self *= Mat4::get_rotation_mat_flex_euler_rad(angles, order)
    }

    pub fn rotate_by_euler_upr_obj_deg(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat4::get_rotation_mat_euler_upr_obj_deg(pitch, heading, bank)
    }

    pub fn rotate_by_euler_upr_obj_rad(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat4::get_rotation_mat_euler_upr_obj_rad(pitch, heading, bank)
    }

    pub fn rotate_by_euler_obj_upr_deg(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat4::get_rotation_mat_euler_obj_upr_deg(pitch, heading, bank)
    }

    pub fn rotate_by_euler_obj_upr_rad(&mut self, pitch: S, heading: S, bank: S) {
        *self *= Mat4::get_rotation_mat_euler_obj_upr_rad(pitch, heading, bank)
    }

    pub fn rotate_around_axis_deg(&mut self, n: Vec3<S>, theta: S) {
        *self *= Mat4::get_angle_axis_mat_deg(n, theta);
    }

    pub fn rotate_around_axis_rad(&mut self, n: Vec3<S>, theta: S) {
        *self *= Mat4::get_angle_axis_mat_rad(n, theta);
    }

    pub fn scale_uniformly(&mut self, factors: Vec3<S>) {
        *self *= Mat4::get_uniform_scale_mat(factors);
    }

    pub fn scale_along_axis(&mut self, n: Vec3<S>, s: S) {
        *self *= Mat4::get_scale_along_axis_mat(n, s);
    }

    pub fn translate(&mut self, trans: Vec3<S>) {
        *self *= Mat4::get_translation_mat(trans);
    }
}

impl<S> Index<usize> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

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

impl<S> IndexMut<usize> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn index_mut(&mut self, index: usize) -> &mut Vec4<S> {
        match index {
            0 => &mut self.r0,
            1 => &mut self.r1,
            2 => &mut self.r2,
            3 => &mut self.r3,
            _ => panic!("Requested an invalid row of a Mat4: {}", index)
        }
    }
}


impl<S> Not for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat4<S>;

    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl<S> Neg for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat4<S>;

    fn neg(self) -> Self::Output {
        Mat4 {
            r0: -self.r0,
            r1: -self.r1,
            r2: -self.r2,
            r3: -self.r3,
        }
    }
}


impl<S> Mul<Mat4<S>> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat4<S>;

    fn mul(self, rhs: Mat4<S>) -> Self::Output {
        let rhs = rhs.transpose();
        Mat4 {
            r0: Vec4 { x: self[0].dot(rhs[0]), y: self[0].dot(rhs[1]), z: self[0].dot(rhs[2]), w: self[0].dot(rhs[3]) },
            r1: Vec4 { x: self[1].dot(rhs[0]), y: self[1].dot(rhs[1]), z: self[1].dot(rhs[2]), w: self[1].dot(rhs[3]) },
            r2: Vec4 { x: self[2].dot(rhs[0]), y: self[2].dot(rhs[1]), z: self[2].dot(rhs[2]), w: self[2].dot(rhs[3]) },
            r3: Vec4 { x: self[3].dot(rhs[0]), y: self[3].dot(rhs[1]), z: self[3].dot(rhs[2]), w: self[3].dot(rhs[3]) },
        }
    }
}

impl<S> Mul<Vec4<S>> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Vec4<S>;

    fn mul(self, rhs: Vec4<S>) -> Self::Output {
        Vec4 {
            x: self.r0.dot(rhs),
            y: self.r1.dot(rhs),
            z: self.r2.dot(rhs),
            w: self.r3.dot(rhs),
        }
    }
}

impl<S> Mul<S> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat4<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Mat4 {
            r0: self.r0 * rhs,
            r1: self.r1 * rhs,
            r2: self.r2 * rhs,
            r3: self.r3 * rhs,
        }
    }
}

impl<S> MulAssign<Mat4<S>> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn mul_assign(&mut self, rhs: Mat4<S>) {
        let new = self.clone() * rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
        self.r3 = new.r3;
    }
}

impl<S> Div<S> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat4<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv_scale = S::one() / rhs;
        self * inv_scale
    }
}

impl<S> Div<Mat4<S>> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    type Output = Mat4<S>;

    fn div(self, rhs: Mat4<S>) -> Self::Output {
        let inv_mat = rhs.inverse();
        self * inv_mat
    }
}

impl<S> DivAssign<S> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: S) {
        let new = self.clone() / rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
        self.r3 = new.r3;
    }
}

impl<S> DivAssign<Mat4<S>> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn div_assign(&mut self, rhs: Mat4<S>) {
        let new = self.clone() / rhs;
        self.r0 = new.r0;
        self.r1 = new.r1;
        self.r2 = new.r2;
        self.r3 = new.r3;
    }
}


impl<S> From<[[S; 4]; 4]> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn from(mat: [[S; 4]; 4]) -> Mat4<S> {
        Mat4 {
            r0: Vec4::from(mat[0]),
            r1: Vec4::from(mat[1]),
            r2: Vec4::from(mat[2]),
            r3: Vec4::from(mat[3]),
        }
    }
}

//impl From<Quat> for Mat4 {
//    fn from(q: Quat) -> Self {
//        Self::from(Mat3::from(q))
//    }
//}


impl<S> PartialEq for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn eq(&self, other: &Mat4<S>) -> bool {
        self.r0 == other.r0 &&
            self.r1 == other.r1 &&
            self.r2 == other.r2 &&
            self.r3 == other.r3
    }
}

impl<S> fmt::Display for Mat4<S> where S: num::Float + DefaultEpsilon<S> + fmt::Display {
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

impl<S> From<Mat2<S>> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn from(mat: Mat2<S>) -> Mat4<S> {
        Mat4 {
            r0: Vec4::from(mat.r0),
            r1: Vec4::from(mat.r1),
            r2: Vec4 { x: S::zero(), y: S::zero(), z: S::one(), w: S::zero() },
            r3: Vec4 { x: S::zero(), y: S::zero(), z: S::zero(), w: S::one() },
        }
    }
}

impl<S> From<Mat3<S>> for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn from(mat: Mat3<S>) -> Mat4<S> {
        Mat4 {
            r0: Vec4::from(mat.r0),
            r1: Vec4::from(mat.r1),
            r2: Vec4::from(mat.r2),
            r3: Vec4 { x: S::zero(), y: S::zero(), z: S::zero(), w: S::one() },
        }
    }
}

impl<S> Default for Mat4<S> where S: num::Float + DefaultEpsilon<S> {
    fn default() -> Mat4<S> {
        Mat4::identity()
    }
}

impl glium::uniforms::AsUniformValue for Mat4<f32> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::Mat4(std::mem::transmute::<Mat4<f32>, [[f32; 4]; 4]>(self.transpose()))
        }
    }
}

impl glium::uniforms::AsUniformValue for Mat4<f64> {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        unsafe {
            glium::uniforms::UniformValue::DoubleMat4(std::mem::transmute::<Mat4<f64>, [[f64; 4]; 4]>(self.transpose()))
        }
    }
}


unsafe impl glium::vertex::Attribute for Mat4<f32> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32x4x4
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}

unsafe impl glium::vertex::Attribute for Mat4<f64> {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F64x4x4
    }

    fn is_supported<C: ?Sized>(caps: &C) -> bool where C: glium::CapabilitiesSource {
        true
    }
}