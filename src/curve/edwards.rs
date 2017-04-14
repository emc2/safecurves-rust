use curve::point::*;
use field::prime_field::PrimeField;
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

impl<C: EdwardsCurve> Point for EdwardsExtended<C> {
    type Scalar = C::Scalar;

    fn init(&mut self, x: Self::Scalar, y: Self::Scalar) {
        self.x = x;
        self.y = y;
        self.z = C::Scalar::one();
        self.t = C::Scalar::one();
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
