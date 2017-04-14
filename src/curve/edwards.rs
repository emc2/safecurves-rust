use curve::point::*;
use field::prime_field::PrimeField;
use normalize::Normalize;
use std::ops::Add;
use std::ops::AddAssign;

pub trait EdwardsCurve : Copy {
    type Scalar : PrimeField + Copy;

    fn d_val() -> Self::Scalar;
}

/// Edwards curve point in homogeneous extended coordinates.
#[derive(Copy, Clone)]
pub struct EdwardsExtended<C : EdwardsCurve> {
    x : C::Scalar,
    y : C::Scalar,
    z : C::Scalar,
    t : C::Scalar
}

impl<'b, C : EdwardsCurve> AddAssign<&'b EdwardsExtended<C>>
    for EdwardsExtended<C> {
    fn add_assign(&mut self, rhs: &'b EdwardsExtended<C>) {
        let a = self.x * rhs.x;
        let b = self.y * rhs.y;
        let c = self.t * C::d_val() * rhs.t;
        let d = self.z * rhs.z;
        let e = ((self.x + self.y) * (rhs.x + rhs.y)) - a - b;
        let f = d - c;
        let g = d + c;
        let h = b - a;
        let x3 = e * f;
        let y3 = g * h;
        let z3 = f * g;
        let t3 = e * h;

        self.x = x3;
        self.y = y3;
        self.z = z3;
        self.t = t3;
    }
}

impl<C : EdwardsCurve> AddAssign<EdwardsExtended<C>>
    for EdwardsExtended<C> {
    fn add_assign(&mut self, rhs: EdwardsExtended<C>) {
        *self += &rhs;
    }
}

impl<'a, 'b, C : EdwardsCurve> Add<&'b EdwardsExtended<C>>
    for &'a EdwardsExtended<C> {
    type Output = EdwardsExtended<C>;

    fn add(self, rhs: &'b EdwardsExtended<C>) -> EdwardsExtended<C> {
        let mut out = self.clone();
        out += rhs;
        out
    }
}

impl<C : EdwardsCurve> Add<EdwardsExtended<C>> for EdwardsExtended<C> {
    type Output = EdwardsExtended<C>;

    fn add(self, rhs: EdwardsExtended<C>) -> EdwardsExtended<C> {
        &self + &rhs
    }
}

impl<C: EdwardsCurve> EdwardsExtended<C> {
    fn normalized_bitand(&mut self, mask: &C::Scalar) {
        self.x.normalized_bitand(mask);
        self.y.normalized_bitand(mask);
        self.z.normalized_bitand(mask);
        self.t.normalized_bitand(mask);
    }

    fn normalized_bitor(&mut self, rhs: &Self) {
        self.x.normalized_bitor(&rhs.x);
        self.y.normalized_bitor(&rhs.y);
        self.z.normalized_bitor(&rhs.z);
        self.t.normalized_bitor(&rhs.t);
    }
}

impl<C: EdwardsCurve> Point for EdwardsExtended<C> {
    type Scalar = C::Scalar;

    fn zero() -> Self {
        EdwardsExtended {
            x: C::Scalar::zero(),
            y: C::Scalar::one(),
            z: C::Scalar::one(),
            t: C::Scalar::one()
        }
    }

    fn init(&mut self, x: C::Scalar, y: C::Scalar) {
        self.x = x;
        self.y = y;
        self.z = C::Scalar::one();
        self.t = C::Scalar::one();
    }

    fn scalar_mult(&mut self, rhs: &mut Self::Scalar) {
        rhs.normalize();
        self.scalar_mult_normalized(rhs);
    }

    fn scalar_mult_normalized(&mut self, rhs: &Self::Scalar) {
        // This is a branchless variant of the classic Montgomery
        // ladder using bitwise operations in lieu of branches.

        let mut r0 = Self::zero();
        let mut r1 = self.clone();
        let nbits = C::Scalar::nbits();

        for i in 0..nbits {
            let idx = nbits - 1 - i;
            let bit = rhs.bit_normalized(idx);
            // Note that we fill everything with 0s or 1s.  This
            // allows us to skip normalization.
            let tmask = C::Scalar::filled(bit);
            let fmask = C::Scalar::filled(!bit);
            let mut d0 = r0;
            let mut d1 = r1;

            // r0 = r1 = r0 + r1
            r0 += r1;
            r1 = r0;

            // d0 = bit ? r0 : r1
            d0.normalized_bitand(&tmask);
            d1.normalized_bitand(&fmask);
            d0.normalized_bitor(&d1);

            // d0 *= 2
            d0.double();

            // d1 = d0
            d1 = d0;

            // r0 = bit ? d0 : r0
            r0.normalized_bitand(&fmask);
            d0.normalized_bitand(&tmask);
            r0.normalized_bitor(&d0);

            // r1 = bit ? r1 : d1
            r1.normalized_bitand(&tmask);
            d1.normalized_bitand(&fmask);
            r1.normalized_bitor(&d1);
        }

        self.x = r0.x;
        self.y = r0.y;
        self.z = r0.z;
        self.t = r0.t;
    }

    fn double(&mut self) {
        let a = self.x.squared();
        let b = self.y.squared();
        let c = self.z.squared().small_mul(2);
        let d = a;
        let e = (self.x + self.y).squared() - a - b;
        let g = d + b;
        let f = g - c;
        let h = d - b;
        let x3 = e * f;
        let y3 = g * h;
        let z3 = f * g;
        let t3 = e * h;

        self.x = x3;
        self.y = y3;
        self.z = z3;
        self.t = t3;
    }

    fn doubled(&self) -> Self {
        let mut out = self.clone();
        out.double();
        out
    }

    fn triple(&mut self) {
        let yy = self.y.squared();
        let xx = self.x.squared();
        let ap = yy + xx;
        let b = (self.z.squared().small_mul(2) - ap).small_mul(2);
        let xb = xx * b;
        let yb = yy * b;
        let aa = ap * (yy - xx);
        let f = aa - yb;
        let g = aa - xb;
        let xe = self.x * (yb + aa);
        let yh = self.y * (xb - aa);
        let zf = self.z * f;
        let zg = self.z * g;
        let x3 = xe * zf;
        let y3 = yh * zg;
        let z3 = zf * zg;
        let t3 = xe * yh;

        self.x = x3;
        self.y = y3;
        self.z = z3;
        self.t = t3;
    }

    fn tripled(&self) -> Self {
        let mut out = self.clone();
        out.triple();
        out
    }
}
