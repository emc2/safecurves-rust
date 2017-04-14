use curve::edwards::*;
use field::mod_e251_9::*;

#[derive(Copy, Clone)]
pub struct Curve1174();

impl EdwardsCurve for Curve1174 {
    type Scalar = Mod_e251_9;

    fn d_val() -> Self::Scalar { CURVE1174_D }
}
