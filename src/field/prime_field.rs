/// Operations on prime fields.
pub trait PrimeField {
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
