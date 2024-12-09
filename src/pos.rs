use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_arithmetic_traits {
    ($t:ident, [$($var:ident),* $(,)?]) => {
        impl<T: Copy + Add<Output = T>> Add for $t<T> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $var: self.$var + rhs.$var,
                    )*
                }
            }
        }

        impl<T: Copy + AddAssign> AddAssign for $t<T> {
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$var += rhs.$var;
                )*
            }
        }

        impl<T: Copy + Sub<Output = T>> Sub for $t<T> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $(
                        $var: self.$var - rhs.$var,
                    )*
                }
            }
        }

        impl<T: Copy + SubAssign> SubAssign for $t<T> {
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$var -= rhs.$var;
                )*
            }
        }

        impl<T: Copy + Mul<Output = T>> Mul<T> for $t<T> {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                Self {
                    $(
                        $var: self.$var * rhs,
                    )*
                }
            }
        }

        impl<T: Copy + MulAssign> MulAssign<T> for $t<T> {
            fn mul_assign(&mut self, rhs: T) {
                $(
                    self.$var *= rhs;
                )*
            }
        }
    }
}

impl_arithmetic_traits!(Pos2, [x, y]);
impl_arithmetic_traits!(Pos3, [x, y, z]);
