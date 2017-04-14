use field::prime_field::PrimeField;
use std::ops::Add;
use std::ops::AddAssign;

/// Operations on elliptic curve points.
pub trait Point : Add<Self, Output = Self> + AddAssign<Self> + Copy {
    type Scalar : PrimeField;

    fn zero() -> Self;

    /// In-place constructor for points from x-y coordinates.
    fn init(&mut self, x: Self::Scalar, y: Self::Scalar);

    /// Scalar multiplication.
    fn scalar_mult(&mut self, rhs: &mut Self::Scalar);

    /// Scalar multiplication by a normalized scalar.
    fn scalar_mult_normalized(&mut self, rhs: &Self::Scalar);

    /// Double a point in place.
    fn double(&mut self);

    /// Functional doubling.
    fn doubled(&self) -> Self;

    /// Triple a point in place.
    fn triple(&mut self);

    /// Functional tripling.
    fn tripled(&self) -> Self;
}

pub trait Compressed<T> {
    fn compress(&mut self, T);
    fn compressed(T) -> Self;
    fn decompress(&self, &mut T);
    fn decomressed(&self) -> T;
}
