use std::fmt;
use std::ops::*;
use std::str;

use super::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Quat {
    pub a: Scalar,
    pub b: Scalar,
    pub c: Scalar,
    pub d: Scalar,
}

impl Quat {
    pub fn new(a: Scalar, b: Scalar, c: Scalar, d: Scalar) -> Quat {
        Quat { a, b, c, d }
    }

    pub fn identity() -> Quat {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn inverse(&self) -> Quat {
        let inv_fact = 1.0 / (self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d);
        Self::new(self.a, -self.b, -self.c, -self.d) * inv_fact
    }

    pub fn conjugate(&self) -> Quat {
        unimplemented!()
    }
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
        Self::new(self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d,
                  self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c,
                  self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b,
                  self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a)
    }
}

impl Mul<Scalar> for Quat {
    type Output = Quat;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self::new(self.a * rhs,
                  self.b * rhs,
                  self.c * rhs,
                  self.d * rhs)
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Quat) -> bool {
        (self.a == other.a) && (self.b == other.b) && (self.c == other.c) && (self.d == other.d)
    }
}
