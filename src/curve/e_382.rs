use curve::edwards::*;
use field::mod_e382_105::*;

#[derive(Copy, Clone)]
pub struct E382();

impl EdwardsCurve for E382 {
    type Scalar = Mod_e382_105;

    fn d_val() -> Self::Scalar { E382_D }
}
