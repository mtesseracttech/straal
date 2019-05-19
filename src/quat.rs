use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash)]
pub struct Quat<S> {
    pub w: S,
    pub v: Vec3<S>,
}


impl<S> Quat<S> where S: FloatType<S> {
    pub fn identity() -> Quat<S> {
        Quat {
            w: S::one(),
            v: Vec3::zero(),
        }
    }

    pub fn new<U>(w: U, x: U, y: U, z: U) -> Quat<S> where U: InputType {
        Quat {
            w: num::cast(w).unwrap(),
            v: Vec3 {
                x: num::cast(x).unwrap(),
                y: num::cast(y).unwrap(),
                z: num::cast(z).unwrap(),
            },
        }
    }

    pub fn dot(self, rhs: Quat<S>) -> S {
        self.w * rhs.w + self.v.dot(rhs.v)
    }

    pub fn magnitude_squared(self) -> S {
        self.dot(self)
    }

    pub fn magnitude(self) -> S {
        self.magnitude_squared().sqrt()
    }

    pub fn conjugate(self) -> Quat<S> {
        Quat {
            w: self.w,
            v: -self.v,
        }
    }

    pub fn inverse(self) -> Quat<S> {
        let inv_fact = S::one() / self.magnitude();
        self.conjugate() * inv_fact
    }

    pub fn normalized_safe(self) -> Quat<S> {
        let mag = self.magnitude();
        if mag < S::DEF_EPSILON {
            Quat::identity()
        } else {
            self / mag
        }
    }

    pub fn normalized(self) -> Quat<S> {
        self / self.magnitude()
    }

    pub fn is_pure(self) -> bool {
        self.w.approx_eq(S::zero(), S::DEF_EPSILON)
    }

    pub fn is_unit(self) -> bool {
        self.magnitude_squared().approx_eq(S::one(), S::DEF_EPSILON)
    }

    pub fn is_pure_unit(self) -> bool {
        self.is_pure() && self.is_unit()
    }

    pub fn pow(&self, exponent: S) -> Quat<S> {
        if self.w.abs() < num::cast(0.9999).unwrap() {
            let alpha = self.w.acos();
            let new_alpha = alpha * exponent;
            let scalar = new_alpha.sin() / alpha.sin();
            Quat {
                w: new_alpha.cos(),
                v: self.v * scalar,
            }
        } else {
            Quat {
                w: self.w,
                v: self.v,
            }
        }
    }


