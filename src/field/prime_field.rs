use normalize::*;
use pack::Pack;
use rand::Rand;
use std::marker::Sized;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

/// Operations on prime fields.
pub trait PrimeField : Add<i32, Output = Self> + Add<i16, Output = Self> +
    Add<i8, Output = Self> + Add<Self, Output = Self> +
    AddAssign<i32> + AddAssign<i16> + AddAssign<i8> + AddAssign<Self> +
    Div<Self, Output = Self> + DivAssign<Self> +
    MulAssign<i32> + MulAssign<i16> + MulAssign<i8> + MulAssign<Self> +
    Mul<i32, Output = Self> + Mul<i16, Output = Self> +
    Mul<i8, Output = Self> + Mul<Self, Output = Self> +
    Neg<Output = Self> + Normalize + NormalizeEq + Pack + Rand + Sized +
    SubAssign<i32> + SubAssign<i16> + SubAssign<i8> + SubAssign<Self> +
    Sub<i32, Output = Self> + Sub<i16, Output = Self> +
    Sub<i8, Output = Self> + Sub<Self, Output = Self> {
    /// Get the number of bits in the number.
    fn nbits() -> usize;

    /// Get the bit given by idx.  This normalizes the internal representation.
    fn bit(&mut self, idx: usize) -> bool;

    /// Get the bit given by idx, assuming the internal representation
    /// is already normalized.
    fn bit_normalized(&self, idx: usize) -> bool;

    /// Set every bit of this number to the given bit.
    fn fill(&mut self, bit: bool);

    /// Generate a mask consisting entirely of the given bit.
    fn filled(bit: bool) -> Self;

    /// Normalize both arguments and bitwise-and assign.
    fn normalize_bitand(&mut self, rhs: &mut Self);

    /// Normalize self and bitwise-and assign.
    fn normalize_self_bitand(&mut self, rhs: &Self);

    /// Bitwise-and assign with both arguments normalized.
    fn normalized_bitand(&mut self, rhs: &Self);

    /// Normalize both arguments and bitwise-or assign.
    fn normalize_bitor(&mut self, rhs: &mut Self);

    /// Normalize self and bitwise-or assign.
    fn normalize_self_bitor(&mut self, rhs: &Self);

    /// Bitwise-or assign with both arguments normalized.
    fn normalized_bitor(&mut self, rhs: &Self);

    /// Get the representation of the value 0.
    fn zero() -> Self;

    /// Get the representation of the value 1.
    fn one() -> Self;

    /// Get the representation of the value -1.
    fn m_one() -> Self;

    /// Get the representation of the modulus.
    fn modulus() -> Self;

    /// In-place square.
    fn square(&mut self);

    /// Functional square.
    fn squared(&self) -> Self;

    /// In-place multiplicative inverse.
    fn invert(&mut self);

    /// Functional multiplicative inverse.
    fn inverted(&self) -> Self;

    /// Legendre symbol.  This is 1 for a quadratic residue (meaning a
    /// square root value exists), and -1 otherwise.
    fn legendre(&self) -> Self;

    /// Compute the square root.  This has meaning only for quadratic
    /// residue (legendre returns 1).  Non-residues return a garbage
    /// value.
    fn sqrt(&self) -> Self;

    /// Add a single digit (represented as an i32) in-place.
    fn small_add_assign(&mut self, b: i32);

    /// Add a single digit (represented as an i32).
    fn small_add(&self, b: i32) -> Self;

    /// Subtract a single digit (represented as an i32) in place.
    fn small_sub_assign(&mut self, b: i32);

    /// Subtract a single digit (represented as an i32).
    fn small_sub(&self, b: i32) -> Self;

    /// Multiply in-place by a single digit (represented as an i32).
    fn small_mul_assign(&mut self, b: i32);

    /// Multiply by a single digit (represented as an i32).
    fn small_mul(&self, b: i32) -> Self;
}

pub trait PrimeFieldMask {
    /// Set every bit of this number to the given bit.
    fn fill(&mut self, bit: bool);

    /// Generate a mask consisting entirely of the given bit.
    fn filled(bit: bool) -> Self;
}
