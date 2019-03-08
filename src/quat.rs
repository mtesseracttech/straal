use std::fmt;
use std::ops::*;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Quat {
    pub w: Real,
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Quat {
    pub fn new(w: Real, x: Real, y: Real, z: Real) -> Quat {
        Quat { w, x, y, z }
    }

    pub fn identity() -> Quat {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn dot(lhs: &Quat, rhs: &Quat) -> Real {
        lhs.w * rhs.w + lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    fn length_squared(&self) -> Real {
        Self::dot(self, self)
    }

    pub fn length(&self) -> Real {
        self.length_squared().sqrt()
    }

    pub fn conjugate(&self) -> Quat {
        Self::new(self.w,
                  -self.x,
                  -self.y,
                  -self.z)
    }

    pub fn inverse(&self) -> Quat {
        let inv_fact = 1.0 / Self::dot(self, self);
        self.conjugate() * inv_fact
    }

    pub fn is_pure(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_pure_unit(&self) -> bool {
        self.is_pure() && self.length_squared() == 1.0
    }

    pub fn from_euler_deg_zxy(angles: Vec3) -> Quat {
        const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
        Self::from_euler_rad_zxy(angles * DEG_TO_RAD)
    }


    //Euler angles to rad in zxy order
    pub fn from_euler_rad_zxy(angles: Vec3) -> Quat {
        let angles = angles / 2.0;

        let cx = angles.x.cos();
        let sx = angles.x.sin();
        let cy = angles.y.cos();
        let sy = angles.y.sin();
        let cz = angles.z.cos();
        let sz = angles.z.sin();

        Self::new(cz * cx * cy - sz * sx * sy,
                  cz * sx * cy - sz * cx * sy,
                  cz * cx * sy + sz * sx * cy,
                  cz * sx * sy + sz * cx * cy)
    }

    pub fn to_euler_deg_zxy(&self) -> Vec3 {
        const RAD_TO_DEG: f32 = 180.0 / std::f32::consts::PI;
        self.to_euler_rad_zxy() * RAD_TO_DEG
    }

    pub fn to_euler_rad_zxy(&self) -> Vec3 {
        let sine_pitch = 2.0 * (self.w * self.x + self.y * self.z);
        if sine_pitch.abs() > 0.9999 {
            println!("using the sp abs line");
            let x = std::f32::consts::FRAC_PI_2 * sine_pitch;
            let y = 0.0;
            let z = (self.x * self.z + self.w * self.y).atan2(0.5 - self.y * self.y - self.z * self.z);
            Vec3 { x, y, z }
        } else {
            let x = sine_pitch.asin();
            let y = (-self.x * self.z + self.w * self.y).atan2(0.5 - self.x * self.x - self.y * self.y);
            let z = (-self.x * self.y + self.z * self.w).atan2(0.5 - self.x * self.x - self.z * self.z);
            Vec3 { x, y, z }
        }
    }

    //TODO: SLERP
    pub fn slerp(&self, other: Quat) -> Quat { unimplemented!() }
}

impl Not for Quat {
    type Output = Quat;

    fn not(self) -> Self::Output {
        self.inverse()
    }
}

impl Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Quat) -> Self::Output {
        Self::new(self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
                  self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
                  self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
                  self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w)
    }
}

impl Mul<Real> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Real) -> Self::Output {
        Self::new(self.w * rhs,
                  self.x * rhs,
                  self.y * rhs,
                  self.z * rhs)
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
        Self::new(self.w * inv,
                  self.x * inv,
                  self.y * inv,
                  self.z * inv)
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Quat) -> bool {
        self.w.approx_eq(other.w, DEF_F32_EPSILON) && self.x.approx_eq(other.x, DEF_F32_EPSILON) &&
            self.y.approx_eq(other.y, DEF_F32_EPSILON) && self.z.approx_eq(other.z, DEF_F32_EPSILON)
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} ({:.2}i {:.2}j {:.2}k)", self.w, self.x, self.y, self.z)
    }
}

impl From<(Real, Real, Real, Real)> for Quat {
    fn from(tuple: (Real, Real, Real, Real)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}

impl From<[Real; 4]> for Quat {
    fn from(arr: [Real; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }
}