    pub fn get_quat_flex_euler_deg(pitch: S, heading: S, bank: S, order: RotationOrder) -> Quat<S> {
        Quat::get_quat_flex_euler_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians(), order)
    }

    pub fn get_quat_flex_euler_rad(pitch: S, heading: S, bank: S, order: RotationOrder) -> Quat<S> {
        let two = num::cast(2).unwrap();
        let pitch = pitch / two;
        let heading = heading / two;
        let bank = bank / two;

        let sp = pitch.sin();
        let cp = pitch.cos();
        let sh = heading.sin();
        let ch = heading.cos();
        let sb = bank.sin();
        let cb = bank.cos();

        let p = Quat { w: cp, v: Vec3 { x: sp, y: S::zero(), z: S::zero() } };
        let h = Quat { w: ch, v: Vec3 { x: S::zero(), y: sh, z: S::zero() } };
        let b = Quat { w: cb, v: Vec3 { x: S::zero(), y: S::zero(), z: sb } };

        match order {
            RotationOrder::PHB => p * h * b,
            RotationOrder::PBH => p * b * h,
            RotationOrder::HPB => h * p * b,
            RotationOrder::HBP => b * b * p,
            RotationOrder::BPH => b * p * h,
            RotationOrder::BHP => b * h * p,
        }
    }

    pub fn get_quat_euler_obj_upr_deg(pitch: S, heading: S, bank: S) -> Quat<S> {
        Quat::get_quat_euler_obj_upr_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }

    //Performs a rotation around the cardinal axes, in the order BPH (handy for camera rotation)
    pub fn get_quat_euler_obj_upr_rad(pitch: S, heading: S, bank: S) -> Quat<S> {
        let two = num::cast(2).unwrap();
        let pitch = pitch / two;
        let heading = heading / two;
        let bank = bank / two;

        let sp = pitch.sin();
        let cp = pitch.cos();
        let sh = heading.sin();
        let ch = heading.cos();
        let sb = bank.sin();
        let cb = bank.cos();

        Quat {
            w: ch * cp * cb + sh * sp * sb,
            v: Vec3 {
                x: ch * sp * cb + sh * cp * sb,
                y: sh * cp * cb - ch * sp * sb,
                z: ch * cp * sb - sh * sp * cb,
            },
        }
    }

    pub fn get_quat_euler_upr_obj_deg(pitch: S, heading: S, bank: S) -> Quat<S> {
        Quat::get_quat_euler_upr_obj_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }

    pub fn get_quat_euler_upr_obj_rad(pitch: S, heading: S, bank: S) -> Quat<S> {
        let two = num::cast(2).unwrap();
        let pitch = pitch / two;
        let heading = heading / two;
        let bank = bank / two;

        let sp = pitch.sin();
        let cp = pitch.cos();
        let sh = heading.sin();
        let ch = heading.cos();
        let sb = bank.sin();
        let cb = bank.cos();

        Quat {
            w: ch * cp * cb + sh * sp * sb,
            v: Vec3 {
                x: -ch * sp * cb - sh * cp * sb,
                y: ch * sp * sb - sh * cp * cb,
                z: sh * sp * cb - ch * cp * sb,
            },
        }
    }

    pub fn get_euler_angles_obj_upr_deg(&self) -> Vec3<S> {
        self.get_euler_angles_obj_upr_rad() * S::one().to_degrees()
    }

    pub fn get_euler_angles_obj_upr_rad(&self) -> Vec3<S> {
        let min_two: S = num::cast(-2).unwrap();
        let half: S = num::cast(0.5).unwrap();
        let almost_one: S = num::cast(0.9999).unwrap();
        let half_pi: S = num::cast(std::f64::consts::FRAC_PI_2).unwrap();
        let sin_pitch: S = min_two * (self.v.y * self.v.z - self.w * self.v.x);

        if sin_pitch.abs() > almost_one {
            Vec3 {
                x: half_pi * sin_pitch,
                y: (-self.v.x * self.v.z + self.w * self.v.y).atan2(half - self.v.y * self.v.y - self.v.z * self.v.z),
                z: S::zero(),
            }
        } else {
            Vec3 {
                x: sin_pitch.asin(),
                y: (self.v.x * self.v.z + self.w * self.v.y).atan2(half - self.v.x * self.v.x - self.v.y * self.v.y),
                z: (self.v.x * self.v.y + self.w * self.v.z).atan2(half - self.v.x * self.v.x - self.v.z * self.v.z),
            }
        }
    }


    pub fn get_euler_angles_upr_obj_deg(self) -> Vec3<S> {
        self.get_euler_angles_upr_obj_rad() * S::one().to_degrees()
    }

    pub fn get_euler_angles_upr_obj_rad(self) -> Vec3<S> {
        let min_two: S = num::cast(-2).unwrap();
        let half: S = num::cast(0.5).unwrap();
        let almost_one: S = num::cast(0.9999).unwrap();
        let half_pi: S = num::cast(std::f64::consts::FRAC_PI_2).unwrap();
        let sin_pitch: S = min_two * (self.v.y * self.v.z + self.w * self.v.x);

        if sin_pitch.abs() > almost_one {
            Vec3 {
                x: half_pi * sin_pitch,
                y: (-self.v.x * self.v.z - self.w * self.v.y).atan2(half - self.v.y * self.v.y - self.v.z * self.v.z),
                z: S::zero(),
            }
        } else {
            Vec3 {
                x: sin_pitch.asin(),
                y: (self.v.x * self.v.z - self.w * self.v.y).atan2(half - self.v.x * self.v.x - self.v.y * self.v.y),
                z: (self.v.x * self.v.y - self.w * self.v.z).atan2(half - self.v.x * self.v.x - self.v.z * self.v.z),
            }
        }
    }

    //Performs a rotation around an arbitary unit axis
    pub fn get_quat_from_angle_axis(theta: S, n: Vec3<S>) -> Quat<S> {
        debug_assert!(n.is_unit());
        let half_theta = theta * num::cast(0.5).unwrap();
        Quat {
            w: half_theta.cos(),
            v: n * half_theta.sin(),
        }
    }

    pub fn get_quat_from_angle_axis_safe(theta: S, n: Vec3<S>) -> Quat<S> {
        let magnitude = n.length();
        if magnitude > S::DEF_EPSILON {
            let half_theta = theta * num::cast(0.5).unwrap();
            let s = half_theta.sin() / magnitude;
            Quat {
                w: half_theta.cos(),
                v: n * half_theta.sin(),
            }
        } else {
            Quat::identity()
        }
    }

    pub fn get_angle_axis_from_quat(&self) -> (Vec3<S>, S) {
        let q = if self.w > S::one() {
            self.normalized()
        } else {
            *self
        };

        let theta = (S::one() + S::one()) * q.w;
        let s = (S::one() - q.w * q.w).sqrt();
        if s < num::cast(0.0001).unwrap() {
            (q.v.normalized(), theta)
        } else {
            (q.v / s, theta)
        }
    }

    pub fn slerp(self, other: Quat<S>, t: S) -> Quat<S> {
        let mut cos_omega = self.dot(other);

        let q0 = self;
        let mut q1 = other;

        if cos_omega < S::zero() {
            q1 = -q1;
            cos_omega = -cos_omega;
        }

        if cos_omega > num::cast(0.9999).unwrap() {
            let k0 = S::one() - t;
            let k1 = t;
            Quat {
                w: q0.w * k0 + q1.w * k1,
                v: q0.v * k0 + q1.v * k1,
            }
        } else {
            let sin_omega = (S::one() - cos_omega * cos_omega).sqrt();

            let omega = sin_omega.atan2(cos_omega);

            let one_over_sin_omega = S::one() / sin_omega;

            let k0 = ((S::one() - t) * omega).sin() * one_over_sin_omega;
            let k1 = (t * omega).sin() * one_over_sin_omega;
            Quat {
                w: q0.w * k0 + q1.w * k1,
                v: q0.v * k0 + q1.v * k1,
            }
        }
    }

    pub fn lerp(self, other: Quat<S>, t: S) -> Quat<S> {
        if self.dot(other) < S::zero() {
            Quat {
                w: self.w + (-other.w - self.w) * t,
                v: self.v + (-other.v - self.v) * t,
            }
        } else {
            Quat {
                w: self.w + (other.w - self.w) * t,
                v: self.v + (other.v - self.v) * t,
            }
        }.normalized()
    }


    //Rotates the quaternion around an arbitrary axis
    pub fn rotate_around(&mut self, theta: S, n: Vec3<S>) {
        *self *= Quat::get_quat_from_angle_axis(theta, n);
    }
}


