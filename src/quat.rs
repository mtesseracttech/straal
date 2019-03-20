use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Quat {
    pub w: Real,
    pub v: Vec3,
}

impl Quat {
    pub fn new(w: Real, x: Real, y: Real, z: Real) -> Quat {
        Quat {
            w,
            v: Vec3 { x, y, z },
        }
    }

    pub fn identity() -> Quat {
        Quat {
            w: 1.0,
            v: Vec3::zero(),
        }
    }

    pub fn dot(lhs: Quat, rhs: Quat) -> Real {
        lhs.w * rhs.w + Vec3::dot(lhs.v, rhs.v)
    }

    fn magnitude_squared(&self) -> Real {
        Self::dot(*self, *self)
    }

    pub fn magnitude(&self) -> Real {
        self.magnitude_squared().sqrt()
    }

    pub fn conjugate(&self) -> Quat {
        Quat {
            w: self.w,
            v: -self.v,
        }
    }

    pub fn inverse(&self) -> Quat {
        let inv_fact = 1.0 / self.magnitude();
        self.conjugate() * inv_fact
    }

    pub fn normalized(&self) -> Quat {
        let scale = 1.0 / self.magnitude();
        Quat {
            w: self.w * scale,
            v: self.v * scale,
        }
    }

    pub fn is_pure(&self) -> bool {
        self.w.approx_eq(0.0, DEF_F32_EPSILON)
    }

    pub fn is_unit(&self) -> bool {
        self.magnitude_squared().approx_eq(1.0, DEF_F32_EPSILON)
    }

    pub fn is_pure_unit(&self) -> bool {
        self.is_pure() && self.is_unit()
    }


