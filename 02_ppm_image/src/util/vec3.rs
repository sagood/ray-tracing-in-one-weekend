use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

// use Vec3 as Point3;
// use Vec3 as Color;

struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    fn x(&self) -> f64 {
        self.e[0]
    }

    fn y(&self) -> f64 {
        self.e[1]
    }

    fn z(&self) -> f64 {
        self.e[2]
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vec3() {
        let a = Vec3::default();
        assert_eq!(0.0, a.x());
        assert_eq!(0.0, a.y());
        assert_eq!(0.0, a.z());
    }

    #[test]
    fn test_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, a.x());
        assert_eq!(2.0, a.y());
        assert_eq!(3.0, a.z());
        assert_eq!(14.0, a.length_squared());
        assert_eq!(14.0f64.sqrt(), a.length());
    }

    #[test]
    fn test_vec3_neg() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a = -a;
        assert_eq!(-1.0, a.x());
        assert_eq!(-2.0, a.y());
        assert_eq!(-3.0, a.z());
    }


    #[test]
    fn test_vec3_add_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(5.0, a.x());
        assert_eq!(7.0, a.y());
        assert_eq!(9.0, a.z());
    }

    #[test]
    fn test_vec3_multiply_f64() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a *= b;
        assert_eq!(2.0, a.x());
        assert_eq!(4.0, a.y());
        assert_eq!(6.0, a.z());
    }

    #[test]
    fn test_vec3_divide_f64() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a /= b;
        assert_eq!(0.5, a.x());
        assert_eq!(1.0, a.y());
        assert_eq!(1.5, a.z());
    }
}