impl<S> Not for Quat<S> where S: FloatType<S> {
    type Output = Quat<S>;

    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl<S> Neg for Quat<S> where S: FloatType<S> {
    type Output = Quat<S>;

    fn neg(self) -> Self::Output {
        Quat {
            w: -self.w,
            v: -self.v,
        }
    }
}


impl<S> Mul<Quat<S>> for Quat<S> where S: FloatType<S> {
    type Output = Quat<S>;

    fn mul(self, rhs: Quat<S>) -> Quat<S> {
        Quat {
            w: rhs.w * self.w - self.v.dot(rhs.v),
            v: rhs.v * self.w + self.v * rhs.w + self.v.cross(rhs.v),
        }
    }
}

impl<S> Mul<Vec3<S>> for Quat<S> where S: FloatType<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: Vec3<S>) -> Self::Output {
        let p = Quat { w: S::zero(), v: rhs };
        let ps = self * p * self.inverse();
        ps.v
    }
}

impl<S> Mul<S> for Quat<S> where S: FloatType<S> {
    type Output = Quat<S>;

    fn mul(self, rhs: S) -> Self::Output {
        Quat {
            w: self.w * rhs,
            v: self.v * rhs,
        }
    }
}


impl<S> MulAssign<Quat<S>> for Quat<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: Quat<S>) {
        let temp = *self * rhs;
        self.w = temp.w;
        self.v = temp.v;
    }
}

impl<S> MulAssign<S> for Quat<S> where S: FloatType<S> {
    fn mul_assign(&mut self, rhs: S) {
        let temp = *self * rhs;
        self.w = temp.w;
        self.v = temp.v;
    }
}

