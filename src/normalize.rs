/// Trait for things that have an internal representation that needs
/// to be normalized prior to certain operations.
pub trait Normalize {
    /// Normalize the internal representation.
    fn normalize(&mut self);
}

pub trait NormalizeEq : Normalize {
    /// Normalize self and compare for equality.
    fn normalize_self_eq(&mut self, other: &Self) -> bool;

    /// Normalize both arguments and compare for equality.
    fn normalize_eq(&mut self, other: &mut Self) -> bool;
}
