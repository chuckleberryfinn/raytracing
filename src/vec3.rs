use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::rtweekend::{random_float, random_float_range};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_float_range(min, max),
            random_float_range(min, max),
            random_float_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        match on_unit_sphere.dot(normal) > 0.0 {
            true => on_unit_sphere,
            false => -on_unit_sphere,
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cost_theta = 1.0_f64.min(-uv.dot(n));
        let r_out_perp = etai_over_etat * (uv + cost_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.x();
        self.e[1] /= rhs.y();
        self.e[2] /= rhs.z();
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.x();
        self.e[1] *= rhs.y();
        self.e[2] *= rhs.z();
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

pub type Point3 = Vec3;

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(Vec3::new(1.0, 2.0, 3.0) + Vec3::new(1.0, 2.0, 3.0) == Vec3::new(2.0, 4.0, 6.0))
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        v += v1;
        assert!(v == Vec3::new(2.0, 4.0, 6.0))
    }

    #[test]
    fn test_length_squared() {
        assert!(Vec3::new(1.0, 2.0, 3.0).length_squared() == 14.0)
    }

    #[test]
    fn test_length() {
        assert!(Vec3::new(5.0, 5.0, 5.0).length() == 8.660254037844387)
    }

    #[test]
    fn test_dot() {
        assert!(Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(2.0, 3.0, 4.0)) == 20.0);
        assert!(Vec3::new(2.0, 3.0, 4.0).dot(Vec3::new(1.0, 2.0, 3.0)) == 20.0);
        assert!(2.0 * Vec3::new(2.0, 3.0, 4.0).dot(Vec3::new(1.0, 2.0, 3.0)) == 40.0);
        assert!(Vec3::new(2.0, 3.0, 4.0).dot(Vec3::new(1.0, 2.0, 3.0)) - 3.0 * 3.0 == 11.0);
    }

    #[test]
    fn test_div() {
        assert!(Vec3::new(2.0, 4.0, 6.0) / 2.0 == Vec3::new(1.0, 2.0, 3.0))
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        let v1 = Vec3::new(2.0, 2.0, 2.0);
        v /= v1;
        assert!(v == Vec3::new(1.0, 2.0, 3.0))
    }

    #[test]
    fn test_mul() {
        assert!(Vec3::new(2.0, 4.0, 6.0) * 2.0 == Vec3::new(4.0, 8.0, 12.0))
    }

    #[test]
    fn test_mul_swap() {
        assert!(2.0 * Vec3::new(2.0, 4.0, 6.0) == Vec3::new(4.0, 8.0, 12.0))
    }

    #[test]
    fn test_mul_vec() {
        assert!(Vec3::new(2.0, 4.0, 6.0) * Vec3::new(2.0, 2.0, 2.0) == Vec3::new(4.0, 8.0, 12.0))
    }

    #[test]
    fn test_sub() {
        assert!(Vec3::new(2.0, 4.0, 6.0) - Vec3::new(2.0, 2.0, 2.0) == Vec3::new(0.0, 2.0, 4.0));
        assert!(Vec3::new(2.0, 2.0, 2.0) - Vec3::new(2.0, 4.0, 6.0) == Vec3::new(0.0, -2.0, -4.0));
    }

    #[test]
    fn test_neg() {
        assert!(-Vec3::new(2.0, 4.0, 6.0) == Vec3::new(-2.0, -4.0, -6.0))
    }
}
