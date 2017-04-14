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
    Neg<Output = Self> + Pack + Rand + Sized +
    SubAssign<i32> + SubAssign<i16> + SubAssign<i8> + SubAssign<Self> +
    Sub<i32, Output = Self> + Sub<i16, Output = Self> +
    Sub<i8, Output = Self> + Sub<Self, Output = Self> {
    /// Get the number of bits in the number.
    fn nbits() -> i32;

    /// Get the bit given by idx.  This normalizes the internal representation.
    fn bit(&mut self, idx: usize) -> bool;

    /// Get the bit given by idx, assuming the internal representation
    /// is already normalized.
    fn bit_normalized(&self, idx: usize) -> bool;

    /// Set every bit of this number to the given bit.
    fn fill(&mut self, bit: bool);

    /// Generate a mask consisting entirely of the given bit.
    fn filled(bit: bool) -> Self;

    /// Normalize the internal representation, resulting in the
    /// internal digits holding a value that is truly less than the
    /// modulus.
    ///
    /// This can be done n mod (2^m - c) using a single add and small
    /// multiply as follows: we can detect overflow by doing
    /// carry_out(n + c), thus, we can normalize the number by doing
    /// n - (carry_out(n + c) * (2^m - c))
    fn normalize(&mut self);

    /// Normalize self and compare for equality.
    fn normalize_self_eq(&mut self, other: &Self) -> bool;

    /// Normalize both arguments and compare for equality.
    fn normalize_eq(&mut self, other: &mut Self) -> bool;

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
