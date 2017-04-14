use curve::point::Point;
use field::prime_field::PrimeField;

pub trait Group<P : Point> {
    fn base() -> P;

    fn order() -> P::Scalar;

    fn cofactor() -> i32;
}