impl<S> Div<Quat<S>> for Quat<S> where S: FloatType<S> {
    type Output = Quat<S>;

    fn div(self, rhs: Quat<S>) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<S> Div<S> for Quat<S> where S: FloatType<S> {
    type Output = Quat<S>;

    fn div(self, rhs: S) -> Self::Output {
        let inv = S::one() / rhs;
        Quat {
            w: self.w * inv,
            v: self.v * inv,
        }
    }
}

impl<S> PartialEq for Quat<S> where S: FloatType<S> {
    fn eq(&self, other: &Quat<S>) -> bool {
        self.w.approx_eq(other.w, S::DEF_EPSILON) && self.v == other.v
    }
}

impl<S> fmt::Display for Quat<S> where S: FloatType<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} ({:.2}i {:.2}j {:.2}k)", self.w, self.v.x, self.v.y, self.v.z)
    }
}

impl<S> From<(S, S, S, S)> for Quat<S> where S: FloatType<S> {
    fn from(tuple: (S, S, S, S)) -> Quat<S> {
        Quat {
            w: tuple.0,
            v: Vec3 {
                x: tuple.1,
                y: tuple.2,
                z: tuple.3,
            },
        }
    }
}

impl<S> From<[S; 4]> for Quat<S> where S: FloatType<S> {
    fn from(arr: [S; 4]) -> Quat<S> {
        Quat {
            w: arr[0],
            v: Vec3 {
                x: arr[1],
                y: arr[2],
                z: arr[3],
            },
        }
    }
}

impl<S> From<Mat3<S>> for Quat<S> where S: FloatType<S> {
    fn from(m: Mat3<S>) -> Quat<S> {
        let four_w_sq_m_1 = m[0][0] + m[1][1] + m[2][2];
        let four_x_sq_m_1 = m[0][0] - m[1][1] - m[2][2];
        let four_y_sq_m_1 = -m[0][0] + m[1][1] - m[2][2];
        let four_z_sq_m_1 = -m[0][0] - m[1][1] + m[2][2];

        let mut biggest_index = 0;
        let mut four_biggest_sq_m_1 = four_w_sq_m_1;
        if four_x_sq_m_1 > four_biggest_sq_m_1 {
            four_biggest_sq_m_1 = four_x_sq_m_1;
            biggest_index = 1;
        }
        if four_y_sq_m_1 > four_biggest_sq_m_1 {
            four_biggest_sq_m_1 = four_y_sq_m_1;
            biggest_index = 2;
        }
        if four_z_sq_m_1 > four_biggest_sq_m_1 {
            four_biggest_sq_m_1 = four_z_sq_m_1;
            biggest_index = 3;
        }

        let biggest_val = (four_biggest_sq_m_1 + S::one()).sqrt() * num::cast(0.5).unwrap();
        let one_fourth: S = num::cast(0.25).unwrap();
        let mult = one_fourth / biggest_val;

        match biggest_index {
            0 => {
                Quat {
                    w: biggest_val,
                    v: Vec3 {
                        x: m[1][2] - m[2][1],
                        y: m[2][0] - m[0][2],
                        z: m[0][1] - m[1][0],
                    } * mult,
                }
            }
            1 => {
                Quat {
                    w: (m[1][2] - m[2][1]) * mult,
                    v: Vec3 {
                        x: biggest_val,
                        y: (m[0][1] + m[1][0]) * mult,
                        z: (m[2][0] + m[0][2]) * mult,
                    },
                }
            }
            2 => {
                Quat {
                    w: (m[2][0] - m[0][2]) * mult,
                    v: Vec3 {
                        x: (m[0][1] + m[1][0]) * mult,
                        y: biggest_val,
                        z: (m[2][1] + m[1][2]) * mult,
                    },
                }
            }
            3 => {
                Quat {
                    w: (m[0][1] - m[1][0]) * mult,
                    v: Vec3 {
                        x: (m[2][0] + m[0][2]) * mult,
                        y: (m[1][2] + m[2][1]) * mult,
                        z: biggest_val,
                    },
                }
            }
            _ => {
                Quat::identity()
            }
        }
    }
}


impl<S> Default for Quat<S> where S: FloatType<S> {
    fn default() -> Quat<S> {
        Quat::identity()
    }
}