    pub fn pow(&self, exponent: Real) -> Quat {
        if self.w.abs() < 0.9999 {
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

    //Performs a rotation around the cardinal axes, in the order BPH (handy for camera rotation)
    pub fn from_euler_obj_upr_rad(pitch: Real, heading: Real, bank: Real) -> Quat {
        let pitch = pitch / 2.0;
        let heading = heading / 2.0;
        let bank = bank / 2.0;

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

    pub fn from_euler_obj_upr_deg(pitch: Real, heading: Real, bank: Real) -> Quat {
        Self::from_euler_obj_upr_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }

    //Performs a rotation around the cardinal axes, in the order BPH (handy for camera rotation)
    pub fn from_euler_upr_obj_rad(pitch: Real, heading: Real, bank: Real) -> Quat {
        let pitch = pitch / 2.0;
        let heading = heading / 2.0;
        let bank = bank / 2.0;

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

    pub fn from_euler_upr_obj_deg(pitch: Real, heading: Real, bank: Real) -> Quat {
        Self::from_euler_upr_obj_rad(pitch.to_radians(), heading.to_radians(), bank.to_radians())
    }

    pub fn get_euler_angles_obj_upr_rad(&self) -> Vec3 {
        let sin_pitch = -2.0 * (self.v.y * self.v.z - self.w * self.v.x);

        if sin_pitch.abs() > 0.9999 {
            Vec3 {
                x: std::f32::consts::FRAC_PI_2 * sin_pitch,
                y: (-self.v.x * self.v.z + self.w * self.v.y).atan2(0.5 - self.v.y * self.v.y - self.v.z * self.v.z),
                z: 0.0,
            }
        } else {
            Vec3 {
                x: sin_pitch.asin(),
                y: (self.v.x * self.v.z + self.w * self.v.y).atan2(0.5 - self.v.x * self.v.x - self.v.y * self.v.y),
                z: (self.v.x * self.v.y + self.w * self.v.z).atan2(0.5 - self.v.x * self.v.x - self.v.z * self.v.z),
            }
        }
    }

    pub fn get_euler_angles_obj_upr_deg(&self) -> Vec3 {
        self.get_euler_angles_obj_upr_rad() * f32::to_degrees(1.0)
    }

    pub fn get_euler_angles_upr_obj_rad(&self) -> Vec3 {
        let sin_pitch = -2.0 * (self.v.y * self.v.z + self.w * self.v.x);

        if sin_pitch.abs() > 0.9999 {
            Vec3 {
                x: std::f32::consts::FRAC_PI_2 * sin_pitch,
                y: (-self.v.x * self.v.z - self.w * self.v.y).atan2(0.5 - self.v.y * self.v.y - self.v.z * self.v.z),
                z: 0.0,
            }
        } else {
            Vec3 {
                x: sin_pitch.asin(),
                y: (self.v.x * self.v.z - self.w * self.v.y).atan2(0.5 - self.v.x * self.v.x - self.v.y * self.v.y),
                z: (self.v.x * self.v.y - self.w * self.v.z).atan2(0.5 - self.v.x * self.v.x - self.v.z * self.v.z),
            }
        }
    }

    pub fn get_euler_angles_upr_obj_deg(&self) -> Vec3 {
        self.get_euler_angles_upr_obj_rad() * f32::to_degrees(1.0)
    }

    //Performs a rotation around an arbitary unit axis
    pub fn from_angle_axis(n: Vec3, theta: Real) -> Quat {
        debug_assert!(n.is_unit());
        let half_theta = theta * 0.5;

        Quat {
            w: half_theta.cos(),
            v: n * half_theta.sin(),
        }
    }

    pub fn to_angle_axis(&self) -> (Vec3, Real) {
        let q = if self.w > 1.0 {
            self.normalized()
        } else {
            *self
        };

        let theta = 2.0 * q.w;
        let s = (1.0 - q.w * q.w).sqrt();
        if s < 0.001 {
            (q.v.normalized(), theta)
        } else {
            (q.v / s, theta)
        }
    }

//    //Rotates the quaternion around an arbitrary axis
//    pub fn rotate_around(&mut self, n: Vec3, theta: Real) {
//        *self *= Self::from_angle_axis(n, theta);
//    }


    pub fn slerp(&self, other: Quat, t: Real) -> Quat {
        let mut cos_omega = Self::dot(*self, other);

        let q0 = *self;
        let mut q1 = other;

        if cos_omega < 0.0 {
            q1 = -q1;
            cos_omega = -cos_omega;
        }

        let mut k0 = 0.0;
        let mut k1 = 0.0;

        if cos_omega > 0.9999 {
            k0 = 1.0 - t;
            k1 = t;
        } else {
            let sin_omega = (1.0 - cos_omega * cos_omega).sqrt();

            let omega = sin_omega.atan2(cos_omega);

            let one_over_sin_omega = 1.0 / sin_omega;

            k0 = ((1.0 - t) * omega).sin() * one_over_sin_omega;
            k1 = (t * omega).sin() * one_over_sin_omega;
        }

        Quat {
            w: q0.w * k0 + q1.w * k1,
            v: q0.v * k0 + q1.v * k1,
        }
    }

    pub fn lerp(&self, other: Quat, t: Real) -> Quat {
        let mut cos_omega = Self::dot(*self, other);

        let q0 = *self;
        let mut q1 = other;

        if cos_omega < 0.0 {
            q1 = -q1;
        }

        let one_min_t = 1.0 - t;

        Quat {
            w: q0.w * one_min_t + q1.w * t,
            v: q0.v * one_min_t + q1.v * t,
        }
    }
}

impl Not for Quat {
    type Output = Quat;

    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl Neg for Quat {
    type Output = Quat;

    fn neg(self) -> Self::Output {
        Quat {
            w: -self.w,
            v: -self.v,
        }
    }
}

impl Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Quat) -> Self::Output {
        Quat {
            w: self.w * rhs.w - Vec3::dot(self.v, rhs.v),
            v: self.w * rhs.v + rhs.w * self.v + Vec3::cross(self.v, rhs.v),
        }
    }
}

impl Mul<Real> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Real) -> Self::Output {
        Quat {
            w: self.w * rhs,
            v: self.v * rhs,
        }
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let p = Quat {
            w: 0.0,
            v: rhs,
        };
        let ps = self * p * self.inverse();
        ps.v
    }
}

impl MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, rhs: Quat) {
        let temp = *self * rhs;
        self.w = temp.w;
        self.v = temp.v;
    }
}

impl Div<Quat> for Quat {
    type Output = Quat;

    fn div(self, rhs: Quat) -> Self::Output {
        self * rhs.inverse()
    }
}

impl Div<Real> for Quat {
    type Output = Quat;

    fn div(self, rhs: Real) -> Self::Output {
        let inv = 1.0 / rhs;
        Quat {
            w: self.w * inv,
            v: self.v * inv,
        }
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Quat) -> bool {
        self.w.approx_eq(other.w, DEF_F32_EPSILON) && self.v == other.v
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} ({:.2}i {:.2}j {:.2}k)", self.w, self.v.x, self.v.y, self.v.z)
    }
}

impl From<(Real, Real, Real, Real)> for Quat {
    fn from(tuple: (Real, Real, Real, Real)) -> Self {
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

impl From<[Real; 4]> for Quat {
    fn from(arr: [Real; 4]) -> Self {
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

impl From<Mat3> for Quat {
    fn from(m: Mat3) -> Self {
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

        let biggest_val = (four_biggest_sq_m_1 + 1.0).sqrt() * 0.5;
        let mult = 0.25 / biggest_val;

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

impl Default for Quat {
    fn default() -> Self {
        Quat::identity()
    }
}