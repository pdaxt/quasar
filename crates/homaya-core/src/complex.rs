//! High-performance complex number operations.
//!
//! Optimized for quantum state vector manipulation.

use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

/// A complex number with 64-bit floating point components.
///
/// Optimized for quantum computing operations where complex arithmetic
/// dominates the computation.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Complex {
    /// Real component
    pub re: f64,
    /// Imaginary component
    pub im: f64,
}

impl Complex {
    /// Zero: 0 + 0i
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

    /// One: 1 + 0i
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };

    /// Imaginary unit: 0 + 1i
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    /// Create a new complex number.
    #[inline(always)]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Create a complex number from a real value.
    #[inline(always)]
    pub const fn from_real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    /// Create a complex number from polar coordinates.
    ///
    /// # Arguments
    /// * `r` - Magnitude (radius)
    /// * `theta` - Phase angle in radians
    #[inline]
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Compute the complex conjugate.
    #[inline(always)]
    pub const fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Compute the squared magnitude (norm squared).
    ///
    /// This is faster than `abs()` when you only need to compare magnitudes.
    #[inline(always)]
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Compute the magnitude (absolute value).
    #[inline(always)]
    pub fn abs(self) -> f64 {
        self.norm_sqr().sqrt()
    }

    /// Compute the phase angle in radians.
    #[inline(always)]
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Compute the complex exponential e^(self).
    #[inline]
    pub fn exp(self) -> Self {
        let r = self.re.exp();
        Self {
            re: r * self.im.cos(),
            im: r * self.im.sin(),
        }
    }

    /// Check if this is approximately zero.
    #[inline]
    pub fn is_zero(self, epsilon: f64) -> bool {
        self.norm_sqr() < epsilon * epsilon
    }

    /// Check if this is approximately equal to another complex number.
    #[inline]
    pub fn approx_eq(self, other: Self, epsilon: f64) -> bool {
        (self - other).is_zero(epsilon)
    }
}

impl Default for Complex {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<f64> for Complex {
    #[inline(always)]
    fn from(re: f64) -> Self {
        Self::from_real(re)
    }
}

impl From<(f64, f64)> for Complex {
    #[inline(always)]
    fn from((re, im): (f64, f64)) -> Self {
        Self::new(re, im)
    }
}

// Arithmetic operations - all inlined for maximum performance

impl Add for Complex {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl AddAssign for Complex {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Sub for Complex {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl MulAssign for Complex {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.norm_sqr();
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Complex {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.re, self.im).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Complex {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (re, im) = <(f64, f64)>::deserialize(deserializer)?;
        Ok(Self::new(re, im))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);

        assert_eq!(a + b, Complex::new(4.0, 6.0));
        assert_eq!(a * b, Complex::new(-5.0, 10.0));
        assert_eq!(a.conj(), Complex::new(1.0, -2.0));
    }

    #[test]
    fn test_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!((c.abs() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_polar() {
        use crate::PI;
        let c = Complex::from_polar(1.0, PI / 4.0);
        assert!((c.re - 0.7071067811865476).abs() < 1e-10);
        assert!((c.im - 0.7071067811865476).abs() < 1e-10);
    }

    #[test]
    fn test_exp() {
        use crate::PI;
        // e^(iπ) = -1
        let c = Complex::new(0.0, PI);
        let result = c.exp();
        assert!((result.re + 1.0).abs() < 1e-10);
        assert!(result.im.abs() < 1e-10);
    }
}
