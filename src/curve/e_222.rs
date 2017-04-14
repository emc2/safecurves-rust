use curve::edwards::*;
use field::mod_e222_117::*;

#[derive(Copy, Clone)]
pub struct E222();

impl EdwardsCurve for E222 {
    type Scalar = Mod_e222_117;

    fn d_val() -> Self::Scalar { E222_D }
}
