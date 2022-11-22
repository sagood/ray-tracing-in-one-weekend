use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

// use Vec3 as Point3;
// use Vec3 as Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
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

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

macro_rules! gen_binary_ops {
    ($Vector:ident $op:ident $func:ident $symbol:tt) => {
        impl $op for &$Vector {
            type Output = $Vector;

            fn $func(self, other: &$Vector) -> $Vector {
                $Vector::new(self.e[0] $symbol other.e[0], self.e[1] $symbol other.e[1], self.e[2] $symbol other.e[2])
            }
        }

        impl $op<$Vector> for $Vector {
            type Output = $Vector;

            fn $func(self, other: $Vector) -> $Vector {
                &self $symbol &other
            }
        }

        impl $op<& $Vector> for $Vector {
            type Output = $Vector;

            fn $func(self, other: & $Vector) -> $Vector {
                &self $symbol other
            }
        }

        impl $op<$Vector> for & $Vector {
            type Output = $Vector;

            fn $func(self, other: $Vector) -> $Vector {
                self $symbol &other
            }
        }

        impl $op<f64> for & $Vector {
            type Output = $Vector;

            fn $func(self, other: f64) -> $Vector {
                $Vector::new(self.e[0] $symbol other, self.e[1] $symbol other, self.e[2] $symbol other)
            }
        }

        impl $op<f64> for $Vector {
            type Output = $Vector;

            fn $func(self, other: f64) -> $Vector {
                &self $symbol other
            }
        }

        impl $op<$Vector> for f64 {
            type Output = $Vector;

            fn $func(self, other: $Vector) -> $Vector {
                &other $symbol self
            }
        }

        impl $op<&$Vector> for f64 {
            type Output = $Vector;

            fn $func(self, other: &$Vector) -> $Vector {
                other $symbol self
            }
        }
    };
}

macro_rules! gen_unary_ops {
    ($Vector:ident $op:ident $func:ident $symbol:tt) => {
        impl $op for & $Vector {
            type Output = $Vector;

            fn $func(self) -> $Vector {
                $Vector::new($symbol self.e[0], $symbol self.e[1], $symbol self.e[2])
            }
        }

        impl $op for $Vector {
            type Output = $Vector;

            fn $func(self) -> $Vector {
                $symbol &self
            }
        }
    };
}

macro_rules! gen_op_assign {
    ($Vector:ident $op:ident $func:ident $symbol:tt) => {
        impl $op<& $Vector> for $Vector {
            fn $func(&mut self, other: & $Vector) {
                self.e[0] $symbol other.e[0];
                self.e[1] $symbol other.e[1];
                self.e[2] $symbol other.e[2];
            }
        }

        impl $op for $Vector {
            fn $func(&mut self, other: $Vector) {
                self.e[0] $symbol &other.e[0];
                self.e[1] $symbol &other.e[1];
                self.e[2] $symbol &other.e[2];
            }
        }

        impl $op<f64> for $Vector {
            fn $func(&mut self, other: f64) {
                self.e[0] $symbol other;
                self.e[1] $symbol other;
                self.e[2] $symbol other;
            }
        }
    };
}

gen_binary_ops!(Vec3 Add add +);
gen_binary_ops!(Vec3 Sub sub -);
gen_binary_ops!(Vec3 Mul mul *);
gen_binary_ops!(Vec3 Div div /);

gen_unary_ops!(Vec3 Neg neg -);

gen_op_assign!(Vec3 AddAssign add_assign +=);
gen_op_assign!(Vec3 SubAssign sub_assign -=);
gen_op_assign!(Vec3 MulAssign mul_assign *=);
gen_op_assign!(Vec3 DivAssign div_assign /=);

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
    fn test_vec3_addassign_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(5.0, a.x());
        assert_eq!(7.0, a.y());
        assert_eq!(9.0, a.z());

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += &b;
        assert_eq!(5.0, a.x());
        assert_eq!(7.0, a.y());
        assert_eq!(9.0, a.z());
    }

    #[test]
    fn test_vec3_subassign_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a -= b;
        assert_eq!(-3.0, a.x());
        assert_eq!(-3.0, a.y());
        assert_eq!(-3.0, a.z());

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a -= &b;
        assert_eq!(-3.0, a.x());
        assert_eq!(-3.0, a.y());
        assert_eq!(-3.0, a.z());
    }

    #[test]
    fn test_vec3_multiplyassign_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a *= b;
        assert_eq!(4.0, a.x());
        assert_eq!(10.0, a.y());
        assert_eq!(18.0, a.z());

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a *= &b;
        assert_eq!(4.0, a.x());
        assert_eq!(10.0, a.y());
        assert_eq!(18.0, a.z());
    }

    #[test]
    fn test_vec3_divideassign_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let mut b = Vec3::new(4.0, 5.0, 6.0);
        b /= a;
        assert_eq!(4.0, b.x());
        assert_eq!(2.5, b.y());
        assert_eq!(2.0, b.z());

        let a = Vec3::new(1.0, 2.0, 3.0);
        let mut b = Vec3::new(4.0, 5.0, 6.0);
        b /= &a;
        assert_eq!(4.0, b.x());
        assert_eq!(2.5, b.y());
        assert_eq!(2.0, b.z());
    }

    #[test]
    fn test_vec3_multiplyassign_f64() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a *= b;
        assert_eq!(2.0, a.x());
        assert_eq!(4.0, a.y());
        assert_eq!(6.0, a.z());
    }

    #[test]
    fn test_vec3_divideassign_f64() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a /= b;
        assert_eq!(0.5, a.x());
        assert_eq!(1.0, a.y());
        assert_eq!(1.5, a.z());
    }
}
