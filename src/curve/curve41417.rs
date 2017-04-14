use curve::edwards::*;
use field::mod_e414_17::*;

#[derive(Copy, Clone)]
pub struct Curve41417();

impl EdwardsCurve for Curve41417 {
    type Scalar = Mod_e414_17;

    fn d_val() -> Self::Scalar { CURVE41417_D }
}
