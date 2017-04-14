use curve::edwards::*;
use field::mod_e521_1::*;

#[derive(Copy, Clone)]
pub struct E521();

impl EdwardsCurve for E521 {
    type Scalar = Mod_e521_1;

    fn d_val() -> Self::Scalar { E521_D }
}
