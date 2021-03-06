use field::prime_field::PrimeField;
use field::prime_field::PrimeFieldMask;
use normalize::Normalize;
use normalize::NormalizeEq;
use pack::Pack;
use rand::Rand;
use rand::Rng;
use std::clone::Clone;
use std::fmt::Debug;
use std::fmt::LowerHex;
use std::fmt::UpperHex;
use std::ops::AddAssign;
use std::ops::Add;
use std::ops::DivAssign;
use std::ops::Div;
use std::ops::MulAssign;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::SubAssign;
use std::ops::Sub;

/// Elements of the finite field mod 2^382 - 105.  Used by the E-382
/// curve.
///
/// This is represented using fourteen 28-bit digits, stored in a
/// seven-element i64 array with two digits per word.  This combined
/// representation allows many operations to be faster.  The leftover
/// bits in each digit are used to capture carry values.  The internal
/// representation is lazily normalized: it may leave carry values in
/// the highest-order digit, and it may hold a value greater than the
/// modulus.  All operations are guaranteed to work on non-normal values
/// of this kind.

#[derive(Copy, Clone)]
pub struct Mod_e382_105([i64; 7]);

/// Bitmasks corresponding to Mod_e382_105.
#[derive(Copy, Clone)]
pub struct Mod_e382_105_Mask([i64; 7]);

const C_VAL: i64 = 105;

/// The normalized representation of the value 0.
pub const ZERO: Mod_e382_105 = Mod_e382_105([ 0, 0, 0, 0, 0, 0, 0 ]);

/// The normalized representation of the value 1.
pub const ONE: Mod_e382_105 = Mod_e382_105([ 1, 0, 0, 0, 0, 0, 0 ]);

/// The normalized representation of the value -1.
pub const M_ONE: Mod_e382_105 =
    Mod_e382_105([ 0x00ffffffffffff96, 0x00ffffffffffffff,
                   0x00ffffffffffffff, 0x00ffffffffffffff,
                   0x00ffffffffffffff, 0x00ffffffffffffff,
                   0x00003fffffffffff ]);

/// The normalized representation of the modulus 2^382 - 105.
pub const MODULUS: Mod_e382_105 =
    Mod_e382_105([ 0x00ffffffffffff97, 0x00ffffffffffffff,
                   0x00ffffffffffffff, 0x00ffffffffffffff,
                   0x00ffffffffffffff, 0x00ffffffffffffff,
                   0x00003fffffffffff ]);

/// The d value -67254 in the Edwards curve equation x^2 + y^2 = 1 + dx^2 y^2
/// for E-382.
pub const E382_D: Mod_e382_105 =
    Mod_e382_105([ 0x00fffffffffef8e1, 0x00ffffffffffffff,
                   0x00ffffffffffffff, 0x00ffffffffffffff,
                   0x00ffffffffffffff, 0x00ffffffffffffff,
                   0x00003fffffffffff ]);

/// The x-coordinate of the base point of the curve E-382,
/// 0x196f8dd0eab20391e5f05be96e8d20ae68f840032b0b64352923bab85364841193517dbce8105398ebc0cc9470f79603.
pub const E382_BASE_X: Mod_e382_105 =
    Mod_e382_105([ 0x00c0cc9470f79603, 0x007dbce8105398eb,
                   0x00b8536484119351, 0x002b0b64352923ba,
                   0x008d20ae68f84003, 0x000391e5f05be96e,
                   0x0000196f8dd0eab2 ]);

/// The y-coordinate of the base point of the curve E-382, 0x11.
pub const E382_BASE_Y: Mod_e382_105 = Mod_e382_105([ 0x11, 0, 0, 0, 0, 0, 0 ]);

/// The prime subgroup order of the curve E-382,
/// 2^380 - 1030303207694556153926491950732314247062623204330168346855
pub const E382_ORDER: Mod_e382_105 =
    Mod_e382_105([ 0x00e27e13fd3c9719, 0x0069281b102d2773,
                   0x00f21e95eee17c5e, 0x00ffffffffd5fb21,
                   0x00ffffffffffffff, 0x00ffffffffffffff,
                   0x00000fffffffffff ]);

impl Debug for Mod_e382_105 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "Mod_e382_105: [ {:x}", &self.0[0]));

        for i in 1..7 {
            try!(write!(f, ", {:x}", &self.0[i]));
        }

        write!(f, " ]")
    }
}

impl LowerHex for Mod_e382_105 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.packed();

        for i in 0..48 {
            try!(write!(f, "{:02x}", bytes[47 - i]));
        }

        Ok(())
    }
}

impl UpperHex for Mod_e382_105 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.packed();

        for i in 0..48 {
            try!(write!(f, "{:02X}", bytes[47 - i]));
        }

        Ok(())
    }
}

impl Mod_e382_105 {
    /// Get the carry-in value.  We use the highest carry slot to
    /// stash the carry-out value of each operation, and feed that
    /// back into the next one.
    fn carry_out(&self) -> i64 {
        self.0[6] >> 46
    }

    /// Serialize a value as a little-endian byte array.  This has the
    /// effect of normalizing the representation.
    pub fn packed(&mut self) -> [u8; 48] {
        let mut out = [0u8; 48];
        self.pack(&mut out);
        out
    }
}

impl Neg for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn neg(self) -> Mod_e382_105 {
        let mut out = self;

        out += &MODULUS;
        out
    }
}

impl<'b> AddAssign<&'b Mod_e382_105> for Mod_e382_105 {
    fn add_assign(&mut self, rhs: &'b Mod_e382_105) {
        let a0 = self.0[0];
        let a1 = self.0[1];
        let a2 = self.0[2];
        let a3 = self.0[3];
        let a4 = self.0[4];
        let a5 = self.0[5];
        let a6 = self.0[6] & 0x00003fffffffffff;

        let b0 = rhs.0[0];
        let b1 = rhs.0[1];
        let b2 = rhs.0[2];
        let b3 = rhs.0[3];
        let b4 = rhs.0[4];
        let b5 = rhs.0[5];
        let b6 = rhs.0[6] & 0x00003fffffffffff;

        let cin = self.carry_out() + rhs.carry_out();
        let s0 = a0 + b0 + (cin * C_VAL);
        let c0 = s0 >> 56;
        let s1 = a1 + b1 + c0;
        let c1 = s1 >> 56;
        let s2 = a2 + b2 + c1;
        let c2 = s2 >> 56;
        let s3 = a3 + b3 + c2;
        let c3 = s3 >> 56;
        let s4 = a4 + b4 + c3;
        let c4 = s4 >> 56;
        let s5 = a5 + b5 + c4;
        let c5 = s5 >> 56;
        let s6 = a6 + b6 + c5;

        self.0[0] = s0 & 0x00ffffffffffffff;
        self.0[1] = s1 & 0x00ffffffffffffff;
        self.0[2] = s2 & 0x00ffffffffffffff;
        self.0[3] = s3 & 0x00ffffffffffffff;
        self.0[4] = s4 & 0x00ffffffffffffff;
        self.0[5] = s5 & 0x00ffffffffffffff;
        self.0[6] = s6;
    }
}

impl AddAssign<Mod_e382_105> for Mod_e382_105 {
    fn add_assign(&mut self, rhs: Mod_e382_105) {
        *self += &rhs;
    }
}

impl AddAssign<i32> for Mod_e382_105 {
    fn add_assign(&mut self, rhs: i32) {
        self.small_add_assign(rhs);
    }
}

impl AddAssign<i16> for Mod_e382_105 {
    fn add_assign(&mut self, rhs: i16) {
        self.small_add_assign(rhs as i32);
    }
}

impl AddAssign<i8> for Mod_e382_105 {
    fn add_assign(&mut self, rhs: i8) {
        self.small_add_assign(rhs as i32);
    }
}

impl<'a, 'b> Add<&'b Mod_e382_105> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: &'b Mod_e382_105) -> Mod_e382_105 {
        let mut out = self.clone();
        out += a;
        out
    }
}

impl Add<Mod_e382_105> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: Mod_e382_105) -> Mod_e382_105 {
        &self + &a
    }
}

impl<'a> Add<&'a Mod_e382_105> for i32 {
    type Output = Mod_e382_105;

    fn add(self, a: &'a Mod_e382_105) -> Mod_e382_105 {
        a.small_add(self)
    }
}

impl<'a> Add<&'a Mod_e382_105> for i16 {
    type Output = Mod_e382_105;

    fn add(self, a: &'a Mod_e382_105) -> Mod_e382_105 {
        a.small_add(self as i32)
    }
}

impl<'a> Add<&'a Mod_e382_105> for i8 {
    type Output = Mod_e382_105;

    fn add(self, a: &'a Mod_e382_105) -> Mod_e382_105 {
        a.small_add(self as i32)
    }
}

impl<'a> Add<i32> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: i32) -> Mod_e382_105 {
        self.small_add(a)
    }
}

impl<'a> Add<i16> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: i16) -> Mod_e382_105 {
        self.small_add(a as i32)
    }
}

impl<'a> Add<i8> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: i8) -> Mod_e382_105 {
        self.small_add(a as i32)
    }
}

impl Add<i32> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: i32) -> Mod_e382_105 {
        &self + a
    }
}

impl Add<i16> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: i16) -> Mod_e382_105 {
        &self + a
    }
}

impl Add<i8> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn add(self, a: i8) -> Mod_e382_105 {
        &self + a
    }
}

impl<'b> DivAssign<&'b Mod_e382_105> for Mod_e382_105 {
    fn div_assign(&mut self, rhs: &'b Mod_e382_105) {
        *self *= &rhs.inverted();
    }
}

impl DivAssign<Mod_e382_105> for Mod_e382_105 {
    fn div_assign(&mut self, rhs: Mod_e382_105) {
        *self /= &rhs;
    }
}

impl<'a, 'b> Div<&'b Mod_e382_105> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn div(self, a: &'b Mod_e382_105) -> Mod_e382_105 {
        let mut out = self.clone();
        out /= a;
        out
    }
}

impl Div<Mod_e382_105> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn div(self, a: Mod_e382_105) -> Mod_e382_105 {
        &self / &a
    }
}

impl<'b> SubAssign<&'b Mod_e382_105> for Mod_e382_105 {
    fn sub_assign(&mut self, rhs: &'b Mod_e382_105) {
        let a0 = self.0[0];
        let a1 = self.0[1];
        let a2 = self.0[2];
        let a3 = self.0[3];
        let a4 = self.0[4];
        let a5 = self.0[5];
        let a6 = self.0[6] & 0x00003fffffffffff;

        let b0 = rhs.0[0];
        let b1 = rhs.0[1];
        let b2 = rhs.0[2];
        let b3 = rhs.0[3];
        let b4 = rhs.0[4];
        let b5 = rhs.0[5];
        let b6 = rhs.0[6] & 0x00003fffffffffff;

        let cin = self.carry_out() + rhs.carry_out();
        let s0 = a0 - b0 + (cin * C_VAL);
        let c0 = s0 >> 56;
        let s1 = a1 - b1 + c0;
        let c1 = s1 >> 56;
        let s2 = a2 - b2 + c1;
        let c2 = s2 >> 56;
        let s3 = a3 - b3 + c2;
        let c3 = s3 >> 56;
        let s4 = a4 - b4 + c3;
        let c4 = s4 >> 56;
        let s5 = a5 - b5 + c4;
        let c5 = s5 >> 56;
        let s6 = a6 - b6 + c5;

        self.0[0] = s0 & 0x00ffffffffffffff;
        self.0[1] = s1 & 0x00ffffffffffffff;
        self.0[2] = s2 & 0x00ffffffffffffff;
        self.0[3] = s3 & 0x00ffffffffffffff;
        self.0[4] = s4 & 0x00ffffffffffffff;
        self.0[5] = s5 & 0x00ffffffffffffff;
        self.0[6] = s6;
    }
}

impl SubAssign<Mod_e382_105> for Mod_e382_105 {
    fn sub_assign(&mut self, rhs: Mod_e382_105) {
        *self -= &rhs
    }
}

impl SubAssign<i32> for Mod_e382_105 {
    fn sub_assign(&mut self, rhs: i32) {
        self.small_sub_assign(rhs);
    }
}

impl SubAssign<i16> for Mod_e382_105 {
    fn sub_assign(&mut self, rhs: i16) {
        self.small_sub_assign(rhs as i32);
    }
}

impl SubAssign<i8> for Mod_e382_105 {
    fn sub_assign(&mut self, rhs: i8) {
        self.small_sub_assign(rhs as i32);
    }
}

impl Sub<i32> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: i32) -> Mod_e382_105 {
        &self - a
    }
}

impl Sub<i16> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: i16) -> Mod_e382_105 {
        &self - a
    }
}

impl Sub<i8> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: i8) -> Mod_e382_105 {
        &self - a
    }
}

impl<'a, 'b> Sub<&'b Mod_e382_105> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: &'b Mod_e382_105) -> Mod_e382_105 {
        let mut out = self.clone();
        out -= a;
        out
    }
}

impl Sub<Mod_e382_105> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: Mod_e382_105) -> Mod_e382_105 {
        &self - &a
    }
}

impl<'a> Sub<i32> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: i32) -> Mod_e382_105 {
        self.small_sub(a)
    }
}

impl<'a> Sub<i16> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: i16) -> Mod_e382_105 {
        self.small_sub(a as i32)
    }
}

impl<'a> Sub<i8> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn sub(self, a: i8) -> Mod_e382_105 {
        self.small_sub(a as i32)
    }
}

impl MulAssign<i32> for Mod_e382_105 {
    fn mul_assign(&mut self, rhs: i32) {
        self.small_mul_assign(rhs);
    }
}

impl MulAssign<i16> for Mod_e382_105 {
    fn mul_assign(&mut self, rhs: i16) {
        self.small_mul_assign(rhs as i32);
    }
}

impl MulAssign<i8> for Mod_e382_105 {
    fn mul_assign(&mut self, rhs: i8) {
        self.small_mul_assign(rhs as i32);
    }
}

impl<'b> MulAssign<&'b Mod_e382_105> for Mod_e382_105 {
    fn mul_assign(&mut self, rhs: &'b Mod_e382_105) {
        let a0 = self.0[0] & 0x0fffffff;
        let a1 = self.0[0] >> 28;
        let a2 = self.0[1] & 0x0fffffff;
        let a3 = self.0[1] >> 28;
        let a4 = self.0[2] & 0x0fffffff;
        let a5 = self.0[2] >> 28;
        let a6 = self.0[3] & 0x0fffffff;
        let a7 = self.0[3] >> 28;
        let a8 = self.0[4] & 0x0fffffff;
        let a9 = self.0[4] >> 28;
        let a10 = self.0[5] & 0x0fffffff;
        let a11 = self.0[5] >> 28;
        let a12 = self.0[6] & 0x0fffffff;
        let a13 = self.0[6] >> 28;

        let b0 = rhs.0[0] & 0x0fffffff;
        let b1 = rhs.0[0] >> 28;
        let b2 = rhs.0[1] & 0x0fffffff;
        let b3 = rhs.0[1] >> 28;
        let b4 = rhs.0[2] & 0x0fffffff;
        let b5 = rhs.0[2] >> 28;
        let b6 = rhs.0[3] & 0x0fffffff;
        let b7 = rhs.0[3] >> 28;
        let b8 = rhs.0[4] & 0x0fffffff;
        let b9 = rhs.0[4] >> 28;
        let b10 = rhs.0[5] & 0x0fffffff;
        let b11 = rhs.0[5] >> 28;
        let b12 = rhs.0[6] & 0x0fffffff;
        let b13 = rhs.0[6] >> 28;

        // Combined multiples
        let m_0_0 = a0 * b0;
        let m_0_1 = a0 * b1;
        let m_0_2 = a0 * b2;
        let m_0_3 = a0 * b3;
        let m_0_4 = a0 * b4;
        let m_0_5 = a0 * b5;
        let m_0_6 = a0 * b6;
        let m_0_7 = a0 * b7;
        let m_0_8 = a0 * b8;
        let m_0_9 = a0 * b9;
        let m_0_10 = a0 * b10;
        let m_0_11 = a0 * b11;
        let m_0_12 = a0 * b12;
        let m_0_13 = a0 * b13;
        let m_1_0 = a1 * b0;
        let m_1_1 = a1 * b1;
        let m_1_2 = a1 * b2;
        let m_1_3 = a1 * b3;
        let m_1_4 = a1 * b4;
        let m_1_5 = a1 * b5;
        let m_1_6 = a1 * b6;
        let m_1_7 = a1 * b7;
        let m_1_8 = a1 * b8;
        let m_1_9 = a1 * b9;
        let m_1_10 = a1 * b10;
        let m_1_11 = a1 * b11;
        let m_1_12 = a1 * b12;
        let m_1_13 = a1 * b13;
        let m_2_0 = a2 * b0;
        let m_2_1 = a2 * b1;
        let m_2_2 = a2 * b2;
        let m_2_3 = a2 * b3;
        let m_2_4 = a2 * b4;
        let m_2_5 = a2 * b5;
        let m_2_6 = a2 * b6;
        let m_2_7 = a2 * b7;
        let m_2_8 = a2 * b8;
        let m_2_9 = a2 * b9;
        let m_2_10 = a2 * b10;
        let m_2_11 = a2 * b11;
        let m_2_12 = a2 * b12;
        let m_2_13 = a2 * b13;
        let m_3_0 = a3 * b0;
        let m_3_1 = a3 * b1;
        let m_3_2 = a3 * b2;
        let m_3_3 = a3 * b3;
        let m_3_4 = a3 * b4;
        let m_3_5 = a3 * b5;
        let m_3_6 = a3 * b6;
        let m_3_7 = a3 * b7;
        let m_3_8 = a3 * b8;
        let m_3_9 = a3 * b9;
        let m_3_10 = a3 * b10;
        let m_3_11 = a3 * b11;
        let m_3_12 = a3 * b12;
        let m_3_13 = a3 * b13;
        let m_4_0 = a4 * b0;
        let m_4_1 = a4 * b1;
        let m_4_2 = a4 * b2;
        let m_4_3 = a4 * b3;
        let m_4_4 = a4 * b4;
        let m_4_5 = a4 * b5;
        let m_4_6 = a4 * b6;
        let m_4_7 = a4 * b7;
        let m_4_8 = a4 * b8;
        let m_4_9 = a4 * b9;
        let m_4_10 = a4 * b10;
        let m_4_11 = a4 * b11;
        let m_4_12 = a4 * b12;
        let m_4_13 = a4 * b13;
        let m_5_0 = a5 * b0;
        let m_5_1 = a5 * b1;
        let m_5_2 = a5 * b2;
        let m_5_3 = a5 * b3;
        let m_5_4 = a5 * b4;
        let m_5_5 = a5 * b5;
        let m_5_6 = a5 * b6;
        let m_5_7 = a5 * b7;
        let m_5_8 = a5 * b8;
        let m_5_9 = a5 * b9;
        let m_5_10 = a5 * b10;
        let m_5_11 = a5 * b11;
        let m_5_12 = a5 * b12;
        let m_5_13 = a5 * b13;
        let m_6_0 = a6 * b0;
        let m_6_1 = a6 * b1;
        let m_6_2 = a6 * b2;
        let m_6_3 = a6 * b3;
        let m_6_4 = a6 * b4;
        let m_6_5 = a6 * b5;
        let m_6_6 = a6 * b6;
        let m_6_7 = a6 * b7;
        let m_6_8 = a6 * b8;
        let m_6_9 = a6 * b9;
        let m_6_10 = a6 * b10;
        let m_6_11 = a6 * b11;
        let m_6_12 = a6 * b12;
        let m_6_13 = a6 * b13;
        let m_7_0 = a7 * b0;
        let m_7_1 = a7 * b1;
        let m_7_2 = a7 * b2;
        let m_7_3 = a7 * b3;
        let m_7_4 = a7 * b4;
        let m_7_5 = a7 * b5;
        let m_7_6 = a7 * b6;
        let m_7_7 = a7 * b7;
        let m_7_8 = a7 * b8;
        let m_7_9 = a7 * b9;
        let m_7_10 = a7 * b10;
        let m_7_11 = a7 * b11;
        let m_7_12 = a7 * b12;
        let m_7_13 = a7 * b13;
        let m_8_0 = a8 * b0;
        let m_8_1 = a8 * b1;
        let m_8_2 = a8 * b2;
        let m_8_3 = a8 * b3;
        let m_8_4 = a8 * b4;
        let m_8_5 = a8 * b5;
        let m_8_6 = a8 * b6;
        let m_8_7 = a8 * b7;
        let m_8_8 = a8 * b8;
        let m_8_9 = a8 * b9;
        let m_8_10 = a8 * b10;
        let m_8_11 = a8 * b11;
        let m_8_12 = a8 * b12;
        let m_8_13 = a8 * b13;
        let m_9_0 = a9 * b0;
        let m_9_1 = a9 * b1;
        let m_9_2 = a9 * b2;
        let m_9_3 = a9 * b3;
        let m_9_4 = a9 * b4;
        let m_9_5 = a9 * b5;
        let m_9_6 = a9 * b6;
        let m_9_7 = a9 * b7;
        let m_9_8 = a9 * b8;
        let m_9_9 = a9 * b9;
        let m_9_10 = a9 * b10;
        let m_9_11 = a9 * b11;
        let m_9_12 = a9 * b12;
        let m_9_13 = a9 * b13;
        let m_10_0 = a10 * b0;
        let m_10_1 = a10 * b1;
        let m_10_2 = a10 * b2;
        let m_10_3 = a10 * b3;
        let m_10_4 = a10 * b4;
        let m_10_5 = a10 * b5;
        let m_10_6 = a10 * b6;
        let m_10_7 = a10 * b7;
        let m_10_8 = a10 * b8;
        let m_10_9 = a10 * b9;
        let m_10_10 = a10 * b10;
        let m_10_11 = a10 * b11;
        let m_10_12 = a10 * b12;
        let m_10_13 = a10 * b13;
        let m_11_0 = a11 * b0;
        let m_11_1 = a11 * b1;
        let m_11_2 = a11 * b2;
        let m_11_3 = a11 * b3;
        let m_11_4 = a11 * b4;
        let m_11_5 = a11 * b5;
        let m_11_6 = a11 * b6;
        let m_11_7 = a11 * b7;
        let m_11_8 = a11 * b8;
        let m_11_9 = a11 * b9;
        let m_11_10 = a11 * b10;
        let m_11_11 = a11 * b11;
        let m_11_12 = a11 * b12;
        let m_11_13 = a11 * b13;
        let m_12_0 = a12 * b0;
        let m_12_1 = a12 * b1;
        let m_12_2 = a12 * b2;
        let m_12_3 = a12 * b3;
        let m_12_4 = a12 * b4;
        let m_12_5 = a12 * b5;
        let m_12_6 = a12 * b6;
        let m_12_7 = a12 * b7;
        let m_12_8 = a12 * b8;
        let m_12_9 = a12 * b9;
        let m_12_10 = a12 * b10;
        let m_12_11 = a12 * b11;
        let m_12_12 = a12 * b12;
        let m_12_13 = a12 * b13;
        let m_13_0 = a13 * b0;
        let m_13_1 = a13 * b1;
        let m_13_2 = a13 * b2;
        let m_13_3 = a13 * b3;
        let m_13_4 = a13 * b4;
        let m_13_5 = a13 * b5;
        let m_13_6 = a13 * b6;
        let m_13_7 = a13 * b7;
        let m_13_8 = a13 * b8;
        let m_13_9 = a13 * b9;
        let m_13_10 = a13 * b10;
        let m_13_11 = a13 * b11;
        let m_13_12 = a13 * b12;
        let m_13_13 = a13 * b13;

        // Compute the 40-digit combined product using 64-bit operations.
        let d0 = m_0_0 + ((m_0_1 & 0x0fffffff) << 28) +
                 ((m_1_0 & 0x0fffffff) << 28);
        let c0 = d0 >> 56;
        let d1 = (m_0_1 >> 28) + m_0_2 + ((m_0_3 & 0x0fffffff) << 28) +
                 (m_1_0 >> 28) + m_1_1 + ((m_1_2 & 0x0fffffff) << 28) +
                 m_2_0 + ((m_2_1 & 0x0fffffff) << 28) +
                 ((m_3_0 & 0x0fffffff) << 28) + c0;
        let c1 = d1 >> 56;
        let d2 = (m_0_3 >> 28) + m_0_4 + ((m_0_5 & 0x0fffffff) << 28) +
                 (m_1_2 >> 28) + m_1_3 + ((m_1_4 & 0x0fffffff) << 28) +
                 (m_2_1 >> 28) + m_2_2 + ((m_2_3 & 0x0fffffff) << 28) +
                 (m_3_0 >> 28) + m_3_1 + ((m_3_2 & 0x0fffffff) << 28) +
                 m_4_0 + ((m_4_1 & 0x0fffffff) << 28) +
                 ((m_5_0 & 0x0fffffff) << 28) + c1;
        let c2 = d2 >> 56;
        let d3 = (m_0_5 >> 28) + m_0_6 + ((m_0_7 & 0x0fffffff) << 28) +
                 (m_1_4 >> 28) + m_1_5 + ((m_1_6 & 0x0fffffff) << 28) +
                 (m_2_3 >> 28) + m_2_4 + ((m_2_5 & 0x0fffffff) << 28) +
                 (m_3_2 >> 28) + m_3_3 + ((m_3_4 & 0x0fffffff) << 28) +
                 (m_4_1 >> 28) + m_4_2 + ((m_4_3 & 0x0fffffff) << 28) +
                 (m_5_0 >> 28) + m_5_1 + ((m_5_2 & 0x0fffffff) << 28) +
                 m_6_0 + ((m_6_1 & 0x0fffffff) << 28) +
                 ((m_7_0 & 0x0fffffff) << 28) + c2;
        let c3 = d3 >> 56;
        let d4 = (m_0_7 >> 28) + m_0_8 + ((m_0_9 & 0x0fffffff) << 28) +
                 (m_1_6 >> 28) + m_1_7 + ((m_1_8 & 0x0fffffff) << 28) +
                 (m_2_5 >> 28) + m_2_6 + ((m_2_7 & 0x0fffffff) << 28) +
                 (m_3_4 >> 28) + m_3_5 + ((m_3_6 & 0x0fffffff) << 28) +
                 (m_4_3 >> 28) + m_4_4 + ((m_4_5 & 0x0fffffff) << 28) +
                 (m_5_2 >> 28) + m_5_3 + ((m_5_4 & 0x0fffffff) << 28) +
                 (m_6_1 >> 28) + m_6_2 + ((m_6_3 & 0x0fffffff) << 28) +
                 (m_7_0 >> 28) + m_7_1 + ((m_7_2 & 0x0fffffff) << 28) +
                 m_8_0 + ((m_8_1 & 0x0fffffff) << 28) +
                 ((m_9_0 & 0x0fffffff) << 28) + c3;
        let c4 = d4 >> 56;
        let d5 = (m_0_9 >> 28) + m_0_10 + ((m_0_11 & 0x0fffffff) << 28) +
                 (m_1_8 >> 28) + m_1_9 + ((m_1_10 & 0x0fffffff) << 28) +
                 (m_2_7 >> 28) + m_2_8 + ((m_2_9 & 0x0fffffff) << 28) +
                 (m_3_6 >> 28) + m_3_7 + ((m_3_8 & 0x0fffffff) << 28) +
                 (m_4_5 >> 28) + m_4_6 + ((m_4_7 & 0x0fffffff) << 28) +
                 (m_5_4 >> 28) + m_5_5 + ((m_5_6 & 0x0fffffff) << 28) +
                 (m_6_3 >> 28) + m_6_4 + ((m_6_5 & 0x0fffffff) << 28) +
                 (m_7_2 >> 28) + m_7_3 + ((m_7_4 & 0x0fffffff) << 28) +
                 (m_8_1 >> 28) + m_8_2 + ((m_8_3 & 0x0fffffff) << 28) +
                 (m_9_0 >> 28) + m_9_1 + ((m_9_2 & 0x0fffffff) << 28) +
                 m_10_0 + ((m_10_1 & 0x0fffffff) << 28) +
                 ((m_11_0 & 0x0fffffff) << 28) + c4;
        let c5 = d5 >> 56;
        let d6 = (m_0_11 >> 28) + m_0_12 + ((m_0_13 & 0x0fffffff) << 28) +
                 (m_1_10 >> 28) + m_1_11 + ((m_1_12 & 0x0fffffff) << 28) +
                 (m_2_9 >> 28) + m_2_10 + ((m_2_11 & 0x0fffffff) << 28) +
                 (m_3_8 >> 28) + m_3_9 + ((m_3_10 & 0x0fffffff) << 28) +
                 (m_4_7 >> 28) + m_4_8 + ((m_4_9 & 0x0fffffff) << 28) +
                 (m_5_6 >> 28) + m_5_7 + ((m_5_8 & 0x0fffffff) << 28) +
                 (m_6_5 >> 28) + m_6_6 + ((m_6_7 & 0x0fffffff) << 28) +
                 (m_7_4 >> 28) + m_7_5 + ((m_7_6 & 0x0fffffff) << 28) +
                 (m_8_3 >> 28) + m_8_4 + ((m_8_5 & 0x0fffffff) << 28) +
                 (m_9_2 >> 28) + m_9_3 + ((m_9_4 & 0x0fffffff) << 28) +
                 (m_10_1 >> 28) + m_10_2 + ((m_10_3 & 0x0fffffff) << 28) +
                 (m_11_0 >> 28) + m_11_1 + ((m_11_2 & 0x0fffffff) << 28) +
                 m_12_0 + ((m_12_1 & 0x0fffffff) << 28) +
                 ((m_13_0 & 0x0fffffff) << 28) + c5;
        let c6 = d6 >> 56;
        let d7 = (m_0_13 >> 28) +
                 (m_1_12 >> 28) + m_1_13 +
                 (m_2_11 >> 28) + m_2_12 + ((m_2_13 & 0x0fffffff) << 28) +
                 (m_3_10 >> 28) + m_3_11 + ((m_3_12 & 0x0fffffff) << 28) +
                 (m_4_9 >> 28) + m_4_10 + ((m_4_11 & 0x0fffffff) << 28) +
                 (m_5_8 >> 28) + m_5_9 + ((m_5_10 & 0x0fffffff) << 28) +
                 (m_6_7 >> 28) + m_6_8 + ((m_6_9 & 0x0fffffff) << 28) +
                 (m_7_6 >> 28) + m_7_7 + ((m_7_8 & 0x0fffffff) << 28) +
                 (m_8_5 >> 28) + m_8_6 + ((m_8_7 & 0x0fffffff) << 28) +
                 (m_9_4 >> 28) + m_9_5 + ((m_9_6 & 0x0fffffff) << 28) +
                 (m_10_3 >> 28) + m_10_4 + ((m_10_5 & 0x0fffffff) << 28) +
                 (m_11_2 >> 28) + m_11_3 + ((m_11_4 & 0x0fffffff) << 28) +
                 (m_12_1 >> 28) + m_12_2 + ((m_12_3 & 0x0fffffff) << 28) +
                 (m_13_0 >> 28) + m_13_1 + ((m_13_2 & 0x0fffffff) << 28) +
                 c6;
        let c7 = d7 >> 56;
        let d8 = (m_2_13 >> 28) +
                 (m_3_12 >> 28) + m_3_13 +
                 (m_4_11 >> 28) + m_4_12 + ((m_4_13 & 0x0fffffff) << 28) +
                 (m_5_10 >> 28) + m_5_11 + ((m_5_12 & 0x0fffffff) << 28) +
                 (m_6_9 >> 28) + m_6_10 + ((m_6_11 & 0x0fffffff) << 28) +
                 (m_7_8 >> 28) + m_7_9 + ((m_7_10 & 0x0fffffff) << 28) +
                 (m_8_7 >> 28) + m_8_8 + ((m_8_9 & 0x0fffffff) << 28) +
                 (m_9_6 >> 28) + m_9_7 + ((m_9_8 & 0x0fffffff) << 28) +
                 (m_10_5 >> 28) + m_10_6 + ((m_10_7 & 0x0fffffff) << 28) +
                 (m_11_4 >> 28) + m_11_5 + ((m_11_6 & 0x0fffffff) << 28) +
                 (m_12_3 >> 28) + m_12_4 + ((m_12_5 & 0x0fffffff) << 28) +
                 (m_13_2 >> 28) + m_13_3 + ((m_13_4 & 0x0fffffff) << 28) +
                 c7;
        let c8 = d8 >> 56;
        let d9 = (m_4_13 >> 28) +
                 (m_5_12 >> 28) + m_5_13 +
                 (m_6_11 >> 28) + m_6_12 + ((m_6_13 & 0x0fffffff) << 28) +
                 (m_7_10 >> 28) + m_7_11 + ((m_7_12 & 0x0fffffff) << 28) +
                 (m_8_9 >> 28) + m_8_10 + ((m_8_11 & 0x0fffffff) << 28) +
                 (m_9_8 >> 28) + m_9_9 + ((m_9_10 & 0x0fffffff) << 28) +
                 (m_10_7 >> 28) + m_10_8 + ((m_10_9 & 0x0fffffff) << 28) +
                 (m_11_6 >> 28) + m_11_7 + ((m_11_8 & 0x0fffffff) << 28) +
                 (m_12_5 >> 28) + m_12_6 + ((m_12_7 & 0x0fffffff) << 28) +
                 (m_13_4 >> 28) + m_13_5 + ((m_13_6 & 0x0fffffff) << 28) +
                 c8;
        let c9 = d9 >> 56;
        let d10 = (m_6_13 >> 28) +
                  (m_7_12 >> 28) + m_7_13 +
                  (m_8_11 >> 28) + m_8_12 + ((m_8_13 & 0x0fffffff) << 28) +
                  (m_9_10 >> 28) + m_9_11 + ((m_9_12 & 0x0fffffff) << 28) +
                  (m_10_9 >> 28) + m_10_10 + ((m_10_11 & 0x0fffffff) << 28) +
                  (m_11_8 >> 28) + m_11_9 + ((m_11_10 & 0x0fffffff) << 28) +
                  (m_12_7 >> 28) + m_12_8 + ((m_12_9 & 0x0fffffff) << 28) +
                  (m_13_6 >> 28) + m_13_7 + ((m_13_8 & 0x0fffffff) << 28) +
                  c9;
        let c10 = d10 >> 56;
        let d11 = (m_8_13 >> 28) +
                  (m_9_12 >> 28) + m_9_13 +
                  (m_10_11 >> 28) + m_10_12 + ((m_10_13 & 0x0fffffff) << 28) +
                  (m_11_10 >> 28) + m_11_11 + ((m_11_12 & 0x0fffffff) << 28) +
                  (m_12_9 >> 28) + m_12_10 + ((m_12_11 & 0x0fffffff) << 28) +
                  (m_13_8 >> 28) + m_13_9 + ((m_13_10 & 0x0fffffff) << 28) +
                  c10;
        let c11 = d11 >> 56;
        let d12 = (m_10_13 >> 28) +
                  (m_11_12 >> 28) + m_11_13 +
                  (m_12_11 >> 28) + m_12_12 + ((m_12_13 & 0x0fffffff) << 28) +
                  (m_13_10 >> 28) + m_13_11 + ((m_13_12 & 0x0fffffff) << 28) +
                  c11;
        let c12 = d12 >> 56;
        let d13 = (m_12_13 >> 28) +
                  (m_13_12 >> 28) + m_13_13 +
                  c12;

        // Modular reduction by a pseudo-mersenne prime of the form 2^n - c.

        // These are the n low-order
        let l0_0 = d0 & 0x00ffffffffffffff;
        let l1_0 = d1 & 0x00ffffffffffffff;
        let l2_0 = d2 & 0x00ffffffffffffff;
        let l3_0 = d3 & 0x00ffffffffffffff;
        let l4_0 = d4 & 0x00ffffffffffffff;
        let l5_0 = d5 & 0x00ffffffffffffff;
        let l6_0 = d6 & 0x00003fffffffffff;

        // Shift the high bits down into another n-bit number.
        let h0_0 = ((d6 & 0x00ffffffffffffff) >> 46) |
                   ((d7 & 0x00003fffffffffff) << 10);
        let h1_0 = ((d7 & 0x00ffffffffffffff) >> 46) |
                   ((d8 & 0x00003fffffffffff) << 10);
        let h2_0 = ((d8 & 0x00ffffffffffffff) >> 46) |
                   ((d9 & 0x00003fffffffffff) << 10);
        let h3_0 = ((d9 & 0x00ffffffffffffff) >> 46) |
                   ((d10 & 0x00003fffffffffff) << 10);
        let h4_0 = ((d10 & 0x00ffffffffffffff) >> 46) |
                   ((d11 & 0x00003fffffffffff) << 10);
        let h5_0 = ((d11 & 0x00ffffffffffffff) >> 46) |
                   ((d12 & 0x00003fffffffffff) << 10);
        let h6_0 = ((d12 & 0x00ffffffffffffff) >> 46) |
                   (d13 << 10);

        // Multiply by C
        let hc0_0 = h0_0 * C_VAL;
        let hc1_0 = h1_0 * C_VAL;
        let hc2_0 = h2_0 * C_VAL;
        let hc3_0 = h3_0 * C_VAL;
        let hc4_0 = h4_0 * C_VAL;
        let hc5_0 = h5_0 * C_VAL;
        let hc6_0 = h6_0 * C_VAL;

        // Add h and l.
        let kin_0 = hc6_0 >> 46;
        let s0_0 = l0_0 + hc0_0 + (kin_0 * C_VAL);
        let k0_0 = s0_0 >> 56;
        let s1_0 = l1_0 + hc1_0 + k0_0;
        let k1_0 = s1_0 >> 56;
        let s2_0 = l2_0 + hc2_0 + k1_0;
        let k2_0 = s2_0 >> 56;
        let s3_0 = l3_0 + hc3_0 + k2_0;
        let k3_0 = s3_0 >> 56;
        let s4_0 = l4_0 + hc4_0 + k3_0;
        let k4_0 = s4_0 >> 56;
        let s5_0 = l5_0 + hc5_0 + k4_0;
        let k5_0 = s5_0 >> 56;
        let s6_0 = l6_0 + (hc6_0 & 0x00003fffffffffff) + k5_0;

        self.0[0] = s0_0 & 0x00ffffffffffffff;
        self.0[1] = s1_0 & 0x00ffffffffffffff;
        self.0[2] = s2_0 & 0x00ffffffffffffff;
        self.0[3] = s3_0 & 0x00ffffffffffffff;
        self.0[4] = s4_0 & 0x00ffffffffffffff;
        self.0[5] = s5_0 & 0x00ffffffffffffff;
        self.0[6] = s6_0;
     }
}

impl MulAssign<Mod_e382_105> for Mod_e382_105 {
    fn mul_assign(&mut self, rhs: Mod_e382_105) {
        *self *= &rhs;
    }
}

impl<'a> Mul<&'a Mod_e382_105> for i32 {
    type Output = Mod_e382_105;

    fn mul(self, a: &'a Mod_e382_105) -> Mod_e382_105 {
        a.small_mul(self)
    }
}

impl<'a> Mul<&'a Mod_e382_105> for i16 {
    type Output = Mod_e382_105;

    fn mul(self, a: &'a Mod_e382_105) -> Mod_e382_105 {
        a.small_mul(self as i32)
    }
}

impl<'a> Mul<&'a Mod_e382_105> for i8 {
    type Output = Mod_e382_105;

    fn mul(self, a: &'a Mod_e382_105) -> Mod_e382_105 {
        a.small_mul(self as i32)
    }
}

impl<'a> Mul<i32> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: i32) -> Mod_e382_105 {
        self.small_mul(a)
    }
}

impl<'a> Mul<i16> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: i16) -> Mod_e382_105 {
        self.small_mul(a as i32)
    }
}

impl<'a> Mul<i8> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: i8) -> Mod_e382_105 {
        self.small_mul(a as i32)
    }
}

impl Mul<i32> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: i32) -> Mod_e382_105 {
        &self * a
    }
}

impl Mul<i16> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: i16) -> Mod_e382_105 {
        &self * a
    }
}

impl Mul<i8> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: i8) -> Mod_e382_105 {
        &self * a
    }
}

impl<'a, 'b> Mul<&'b Mod_e382_105> for &'a Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: &'b Mod_e382_105) -> Mod_e382_105 {
        let mut out = self.clone();
        out *= a;
        out
    }
}

impl Mul<Mod_e382_105> for Mod_e382_105 {
    type Output = Mod_e382_105;

    fn mul(self, a: Mod_e382_105) -> Mod_e382_105 {
        &self * &a
    }
}

impl Rand for Mod_e382_105 {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut out = Mod_e382_105([0i64; 7]);

        for i in 0..7 {
            out.0[i] = rng.gen_range(0, MODULUS.0[i]);
        }

        out
    }
}

impl Pack for Mod_e382_105 {
    fn unpack(&mut self, bytes: &[u8]) {
        self.0[0] = ((bytes[0] as i64) & 0x00000000000000ff) |
                  (((bytes[1] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[2] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[3] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[4] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[5] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[6] as i64) << 48) & 0x00ff000000000000);
        self.0[1] = ((bytes[7] as i64) & 0x00000000000000ff) |
                  (((bytes[8] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[9] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[10] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[11] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[12] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[13] as i64) << 48) & 0x00ff000000000000);
        self.0[2] = ((bytes[14] as i64) & 0x00000000000000ff) |
                  (((bytes[15] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[16] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[17] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[18] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[19] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[20] as i64) << 48) & 0x00ff000000000000);
        self.0[3] = ((bytes[21] as i64) & 0x00000000000000ff) |
                  (((bytes[22] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[23] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[24] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[25] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[26] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[27] as i64) << 48) & 0x00ff000000000000);
        self.0[4] = ((bytes[28] as i64) & 0x00000000000000ff) |
                  (((bytes[29] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[30] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[31] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[32] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[33] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[34] as i64) << 48) & 0x00ff000000000000);
        self.0[5] = ((bytes[35] as i64) & 0x00000000000000ff) |
                  (((bytes[36] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[37] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[38] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[39] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[40] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[41] as i64) << 48) & 0x00ff000000000000);
        self.0[6] = ((bytes[42] as i64) & 0x00000000000000ff) |
                  (((bytes[43] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[44] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[45] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[46] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[47] as i64) << 40) & 0x00003f0000000000);
    }

    fn unpacked(bytes: &[u8]) -> Self {
        let mut out = ZERO;
        out.unpack(bytes);
        out
    }

    fn pack(&mut self, bytes: &mut [u8]) {
        self.normalize();
        self.pack_normalized(bytes)
    }

    fn pack_normalized(&self, bytes: &mut [u8]) {
        bytes[0] = (self.0[0] & 0b11111111) as u8;
        bytes[1] = ((self.0[0] >> 8) & 0b11111111) as u8;
        bytes[2] = ((self.0[0] >> 16) & 0b11111111) as u8;
        bytes[3] = ((self.0[0] >> 24) & 0b11111111) as u8;
        bytes[4] = ((self.0[0] >> 32) & 0b11111111) as u8;
        bytes[5] = ((self.0[0] >> 40) & 0b11111111) as u8;
        bytes[6] = ((self.0[0] >> 48) & 0b11111111) as u8;
        bytes[7] = (self.0[1] & 0b11111111) as u8;
        bytes[8] = ((self.0[1] >> 8) & 0b11111111) as u8;
        bytes[9] = ((self.0[1] >> 16) & 0b11111111) as u8;
        bytes[10] = ((self.0[1] >> 24) & 0b11111111) as u8;
        bytes[11] = ((self.0[1] >> 32) & 0b11111111) as u8;
        bytes[12] = ((self.0[1] >> 40) & 0b11111111) as u8;
        bytes[13] = ((self.0[1] >> 48) & 0b11111111) as u8;
        bytes[14] = (self.0[2] & 0b11111111) as u8;
        bytes[15] = ((self.0[2] >> 8) & 0b11111111) as u8;
        bytes[16] = ((self.0[2] >> 16) & 0b11111111) as u8;
        bytes[17] = ((self.0[2] >> 24) & 0b11111111) as u8;
        bytes[18] = ((self.0[2] >> 32) & 0b11111111) as u8;
        bytes[19] = ((self.0[2] >> 40) & 0b11111111) as u8;
        bytes[20] = ((self.0[2] >> 48) & 0b11111111) as u8;
        bytes[21] = (self.0[3] & 0b11111111) as u8;
        bytes[22] = ((self.0[3] >> 8) & 0b11111111) as u8;
        bytes[23] = ((self.0[3] >> 16) & 0b11111111) as u8;
        bytes[24] = ((self.0[3] >> 24) & 0b11111111) as u8;
        bytes[25] = ((self.0[3] >> 32) & 0b11111111) as u8;
        bytes[26] = ((self.0[3] >> 40) & 0b11111111) as u8;
        bytes[27] = ((self.0[3] >> 48) & 0b11111111) as u8;
        bytes[28] = (self.0[4] & 0b11111111) as u8;
        bytes[29] = ((self.0[4] >> 8) & 0b11111111) as u8;
        bytes[30] = ((self.0[4] >> 16) & 0b11111111) as u8;
        bytes[31] = ((self.0[4] >> 24) & 0b11111111) as u8;
        bytes[32] = ((self.0[4] >> 32) & 0b11111111) as u8;
        bytes[33] = ((self.0[4] >> 40) & 0b11111111) as u8;
        bytes[34] = ((self.0[4] >> 48) & 0b11111111) as u8;
        bytes[35] = (self.0[5] & 0b11111111) as u8;
        bytes[36] = ((self.0[5] >> 8) & 0b11111111) as u8;
        bytes[37] = ((self.0[5] >> 16) & 0b11111111) as u8;
        bytes[38] = ((self.0[5] >> 24) & 0b11111111) as u8;
        bytes[39] = ((self.0[5] >> 32) & 0b11111111) as u8;
        bytes[40] = ((self.0[5] >> 40) & 0b11111111) as u8;
        bytes[41] = ((self.0[5] >> 48) & 0b11111111) as u8;
        bytes[42] = (self.0[6] & 0b11111111) as u8;
        bytes[43] = ((self.0[6] >> 8) & 0b11111111) as u8;
        bytes[44] = ((self.0[6] >> 16) & 0b11111111) as u8;
        bytes[45] = ((self.0[6] >> 24) & 0b11111111) as u8;
        bytes[46] = ((self.0[6] >> 32) & 0b11111111) as u8;
        bytes[47] = ((self.0[6] >> 40) & 0b00111111) as u8;
    }

    fn nbytes() -> i32 {
        48
    }
}

impl Normalize for Mod_e382_105 {
    fn normalize(&mut self) {
        let plusc = &*self + (C_VAL as i32);
        let offset = &MODULUS * (plusc.carry_out() as i32);
        *self -= &offset;
    }
}

impl NormalizeEq for Mod_e382_105 {
    fn normalize_self_eq(&mut self, other: &Self) -> bool {
        let mut are_equal: bool = true;

        self.normalize();

        are_equal &= self.0[0] == other.0[0];
        are_equal &= self.0[1] == other.0[1];
        are_equal &= self.0[2] == other.0[2];
        are_equal &= self.0[3] == other.0[3];
        are_equal &= self.0[4] == other.0[4];
        are_equal &= self.0[5] == other.0[5];
        are_equal &= self.0[6] == other.0[6];

        are_equal
    }

    fn normalize_eq(&mut self, other: &mut Self) -> bool {
        other.normalize();
        self.normalize_self_eq(other)
    }
}

impl PrimeFieldMask for Mod_e382_105_Mask {
    fn fill(&mut self, bit: bool) {
        let mut mask = bit as i64;

        mask |= mask << 1;
        mask |= mask << 2;
        mask |= mask << 4;
        mask |= mask << 8;
        mask |= mask << 16;
        mask |= mask << 32;

        self.0[0] = mask;
        self.0[1] = mask;
        self.0[2] = mask;
        self.0[3] = mask;
        self.0[4] = mask;
        self.0[5] = mask;
        self.0[6] = mask;
    }

    fn filled(bit: bool) -> Self {
        let mut mask = bit as i64;

        mask |= mask << 1;
        mask |= mask << 2;
        mask |= mask << 4;
        mask |= mask << 8;
        mask |= mask << 16;
        mask |= mask << 32;

        Mod_e382_105_Mask([mask; 7])
    }
}

impl PrimeField for Mod_e382_105 {
    fn fill(&mut self, bit: bool) {
        let mut mask = bit as i64;

        mask |= mask << 1;
        mask |= mask << 2;
        mask |= mask << 4;
        mask |= mask << 8;
        mask |= mask << 16;
        mask |= mask << 32;

        self.0[0] = mask;
        self.0[1] = mask;
        self.0[2] = mask;
        self.0[3] = mask;
        self.0[4] = mask;
        self.0[5] = mask;
        self.0[6] = mask;
    }

    fn filled(bit: bool) -> Self {
        let mut mask = bit as i64;

        mask |= mask << 1;
        mask |= mask << 2;
        mask |= mask << 4;
        mask |= mask << 8;
        mask |= mask << 16;
        mask |= mask << 32;

        Mod_e382_105([mask; 7])
    }

    fn nbits() -> usize { 382 }

    fn bit_normalized(&self, idx: usize) -> bool {
        let byte = idx / 56;
        let bit = idx % 56;

        (self.0[byte] >> bit) & 0x1 == 0x1
    }

    fn bit(&mut self, idx: usize) -> bool {
        self.normalize();
        self.bit_normalized(idx)
    }

    fn normalize_bitand(&mut self, rhs: &mut Self) {
        rhs.normalize();
        self.normalize_self_bitand(rhs);
    }

    fn normalize_self_bitand(&mut self, rhs: &Self) {
        self.normalize();
        self.normalized_bitand(rhs);
    }


    fn normalized_bitand(&mut self, rhs: &Self) {
        self.0[0] &= rhs.0[0];
        self.0[1] &= rhs.0[1];
        self.0[2] &= rhs.0[2];
        self.0[3] &= rhs.0[3];
        self.0[4] &= rhs.0[4];
        self.0[5] &= rhs.0[5];
        self.0[6] &= rhs.0[6];
    }

    fn normalize_bitor(&mut self, rhs: &mut Self) {
        rhs.normalize();
        self.normalize_self_bitor(rhs);
    }

    fn normalize_self_bitor(&mut self, rhs: &Self) {
        self.normalize();
        self.normalized_bitor(rhs);
    }

    fn normalized_bitor(&mut self, rhs: &Self) {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
        self.0[2] |= rhs.0[2];
        self.0[3] |= rhs.0[3];
        self.0[4] |= rhs.0[4];
        self.0[5] |= rhs.0[5];
        self.0[6] |= rhs.0[6];
    }

    fn zero() -> Mod_e382_105 {
        return ZERO;
    }

    fn one() -> Mod_e382_105 {
        return ONE;
    }

    fn m_one() -> Mod_e382_105 {
        return M_ONE;
    }

    fn modulus() -> Mod_e382_105 {
        return MODULUS;
    }

    fn square(&mut self) {
        let a0 = self.0[0] & 0x0fffffff;
        let a1 = self.0[0] >> 28;
        let a2 = self.0[1] & 0x0fffffff;
        let a3 = self.0[1] >> 28;
        let a4 = self.0[2] & 0x0fffffff;
        let a5 = self.0[2] >> 28;
        let a6 = self.0[3] & 0x0fffffff;
        let a7 = self.0[3] >> 28;
        let a8 = self.0[4] & 0x0fffffff;
        let a9 = self.0[4] >> 28;
        let a10 = self.0[5] & 0x0fffffff;
        let a11 = self.0[5] >> 28;
        let a12 = self.0[6] & 0x0fffffff;
        let a13 = self.0[6] >> 28;

        // Combined multiples
        let m_0_0 = a0 * a0;
        let m_0_1 = a0 * a1;
        let m_0_2 = a0 * a2;
        let m_0_3 = a0 * a3;
        let m_0_4 = a0 * a4;
        let m_0_5 = a0 * a5;
        let m_0_6 = a0 * a6;
        let m_0_7 = a0 * a7;
        let m_0_8 = a0 * a8;
        let m_0_9 = a0 * a9;
        let m_0_10 = a0 * a10;
        let m_0_11 = a0 * a11;
        let m_0_12 = a0 * a12;
        let m_0_13 = a0 * a13;
        let m_1_0 = m_0_1;
        let m_1_1 = a1 * a1;
        let m_1_2 = a1 * a2;
        let m_1_3 = a1 * a3;
        let m_1_4 = a1 * a4;
        let m_1_5 = a1 * a5;
        let m_1_6 = a1 * a6;
        let m_1_7 = a1 * a7;
        let m_1_8 = a1 * a8;
        let m_1_9 = a1 * a9;
        let m_1_10 = a1 * a10;
        let m_1_11 = a1 * a11;
        let m_1_12 = a1 * a12;
        let m_1_13 = a1 * a13;
        let m_2_0 = m_0_2;
        let m_2_1 = m_1_2;
        let m_2_2 = a2 * a2;
        let m_2_3 = a2 * a3;
        let m_2_4 = a2 * a4;
        let m_2_5 = a2 * a5;
        let m_2_6 = a2 * a6;
        let m_2_7 = a2 * a7;
        let m_2_8 = a2 * a8;
        let m_2_9 = a2 * a9;
        let m_2_10 = a2 * a10;
        let m_2_11 = a2 * a11;
        let m_2_12 = a2 * a12;
        let m_2_13 = a2 * a13;
        let m_3_0 = m_0_3;
        let m_3_1 = m_1_3;
        let m_3_2 = m_2_3;
        let m_3_3 = a3 * a3;
        let m_3_4 = a3 * a4;
        let m_3_5 = a3 * a5;
        let m_3_6 = a3 * a6;
        let m_3_7 = a3 * a7;
        let m_3_8 = a3 * a8;
        let m_3_9 = a3 * a9;
        let m_3_10 = a3 * a10;
        let m_3_11 = a3 * a11;
        let m_3_12 = a3 * a12;
        let m_3_13 = a3 * a13;
        let m_4_0 = m_0_4;
        let m_4_1 = m_1_4;
        let m_4_2 = m_2_4;
        let m_4_3 = m_3_4;
        let m_4_4 = a4 * a4;
        let m_4_5 = a4 * a5;
        let m_4_6 = a4 * a6;
        let m_4_7 = a4 * a7;
        let m_4_8 = a4 * a8;
        let m_4_9 = a4 * a9;
        let m_4_10 = a4 * a10;
        let m_4_11 = a4 * a11;
        let m_4_12 = a4 * a12;
        let m_4_13 = a4 * a13;
        let m_5_0 = m_0_5;
        let m_5_1 = m_1_5;
        let m_5_2 = m_2_5;
        let m_5_3 = m_3_5;
        let m_5_4 = m_4_5;
        let m_5_5 = a5 * a5;
        let m_5_6 = a5 * a6;
        let m_5_7 = a5 * a7;
        let m_5_8 = a5 * a8;
        let m_5_9 = a5 * a9;
        let m_5_10 = a5 * a10;
        let m_5_11 = a5 * a11;
        let m_5_12 = a5 * a12;
        let m_5_13 = a5 * a13;
        let m_6_0 = m_0_6;
        let m_6_1 = m_1_6;
        let m_6_2 = m_2_6;
        let m_6_3 = m_3_6;
        let m_6_4 = m_4_6;
        let m_6_5 = m_5_6;
        let m_6_6 = a6 * a6;
        let m_6_7 = a6 * a7;
        let m_6_8 = a6 * a8;
        let m_6_9 = a6 * a9;
        let m_6_10 = a6 * a10;
        let m_6_11 = a6 * a11;
        let m_6_12 = a6 * a12;
        let m_6_13 = a6 * a13;
        let m_7_0 = m_0_7;
        let m_7_1 = m_1_7;
        let m_7_2 = m_2_7;
        let m_7_3 = m_3_7;
        let m_7_4 = m_4_7;
        let m_7_5 = m_5_7;
        let m_7_6 = m_6_7;
        let m_7_7 = a7 * a7;
        let m_7_8 = a7 * a8;
        let m_7_9 = a7 * a9;
        let m_7_10 = a7 * a10;
        let m_7_11 = a7 * a11;
        let m_7_12 = a7 * a12;
        let m_7_13 = a7 * a13;
        let m_8_0 = m_0_8;
        let m_8_1 = m_1_8;
        let m_8_2 = m_2_8;
        let m_8_3 = m_3_8;
        let m_8_4 = m_4_8;
        let m_8_5 = m_5_8;
        let m_8_6 = m_6_8;
        let m_8_7 = m_7_8;
        let m_8_8 = a8 * a8;
        let m_8_9 = a8 * a9;
        let m_8_10 = a8 * a10;
        let m_8_11 = a8 * a11;
        let m_8_12 = a8 * a12;
        let m_8_13 = a8 * a13;
        let m_9_0 = m_0_9;
        let m_9_1 = m_1_9;
        let m_9_2 = m_2_9;
        let m_9_3 = m_3_9;
        let m_9_4 = m_4_9;
        let m_9_5 = m_5_9;
        let m_9_6 = m_6_9;
        let m_9_7 = m_7_9;
        let m_9_8 = m_8_9;
        let m_9_9 = a9 * a9;
        let m_9_10 = a9 * a10;
        let m_9_11 = a9 * a11;
        let m_9_12 = a9 * a12;
        let m_9_13 = a9 * a13;
        let m_10_0 = m_0_10;
        let m_10_1 = m_1_10;
        let m_10_2 = m_2_10;
        let m_10_3 = m_3_10;
        let m_10_4 = m_4_10;
        let m_10_5 = m_5_10;
        let m_10_6 = m_6_10;
        let m_10_7 = m_7_10;
        let m_10_8 = m_8_10;
        let m_10_9 = m_9_10;
        let m_10_10 = a10 * a10;
        let m_10_11 = a10 * a11;
        let m_10_12 = a10 * a12;
        let m_10_13 = a10 * a13;
        let m_11_0 = m_0_11;
        let m_11_1 = m_1_11;
        let m_11_2 = m_2_11;
        let m_11_3 = m_3_11;
        let m_11_4 = m_4_11;
        let m_11_5 = m_5_11;
        let m_11_6 = m_6_11;
        let m_11_7 = m_7_11;
        let m_11_8 = m_8_11;
        let m_11_9 = m_9_11;
        let m_11_10 = m_10_11;
        let m_11_11 = a11 * a11;
        let m_11_12 = a11 * a12;
        let m_11_13 = a11 * a13;
        let m_12_0 = m_0_12;
        let m_12_1 = m_1_12;
        let m_12_2 = m_2_12;
        let m_12_3 = m_3_12;
        let m_12_4 = m_4_12;
        let m_12_5 = m_5_12;
        let m_12_6 = m_6_12;
        let m_12_7 = m_7_12;
        let m_12_8 = m_8_12;
        let m_12_9 = m_9_12;
        let m_12_10 = m_10_12;
        let m_12_11 = m_11_12;
        let m_12_12 = a12 * a12;
        let m_12_13 = a12 * a13;
        let m_13_0 = m_0_13;
        let m_13_1 = m_1_13;
        let m_13_2 = m_2_13;
        let m_13_3 = m_3_13;
        let m_13_4 = m_4_13;
        let m_13_5 = m_5_13;
        let m_13_6 = m_6_13;
        let m_13_7 = m_7_13;
        let m_13_8 = m_8_13;
        let m_13_9 = m_9_13;
        let m_13_10 = m_10_13;
        let m_13_11 = m_11_13;
        let m_13_12 = m_12_13;
        let m_13_13 = a13 * a13;

        // Compute the 40-digit combined product using 64-bit operations.
        let d0 = m_0_0 + ((m_0_1 & 0x0fffffff) << 28) +
                 ((m_1_0 & 0x0fffffff) << 28);
        let c0 = d0 >> 56;
        let d1 = (m_0_1 >> 28) + m_0_2 + ((m_0_3 & 0x0fffffff) << 28) +
                 (m_1_0 >> 28) + m_1_1 + ((m_1_2 & 0x0fffffff) << 28) +
                 m_2_0 + ((m_2_1 & 0x0fffffff) << 28) +
                 ((m_3_0 & 0x0fffffff) << 28) + c0;
        let c1 = d1 >> 56;
        let d2 = (m_0_3 >> 28) + m_0_4 + ((m_0_5 & 0x0fffffff) << 28) +
                 (m_1_2 >> 28) + m_1_3 + ((m_1_4 & 0x0fffffff) << 28) +
                 (m_2_1 >> 28) + m_2_2 + ((m_2_3 & 0x0fffffff) << 28) +
                 (m_3_0 >> 28) + m_3_1 + ((m_3_2 & 0x0fffffff) << 28) +
                 m_4_0 + ((m_4_1 & 0x0fffffff) << 28) +
                 ((m_5_0 & 0x0fffffff) << 28) + c1;
        let c2 = d2 >> 56;
        let d3 = (m_0_5 >> 28) + m_0_6 + ((m_0_7 & 0x0fffffff) << 28) +
                 (m_1_4 >> 28) + m_1_5 + ((m_1_6 & 0x0fffffff) << 28) +
                 (m_2_3 >> 28) + m_2_4 + ((m_2_5 & 0x0fffffff) << 28) +
                 (m_3_2 >> 28) + m_3_3 + ((m_3_4 & 0x0fffffff) << 28) +
                 (m_4_1 >> 28) + m_4_2 + ((m_4_3 & 0x0fffffff) << 28) +
                 (m_5_0 >> 28) + m_5_1 + ((m_5_2 & 0x0fffffff) << 28) +
                 m_6_0 + ((m_6_1 & 0x0fffffff) << 28) +
                 ((m_7_0 & 0x0fffffff) << 28) + c2;
        let c3 = d3 >> 56;
        let d4 = (m_0_7 >> 28) + m_0_8 + ((m_0_9 & 0x0fffffff) << 28) +
                 (m_1_6 >> 28) + m_1_7 + ((m_1_8 & 0x0fffffff) << 28) +
                 (m_2_5 >> 28) + m_2_6 + ((m_2_7 & 0x0fffffff) << 28) +
                 (m_3_4 >> 28) + m_3_5 + ((m_3_6 & 0x0fffffff) << 28) +
                 (m_4_3 >> 28) + m_4_4 + ((m_4_5 & 0x0fffffff) << 28) +
                 (m_5_2 >> 28) + m_5_3 + ((m_5_4 & 0x0fffffff) << 28) +
                 (m_6_1 >> 28) + m_6_2 + ((m_6_3 & 0x0fffffff) << 28) +
                 (m_7_0 >> 28) + m_7_1 + ((m_7_2 & 0x0fffffff) << 28) +
                 m_8_0 + ((m_8_1 & 0x0fffffff) << 28) +
                 ((m_9_0 & 0x0fffffff) << 28) + c3;
        let c4 = d4 >> 56;
        let d5 = (m_0_9 >> 28) + m_0_10 + ((m_0_11 & 0x0fffffff) << 28) +
                 (m_1_8 >> 28) + m_1_9 + ((m_1_10 & 0x0fffffff) << 28) +
                 (m_2_7 >> 28) + m_2_8 + ((m_2_9 & 0x0fffffff) << 28) +
                 (m_3_6 >> 28) + m_3_7 + ((m_3_8 & 0x0fffffff) << 28) +
                 (m_4_5 >> 28) + m_4_6 + ((m_4_7 & 0x0fffffff) << 28) +
                 (m_5_4 >> 28) + m_5_5 + ((m_5_6 & 0x0fffffff) << 28) +
                 (m_6_3 >> 28) + m_6_4 + ((m_6_5 & 0x0fffffff) << 28) +
                 (m_7_2 >> 28) + m_7_3 + ((m_7_4 & 0x0fffffff) << 28) +
                 (m_8_1 >> 28) + m_8_2 + ((m_8_3 & 0x0fffffff) << 28) +
                 (m_9_0 >> 28) + m_9_1 + ((m_9_2 & 0x0fffffff) << 28) +
                 m_10_0 + ((m_10_1 & 0x0fffffff) << 28) +
                 ((m_11_0 & 0x0fffffff) << 28) + c4;
        let c5 = d5 >> 56;
        let d6 = (m_0_11 >> 28) + m_0_12 + ((m_0_13 & 0x0fffffff) << 28) +
                 (m_1_10 >> 28) + m_1_11 + ((m_1_12 & 0x0fffffff) << 28) +
                 (m_2_9 >> 28) + m_2_10 + ((m_2_11 & 0x0fffffff) << 28) +
                 (m_3_8 >> 28) + m_3_9 + ((m_3_10 & 0x0fffffff) << 28) +
                 (m_4_7 >> 28) + m_4_8 + ((m_4_9 & 0x0fffffff) << 28) +
                 (m_5_6 >> 28) + m_5_7 + ((m_5_8 & 0x0fffffff) << 28) +
                 (m_6_5 >> 28) + m_6_6 + ((m_6_7 & 0x0fffffff) << 28) +
                 (m_7_4 >> 28) + m_7_5 + ((m_7_6 & 0x0fffffff) << 28) +
                 (m_8_3 >> 28) + m_8_4 + ((m_8_5 & 0x0fffffff) << 28) +
                 (m_9_2 >> 28) + m_9_3 + ((m_9_4 & 0x0fffffff) << 28) +
                 (m_10_1 >> 28) + m_10_2 + ((m_10_3 & 0x0fffffff) << 28) +
                 (m_11_0 >> 28) + m_11_1 + ((m_11_2 & 0x0fffffff) << 28) +
                 m_12_0 + ((m_12_1 & 0x0fffffff) << 28) +
                 ((m_13_0 & 0x0fffffff) << 28) + c5;
        let c6 = d6 >> 56;
        let d7 = (m_0_13 >> 28) +
                 (m_1_12 >> 28) + m_1_13 +
                 (m_2_11 >> 28) + m_2_12 + ((m_2_13 & 0x0fffffff) << 28) +
                 (m_3_10 >> 28) + m_3_11 + ((m_3_12 & 0x0fffffff) << 28) +
                 (m_4_9 >> 28) + m_4_10 + ((m_4_11 & 0x0fffffff) << 28) +
                 (m_5_8 >> 28) + m_5_9 + ((m_5_10 & 0x0fffffff) << 28) +
                 (m_6_7 >> 28) + m_6_8 + ((m_6_9 & 0x0fffffff) << 28) +
                 (m_7_6 >> 28) + m_7_7 + ((m_7_8 & 0x0fffffff) << 28) +
                 (m_8_5 >> 28) + m_8_6 + ((m_8_7 & 0x0fffffff) << 28) +
                 (m_9_4 >> 28) + m_9_5 + ((m_9_6 & 0x0fffffff) << 28) +
                 (m_10_3 >> 28) + m_10_4 + ((m_10_5 & 0x0fffffff) << 28) +
                 (m_11_2 >> 28) + m_11_3 + ((m_11_4 & 0x0fffffff) << 28) +
                 (m_12_1 >> 28) + m_12_2 + ((m_12_3 & 0x0fffffff) << 28) +
                 (m_13_0 >> 28) + m_13_1 + ((m_13_2 & 0x0fffffff) << 28) +
                 c6;
        let c7 = d7 >> 56;
        let d8 = (m_2_13 >> 28) +
                 (m_3_12 >> 28) + m_3_13 +
                 (m_4_11 >> 28) + m_4_12 + ((m_4_13 & 0x0fffffff) << 28) +
                 (m_5_10 >> 28) + m_5_11 + ((m_5_12 & 0x0fffffff) << 28) +
                 (m_6_9 >> 28) + m_6_10 + ((m_6_11 & 0x0fffffff) << 28) +
                 (m_7_8 >> 28) + m_7_9 + ((m_7_10 & 0x0fffffff) << 28) +
                 (m_8_7 >> 28) + m_8_8 + ((m_8_9 & 0x0fffffff) << 28) +
                 (m_9_6 >> 28) + m_9_7 + ((m_9_8 & 0x0fffffff) << 28) +
                 (m_10_5 >> 28) + m_10_6 + ((m_10_7 & 0x0fffffff) << 28) +
                 (m_11_4 >> 28) + m_11_5 + ((m_11_6 & 0x0fffffff) << 28) +
                 (m_12_3 >> 28) + m_12_4 + ((m_12_5 & 0x0fffffff) << 28) +
                 (m_13_2 >> 28) + m_13_3 + ((m_13_4 & 0x0fffffff) << 28) +
                 c7;
        let c8 = d8 >> 56;
        let d9 = (m_4_13 >> 28) +
                 (m_5_12 >> 28) + m_5_13 +
                 (m_6_11 >> 28) + m_6_12 + ((m_6_13 & 0x0fffffff) << 28) +
                 (m_7_10 >> 28) + m_7_11 + ((m_7_12 & 0x0fffffff) << 28) +
                 (m_8_9 >> 28) + m_8_10 + ((m_8_11 & 0x0fffffff) << 28) +
                 (m_9_8 >> 28) + m_9_9 + ((m_9_10 & 0x0fffffff) << 28) +
                 (m_10_7 >> 28) + m_10_8 + ((m_10_9 & 0x0fffffff) << 28) +
                 (m_11_6 >> 28) + m_11_7 + ((m_11_8 & 0x0fffffff) << 28) +
                 (m_12_5 >> 28) + m_12_6 + ((m_12_7 & 0x0fffffff) << 28) +
                 (m_13_4 >> 28) + m_13_5 + ((m_13_6 & 0x0fffffff) << 28) +
                 c8;
        let c9 = d9 >> 56;
        let d10 = (m_6_13 >> 28) +
                  (m_7_12 >> 28) + m_7_13 +
                  (m_8_11 >> 28) + m_8_12 + ((m_8_13 & 0x0fffffff) << 28) +
                  (m_9_10 >> 28) + m_9_11 + ((m_9_12 & 0x0fffffff) << 28) +
                  (m_10_9 >> 28) + m_10_10 + ((m_10_11 & 0x0fffffff) << 28) +
                  (m_11_8 >> 28) + m_11_9 + ((m_11_10 & 0x0fffffff) << 28) +
                  (m_12_7 >> 28) + m_12_8 + ((m_12_9 & 0x0fffffff) << 28) +
                  (m_13_6 >> 28) + m_13_7 + ((m_13_8 & 0x0fffffff) << 28) +
                  c9;
        let c10 = d10 >> 56;
        let d11 = (m_8_13 >> 28) +
                  (m_9_12 >> 28) + m_9_13 +
                  (m_10_11 >> 28) + m_10_12 + ((m_10_13 & 0x0fffffff) << 28) +
                  (m_11_10 >> 28) + m_11_11 + ((m_11_12 & 0x0fffffff) << 28) +
                  (m_12_9 >> 28) + m_12_10 + ((m_12_11 & 0x0fffffff) << 28) +
                  (m_13_8 >> 28) + m_13_9 + ((m_13_10 & 0x0fffffff) << 28) +
                  c10;
        let c11 = d11 >> 56;
        let d12 = (m_10_13 >> 28) +
                  (m_11_12 >> 28) + m_11_13 +
                  (m_12_11 >> 28) + m_12_12 + ((m_12_13 & 0x0fffffff) << 28) +
                  (m_13_10 >> 28) + m_13_11 + ((m_13_12 & 0x0fffffff) << 28) +
                  c11;
        let c12 = d12 >> 56;
        let d13 = (m_12_13 >> 28) +
                  (m_13_12 >> 28) + m_13_13 +
                  c12;

        // Modular reduction by a pseudo-mersenne prime of the form 2^n - c.

        // These are the n low-order
        let l0_0 = d0 & 0x00ffffffffffffff;
        let l1_0 = d1 & 0x00ffffffffffffff;
        let l2_0 = d2 & 0x00ffffffffffffff;
        let l3_0 = d3 & 0x00ffffffffffffff;
        let l4_0 = d4 & 0x00ffffffffffffff;
        let l5_0 = d5 & 0x00ffffffffffffff;
        let l6_0 = d6 & 0x00003fffffffffff;

        // Shift the high bits down into another n-bit number.
        let h0_0 = ((d6 & 0x00ffffffffffffff) >> 46) |
                   ((d7 & 0x00003fffffffffff) << 10);
        let h1_0 = ((d7 & 0x00ffffffffffffff) >> 46) |
                   ((d8 & 0x00003fffffffffff) << 10);
        let h2_0 = ((d8 & 0x00ffffffffffffff) >> 46) |
                   ((d9 & 0x00003fffffffffff) << 10);
        let h3_0 = ((d9 & 0x00ffffffffffffff) >> 46) |
                   ((d10 & 0x00003fffffffffff) << 10);
        let h4_0 = ((d10 & 0x00ffffffffffffff) >> 46) |
                   ((d11 & 0x00003fffffffffff) << 10);
        let h5_0 = ((d11 & 0x00ffffffffffffff) >> 46) |
                   ((d12 & 0x00003fffffffffff) << 10);
        let h6_0 = ((d12 & 0x00ffffffffffffff) >> 46) |
                   (d13 << 10);

        // Multiply by C
        let hc0_0 = h0_0 * C_VAL;
        let hc1_0 = h1_0 * C_VAL;
        let hc2_0 = h2_0 * C_VAL;
        let hc3_0 = h3_0 * C_VAL;
        let hc4_0 = h4_0 * C_VAL;
        let hc5_0 = h5_0 * C_VAL;
        let hc6_0 = h6_0 * C_VAL;

        // Add h and l.
        let kin_0 = hc6_0 >> 46;
        let s0_0 = l0_0 + hc0_0 + (kin_0 * C_VAL);
        let k0_0 = s0_0 >> 56;
        let s1_0 = l1_0 + hc1_0 + k0_0;
        let k1_0 = s1_0 >> 56;
        let s2_0 = l2_0 + hc2_0 + k1_0;
        let k2_0 = s2_0 >> 56;
        let s3_0 = l3_0 + hc3_0 + k2_0;
        let k3_0 = s3_0 >> 56;
        let s4_0 = l4_0 + hc4_0 + k3_0;
        let k4_0 = s4_0 >> 56;
        let s5_0 = l5_0 + hc5_0 + k4_0;
        let k5_0 = s5_0 >> 56;
        let s6_0 = l6_0 + (hc6_0 & 0x00003fffffffffff) + k5_0;

        self.0[0] = s0_0 & 0x00ffffffffffffff;
        self.0[1] = s1_0 & 0x00ffffffffffffff;
        self.0[2] = s2_0 & 0x00ffffffffffffff;
        self.0[3] = s3_0 & 0x00ffffffffffffff;
        self.0[4] = s4_0 & 0x00ffffffffffffff;
        self.0[5] = s5_0 & 0x00ffffffffffffff;
        self.0[6] = s6_0;
    }

    fn squared(&self) -> Self {
        let mut out = self.clone();

        out.square();

        out
    }

    fn invert(&mut self) {
        // First digit is 1.
        let mut sqval = self.clone();

        // Second digit is 0.
        sqval.square();

        // Third digit is 1.
        sqval.square();
        *self *= &sqval;

        // Fourth digit is 0.
        sqval.square();

        // Fifth digit is 1.
        sqval.square();
        *self *= &sqval;

        // Fifth and sixth digits are 0.
        sqval.square();
        sqval.square();

        // All the remaining digits are 1.
        for _ in 7..382 {
            sqval.square();
            *self *= &sqval;
        }
    }

    fn inverted(&self) -> Self {
        let mut out = self.clone();

        out.invert();

        out
    }

    fn legendre(&self) -> Self {
        // First digit is 1.
        let mut out = self.clone();
        let mut sqval = out.clone();

        // Second digit is 1.
        sqval.square();
        out *= &sqval;

        // Third digit is 0.
        sqval.square();

        // Fourth digit is 1.
        sqval.square();
        out *= &sqval;

        // Fifth and sixth digits are 0.
        sqval.square();
        sqval.square();

        // All the remaining digits are 1.
        for _ in 6..381 {
            sqval.square();
            out *= &sqval;
        }

        out
    }

    fn sqrt(&self) -> Self {
        // First digit is 0.

        // Second digit is 1.
        let mut out = self.squared();
        let mut sqval = out.clone();

        // Third digit is 1.
        sqval.square();
        out *= &sqval;

        // Fourth and fifth digits are 0.
        sqval.square();
        sqval.square();

        // All the remaining digits are 1.
        for _ in 5..380 {
            sqval.square();
            out *= &sqval;
        }

        out
    }

    fn small_add_assign(&mut self, rhs: i32) {
        let a0 = self.0[0];
        let a1 = self.0[1];
        let a2 = self.0[2];
        let a3 = self.0[3];
        let a4 = self.0[4];
        let a5 = self.0[5];
        let a6 = self.0[6] & 0x00003fffffffffff;

        let b = i64::from(rhs);

        let cin = self.carry_out();
        let s0 = a0 + b + (cin * C_VAL);
        let c0 = s0 >> 56;
        let s1 = a1 + c0;
        let c1 = s1 >> 56;
        let s2 = a2 + c1;
        let c2 = s2 >> 56;
        let s3 = a3 + c2;
        let c3 = s3 >> 56;
        let s4 = a4 + c3;
        let c4 = s4 >> 56;
        let s5 = a5 + c4;
        let c5 = s5 >> 56;
        let s6 = a6 + c5;

        self.0[0] = s0 & 0x00ffffffffffffff;
        self.0[1] = s1 & 0x00ffffffffffffff;
        self.0[2] = s2 & 0x00ffffffffffffff;
        self.0[3] = s3 & 0x00ffffffffffffff;
        self.0[4] = s4 & 0x00ffffffffffffff;
        self.0[5] = s5 & 0x00ffffffffffffff;
        self.0[6] = s6;
    }

    fn small_add(&self, rhs: i32) -> Mod_e382_105 {
        let mut out = self.clone();

        out.small_add_assign(rhs);

        out
    }

    fn small_sub_assign(&mut self, rhs: i32) {
        let a0 = self.0[0];
        let a1 = self.0[1];
        let a2 = self.0[2];
        let a3 = self.0[3];
        let a4 = self.0[4];
        let a5 = self.0[5];
        let a6 = self.0[6] & 0x00003fffffffffff;

        let b = i64::from(rhs);

        let cin = self.carry_out();
        let s0 = a0 - b + (cin * C_VAL);
        let c0 = s0 >> 56;
        let s1 = a1 + c0;
        let c1 = s1 >> 56;
        let s2 = a2 + c1;
        let c2 = s2 >> 56;
        let s3 = a3 + c2;
        let c3 = s3 >> 56;
        let s4 = a4 + c3;
        let c4 = s4 >> 56;
        let s5 = a5 + c4;
        let c5 = s5 >> 56;
        let s6 = a6 + c5;

        self.0[0] = s0 & 0x00ffffffffffffff;
        self.0[1] = s1 & 0x00ffffffffffffff;
        self.0[2] = s2 & 0x00ffffffffffffff;
        self.0[3] = s3 & 0x00ffffffffffffff;
        self.0[4] = s4 & 0x00ffffffffffffff;
        self.0[5] = s5 & 0x00ffffffffffffff;
        self.0[6] = s6;
    }

    fn small_sub(&self, rhs: i32) -> Mod_e382_105 {
        let mut out = self.clone();

        out.small_sub_assign(rhs);

        out
    }

    fn small_mul_assign(&mut self, rhs: i32) {
        let a0 = self.0[0] & 0x0fffffff;
        let a1 = self.0[0] >> 28;
        let a2 = self.0[1] & 0x0fffffff;
        let a3 = self.0[1] >> 28;
        let a4 = self.0[2] & 0x0fffffff;
        let a5 = self.0[2] >> 28;
        let a6 = self.0[3] & 0x0fffffff;
        let a7 = self.0[3] >> 28;
        let a8 = self.0[4] & 0x0fffffff;
        let a9 = self.0[4] >> 28;
        let a10 = self.0[5] & 0x0fffffff;
        let a11 = self.0[5] >> 28;
        let a12 = self.0[6] & 0x0fffffff;
        let a13 = self.0[6] >> 28;

        let b = i64::from(rhs);

        let m0 = a0 * b;
        let m1 = a1 * b;
        let m2 = a2 * b;
        let m3 = a3 * b;
        let m4 = a4 * b;
        let m5 = a5 * b;
        let m6 = a6 * b;
        let m7 = a7 * b;
        let m8 = a8 * b;
        let m9 = a9 * b;
        let m10 = a10 * b;
        let m11 = a11 * b;
        let m12 = a12 * b;
        let m13 = a13 * b;

        let cin = self.carry_out();
        let d0 = m0 + ((m1 & 0x0fffffff) << 28) + (cin * C_VAL);
        let c0 = d0 >> 56;
        let d1 = (m1 >> 28) + m2 + ((m3 & 0x0fffffff) << 28) + c0;
        let c1 = d1 >> 56;
        let d2 = (m3 >> 28) + m4 + ((m5 & 0x0fffffff) << 28) + c1;
        let c2 = d2 >> 56;
        let d3 = (m5 >> 28) + m6 + ((m7 & 0x0fffffff) << 28) + c2;
        let c3 = d3 >> 56;
        let d4 = (m7 >> 28) + m8 + ((m9 & 0x0fffffff) << 28) + c3;
        let c4 = d4 >> 56;
        let d5 = (m9 >> 28) + m10 + ((m11 & 0x0fffffff) << 28) + c4;
        let c5 = d5 >> 56;
        let d6 = (m11 >> 28) + m12 + (m13 << 28) + c5;

        self.0[0] = d0 & 0x00ffffffffffffff;
        self.0[1] = d1 & 0x00ffffffffffffff;
        self.0[2] = d2 & 0x00ffffffffffffff;
        self.0[3] = d3 & 0x00ffffffffffffff;
        self.0[4] = d4 & 0x00ffffffffffffff;
        self.0[5] = d5 & 0x00ffffffffffffff;
        self.0[6] = d6;
    }

    fn small_mul(&self, b: i32) -> Mod_e382_105 {
        let mut out = self.clone();

        out.small_mul_assign(b);

        out
    }
}

#[cfg(test)]
mod tests {
    use field::prime_field::*;
    use field::mod_e382_105::*;

    const TWO: Mod_e382_105 = Mod_e382_105([ 2, 0, 0, 0, 0, 0, 0 ]);

    const M_TWO: Mod_e382_105 =
        Mod_e382_105([ 0x00ffffffffffff95, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00003fffffffffff ]);

    const THREE: Mod_e382_105 = Mod_e382_105([ 3, 0, 0, 0, 0, 0, 0 ]);

    const M_THREE: Mod_e382_105 =
        Mod_e382_105([ 0x00ffffffffffff94, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00003fffffffffff ]);

    const FOUR: Mod_e382_105 = Mod_e382_105([ 4, 0, 0, 0, 0, 0, 0 ]);

    const M_FOUR: Mod_e382_105 =
        Mod_e382_105([ 0x00ffffffffffff93, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00003fffffffffff ]);

    const SIX: Mod_e382_105 = Mod_e382_105([ 6, 0, 0, 0, 0, 0, 0 ]);

    const M_SIX: Mod_e382_105 =
        Mod_e382_105([ 0x00ffffffffffff91, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00003fffffffffff ]);

    const EIGHT: Mod_e382_105 = Mod_e382_105([ 8, 0, 0, 0, 0, 0, 0 ]);

    const M_EIGHT: Mod_e382_105 =
        Mod_e382_105([ 0x00ffffffffffff8f, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00003fffffffffff ]);

    const NINE: Mod_e382_105 = Mod_e382_105([ 9, 0, 0, 0, 0, 0, 0 ]);

    const M_NINE: Mod_e382_105 =
        Mod_e382_105([ 0x00ffffffffffff8e, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00003fffffffffff ]);

    const SIXTEEN: Mod_e382_105 = Mod_e382_105([ 16, 0, 0, 0, 0, 0, 0 ]);

    const M_SIXTEEN: Mod_e382_105 =
        Mod_e382_105([ 0x00ffffffffffff87, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00ffffffffffffff, 0x00ffffffffffffff,
                       0x00003fffffffffff ]);

    fn test_pack_unpack(expected: &[u8; 48]) {
        let mut unpacked = Mod_e382_105::unpacked(expected);
        let actual = unpacked.packed();

        for i in 0..48 {
            assert!(expected[i] == actual[i]);
        }
    }

    fn test_unpack_pack(expected: &mut Mod_e382_105) {
        let bytes = expected.packed();
        let mut actual = Mod_e382_105::unpacked(&bytes);

        assert!(expected.normalize_eq(&mut actual));
    }

    #[test]
    fn pack_unpack_test() {
        test_pack_unpack(&[0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00]);
        test_pack_unpack(&[0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0x3f]);
        test_pack_unpack(&[0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0x2a]);
        test_pack_unpack(&[0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x15]);
        test_pack_unpack(&[0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0x2a]);
        test_pack_unpack(&[0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00]);
        test_pack_unpack(&[0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0x3f]);
        test_pack_unpack(&[0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x15]);
    }

    #[test]
    fn unpack_pack_test() {
        test_unpack_pack(&mut ZERO.clone());
        test_unpack_pack(&mut ONE.clone());
        test_unpack_pack(&mut M_ONE.clone());
        test_unpack_pack(&mut Mod_e382_105([ 0x00ffffffffffffff,
                                             0x0000000000000000,
                                             0x00ffffffffffffff,
                                             0x0000000000000000,
                                             0x00ffffffffffffff,
                                             0x0000000000000000,
                                             0x00003fffffffffff ]));
        test_unpack_pack(&mut Mod_e382_105([ 0x0000000000000000,
                                             0x00ffffffffffffff,
                                             0x0000000000000000,
                                             0x00ffffffffffffff,
                                             0x0000000000000000,
                                             0x00ffffffffffffff,
                                             0x0000000000000000 ]));
        test_unpack_pack(&mut Mod_e382_105([ 0x00aaaaaaaaaaaaaa,
                                             0x0055555555555555,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0055555555555555,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0055555555555555,
                                             0x00002aaaaaaaaaaa ]));
        test_unpack_pack(&mut Mod_e382_105([ 0x0055555555555555,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0055555555555555,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0055555555555555,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0000155555555555 ]));
        test_unpack_pack(&mut Mod_e382_105([ 0x00aaaaaaaaaaaaaa,
                                             0x0000000000000000,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0000000000000000,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0000000000000000,
                                             0x00002aaaaaaaaaaa ]));
        test_unpack_pack(&mut Mod_e382_105([ 0x0000000000000000,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0000000000000000,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0000000000000000,
                                             0x00aaaaaaaaaaaaaa,
                                             0x0000000000000000 ]));
        test_unpack_pack(&mut Mod_e382_105([ 0x00ffffffffffffff,
                                             0x0055555555555555,
                                             0x00ffffffffffffff,
                                             0x0055555555555555,
                                             0x00ffffffffffffff,
                                             0x0055555555555555,
                                             0x00003fffffffffff ]));
        test_unpack_pack(&mut Mod_e382_105([ 0x0055555555555555,
                                             0x00ffffffffffffff,
                                             0x0055555555555555,
                                             0x00ffffffffffffff,
                                             0x0055555555555555,
                                             0x00ffffffffffffff,
                                             0x0000155555555555 ]));
    }

    #[test]
    fn test_add() {
        let l1_zeros: [&mut Mod_e382_105; 5] = [ &mut (&ZERO + &ZERO),
                                                 &mut (&M_ONE + &ONE),
                                                 &mut (&ONE + &M_ONE),
                                                 &mut (&M_TWO + &TWO),
                                                 &mut (&TWO + &M_TWO) ];

        let l1_ones: [&mut Mod_e382_105; 4] = [ &mut (&ZERO + &ONE),
                                                &mut (&ONE + &ZERO),
                                                &mut (&M_ONE + &TWO),
                                                &mut (&TWO + &M_ONE) ];

        let l1_twos: [&mut Mod_e382_105; 3] = [ &mut (&ZERO + &TWO),
                                                &mut (&ONE + &ONE),
                                                &mut (&TWO + &ZERO) ];

        let l1_mones: [&mut Mod_e382_105; 4] = [ &mut (&ZERO + &M_ONE),
                                                 &mut (&M_ONE + &ZERO),
                                                 &mut (&M_TWO + &ONE),
                                                 &mut (&ONE + &M_TWO) ];

        let l1_mtwos: [&mut Mod_e382_105; 3] = [ &mut (&ZERO + &M_TWO),
                                                 &mut (&M_ONE + &M_ONE),
                                                 &mut (&M_TWO + &ZERO) ];

        for i in 0..5 {
            assert!(ZERO.normalize_eq(l1_zeros[i]));
        }

        for i in 0..4 {
            assert!(ONE.normalize_eq(l1_ones[i]));
        }

        for i in 0..3 {
            assert!(TWO.normalize_eq(l1_twos[i]));
        }

        for i in 0..4 {
            assert!(M_ONE.normalize_eq(l1_mones[i]));
        }

        for i in 0..3 {
            assert!(M_TWO.normalize_eq(l1_mtwos[i]));
        }

        for i in 0..5 {
            for j in 0..5 {
                let mut val = &*l1_zeros[i] + &*l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = &*l1_mones[i] + &*l1_ones[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = &*l1_ones[i] + &*l1_mones[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_mtwos[i] + &*l1_twos[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_twos[i] + &*l1_mtwos[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..4 {
                let mut val = &*l1_zeros[i] + &*l1_ones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..5 {
                let mut val = &*l1_ones[i] + &*l1_zeros[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..3 {
                let mut val = &*l1_mones[i] + &*l1_twos[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = &*l1_twos[i] + &*l1_mones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..3 {
                let mut val = &*l1_zeros[i] + &*l1_twos[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = &*l1_ones[i] + &*l1_ones[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..5 {
                let mut val = &*l1_twos[i] + &*l1_zeros[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..4 {
                let mut val = &*l1_zeros[i] + &*l1_mones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..5 {
                let mut val = &*l1_mones[i] + &*l1_zeros[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = &*l1_mtwos[i] + &*l1_ones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..3 {
                let mut val = &*l1_ones[i] + &*l1_mtwos[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..3 {
                let mut val = &*l1_zeros[i] + &*l1_mtwos[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = &*l1_mones[i] + &*l1_mones[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..5 {
                let mut val = &*l1_mtwos[i] + &*l1_zeros[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }
    }

    #[test]
    fn test_sub() {
        let l1_zeros: [&mut Mod_e382_105; 3] = [ &mut (&ZERO - &ZERO),
                                                 &mut (&ONE - &ONE),
                                                 &mut (&TWO - &TWO) ];

        let l1_ones: [&mut Mod_e382_105; 4] = [ &mut (&ZERO - &M_ONE),
                                                &mut (&ONE - &ZERO),
                                                &mut (&M_ONE - &M_TWO),
                                                &mut (&TWO - &ONE) ];

        let l1_twos: [&mut Mod_e382_105; 3] = [ &mut (&ZERO - &M_TWO),
                                                &mut (&ONE - &M_ONE),
                                                &mut (&TWO - &ZERO) ];

        let l1_mones: [&mut Mod_e382_105; 4] = [ &mut (&ZERO - &ONE),
                                                 &mut (&M_ONE - &ZERO),
                                                 &mut (&M_TWO - &M_ONE),
                                                 &mut (&ONE - &TWO) ];

        let l1_mtwos: [&mut Mod_e382_105; 3] = [ &mut (&ZERO - &TWO),
                                                 &mut (&M_ONE - &ONE),
                                                 &mut (&M_TWO - &ZERO) ];

        for i in 0..3 {
            assert!(ZERO.normalize_eq(l1_zeros[i]));
        }

        for i in 0..4 {
            assert!(ONE.normalize_eq(l1_ones[i]));
        }

        for i in 0..3 {
            assert!(TWO.normalize_eq(l1_twos[i]));
        }

        for i in 0..4 {
            assert!(M_ONE.normalize_eq(l1_mones[i]));
        }

        for i in 0..3 {
            assert!(M_TWO.normalize_eq(l1_mtwos[i]));
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_zeros[i] - &*l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = &*l1_ones[i] - &*l1_ones[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_twos[i] - &*l1_twos[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = &*l1_zeros[i] - &*l1_mones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..3 {
                let mut val = &*l1_ones[i] - &*l1_zeros[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_mones[i] - &*l1_mtwos[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = &*l1_twos[i] - &*l1_ones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_zeros[i] - &*l1_mtwos[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = &*l1_ones[i] - &*l1_mones[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_twos[i] - &*l1_zeros[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = &*l1_zeros[i] - &*l1_ones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..3 {
                let mut val = &*l1_mones[i] - &*l1_zeros[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = &*l1_mtwos[i] - &*l1_mones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..3 {
                let mut val = &*l1_ones[i] - &*l1_twos[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_zeros[i] - &*l1_twos[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = &*l1_mones[i] - &*l1_ones[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = &*l1_mtwos[i] - &*l1_zeros[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

    }

    #[test]
    fn test_mul() {
        let l1_zeros: [&mut Mod_e382_105; 9] = [ &mut (&ZERO * &ZERO),
                                                 &mut (&ONE * &ZERO),
                                                 &mut (&TWO * &ZERO),
                                                 &mut (&M_ONE * &ZERO),
                                                 &mut (&M_TWO * &ZERO),
                                                 &mut (&ZERO * &ONE),
                                                 &mut (&ZERO * &TWO),
                                                 &mut (&ZERO * &M_ONE),
                                                 &mut (&ZERO * &M_TWO) ];

        let l1_ones: [&mut Mod_e382_105; 2] = [ &mut (&ONE * &ONE),
                                                &mut (&M_ONE * &M_ONE) ];

        let l1_twos: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &TWO),
                                                &mut (&TWO * &ONE),
                                                &mut (&M_ONE * &M_TWO),
                                                &mut (&M_TWO * &M_ONE) ];

        let l1_fours: [&mut Mod_e382_105; 2] = [ &mut (&TWO * &TWO),
                                                 &mut (&M_TWO * &M_TWO) ];

        let l1_mones: [&mut Mod_e382_105; 2] = [ &mut (&ONE * &M_ONE),
                                                 &mut (&M_ONE * &ONE) ];

        let l1_mtwos: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &M_TWO),
                                                 &mut (&TWO * &M_ONE),
                                                 &mut (&M_ONE * &TWO),
                                                 &mut (&M_TWO * &ONE) ];

        let l1_mfours: [&mut Mod_e382_105; 2] = [ &mut (&TWO * &M_TWO),
                                                  &mut (&M_TWO * &TWO) ];

        for i in 0..9 {
            assert!(ZERO.normalize_eq(l1_zeros[i]));
        }

        for i in 0..2 {
            assert!(ONE.normalize_eq(l1_ones[i]));
        }

        for i in 0..4 {
            assert!(TWO.normalize_eq(l1_twos[i]));
        }

        for i in 0..2 {
            assert!(FOUR.normalize_eq(l1_fours[i]));
        }

        for i in 0..2 {
            assert!(M_ONE.normalize_eq(l1_mones[i]));
        }

        for i in 0..4 {
            assert!(M_TWO.normalize_eq(l1_mtwos[i]));
        }

        for i in 0..2 {
            assert!(M_FOUR.normalize_eq(l1_mfours[i]));
        }

        for i in 0..9 {
            for j in 0..9 {
                let mut val = &*l1_zeros[i] * &*l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..9 {
                let mut val = &*l1_ones[i] * &*l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..9 {
                let mut val = &*l1_twos[i] * &*l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..9 {
                let mut val = &*l1_mones[i] * &*l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..9 {
                let mut val = &*l1_mtwos[i] * &*l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..9 {
            for j in 0..2 {
                let mut val = &*l1_zeros[i] * &*l1_ones[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..9 {
            for j in 0..4 {
                let mut val = &*l1_zeros[i] * &*l1_twos[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..9 {
            for j in 0..2 {
                let mut val = &*l1_zeros[i] * &*l1_mones[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..9 {
            for j in 0..4 {
                let mut val = &*l1_zeros[i] * &*l1_mtwos[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_ones[i] * &*l1_ones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_mones[i] * &*l1_mones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..4 {
                let mut val = &*l1_ones[i] * &*l1_twos[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..2 {
                let mut val = &*l1_twos[i] * &*l1_ones[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..4 {
                let mut val = &*l1_mones[i] * &*l1_mtwos[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..2 {
                let mut val = &*l1_mtwos[i] * &*l1_mones[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_twos[i] * &*l1_twos[j];

                assert!(FOUR.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_mtwos[i] * &*l1_mtwos[j];

                assert!(FOUR.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_ones[i] * &*l1_mones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_mones[i] * &*l1_ones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..4 {
                let mut val = &*l1_ones[i] * &*l1_mtwos[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..2 {
                let mut val = &*l1_twos[i] * &*l1_mones[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..4 {
                let mut val = &*l1_mones[i] * &*l1_twos[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..2 {
                let mut val = &*l1_mtwos[i] * &*l1_ones[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_twos[i] * &*l1_mtwos[j];

                assert!(M_FOUR.normalize_eq(&mut val));
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                let mut val = &*l1_mtwos[i] * &*l1_twos[j];

                assert!(M_FOUR.normalize_eq(&mut val));
            }
        }
    }

    #[test]
    fn test_square() {
        let l1_zeros: [&mut Mod_e382_105; 10] = [ &mut (&ZERO * &ZERO),
                                                  &mut (&ONE * &ZERO),
                                                  &mut (&TWO * &ZERO),
                                                  &mut (&M_ONE * &ZERO),
                                                  &mut (&M_TWO * &ZERO),
                                                  &mut (&ZERO * &ONE),
                                                  &mut (&ZERO * &TWO),
                                                  &mut (&ZERO * &M_ONE),
                                                  &mut (&ZERO * &M_TWO),
                                                  &mut ZERO.squared() ];

        let l1_ones: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &ONE),
                                                &mut (&M_ONE * &M_ONE),
                                                &mut ONE.squared(),
                                                &mut M_ONE.squared() ];

        let l1_twos: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &TWO),
                                                &mut (&TWO * &ONE),
                                                &mut (&M_ONE * &M_TWO),
                                                &mut (&M_TWO * &M_ONE) ];

        let l1_threes: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &THREE),
                                                  &mut (&THREE * &ONE),
                                                  &mut (&M_ONE * &M_THREE),
                                                  &mut (&M_THREE * &M_ONE) ];

        let l1_fours: [&mut Mod_e382_105; 4] = [ &mut (&TWO * &TWO),
                                                 &mut (&M_TWO * &M_TWO),
                                                 &mut TWO.squared(),
                                                 &mut M_TWO.squared() ];

        for i in 0..10 {
            let mut val = l1_zeros[i].squared();

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_ones[i].squared();

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_twos[i].squared();

            assert!(FOUR.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_threes[i].squared();

            assert!(NINE.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_fours[i].squared();

            assert!(SIXTEEN.normalize_eq(&mut val));
        }
    }

    #[test]
    fn test_inv() {
        let l1_ones: [&mut Mod_e382_105; 2] = [ &mut (&ONE * &ONE),
                                                &mut (&M_ONE * &M_ONE) ];

        let l1_twos: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &TWO),
                                                &mut (&TWO * &ONE),
                                                &mut (&M_ONE * &M_TWO),
                                                &mut (&M_TWO * &M_ONE) ];

        let l1_threes: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &THREE),
                                                  &mut (&THREE * &ONE),
                                                  &mut (&M_ONE * &M_THREE),
                                                  &mut (&M_THREE * &M_ONE) ];

        let l1_fours: [&mut Mod_e382_105; 2] = [ &mut (&TWO * &TWO),
                                                 &mut (&M_TWO * &M_TWO) ];

        let l1_mones: [&mut Mod_e382_105; 2] = [ &mut (&ONE * &M_ONE),
                                                 &mut (&M_ONE * &ONE) ];

        let l1_mtwos: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &M_TWO),
                                                 &mut (&TWO * &M_ONE),
                                                 &mut (&M_ONE * &TWO),
                                                 &mut (&M_TWO * &ONE) ];

        let l1_mthrees: [&mut Mod_e382_105; 4] = [ &mut (&ONE * &M_THREE),
                                                   &mut (&THREE * &M_ONE),
                                                   &mut (&M_ONE * &THREE),
                                                   &mut (&M_THREE * &ONE) ];

        let l1_mfours: [&mut Mod_e382_105; 2] = [ &mut (&TWO * &M_TWO),
                                                  &mut (&M_TWO * &TWO) ];

        for i in 0..2 {
            let inv = l1_ones[i].inverted();
            let mut val = &*l1_ones[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let inv = l1_twos[i].inverted();
            let mut val = &*l1_twos[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let inv = l1_threes[i].inverted();
            let mut val = &*l1_threes[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let inv = l1_fours[i].inverted();
            let mut val = &*l1_fours[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let inv = l1_mones[i].inverted();
            let mut val = &*l1_mones[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let inv = l1_mtwos[i].inverted();
            let mut val = &*l1_mtwos[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let inv = l1_mthrees[i].inverted();
            let mut val = &*l1_mthrees[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let inv = l1_mfours[i].inverted();
            let mut val = &*l1_mfours[i] * &inv;

            assert!(ONE.normalize_eq(&mut val));
        }
    }

    #[test]
    fn test_div() {
        let l1_ones: [&mut Mod_e382_105; 12] = [ &mut (&ONE / &ONE),
                                                 &mut (&M_ONE / &M_ONE),
                                                 &mut (&TWO / &TWO),
                                                 &mut (&M_TWO / &M_TWO),
                                                 &mut (&THREE / &THREE),
                                                 &mut (&M_THREE / &M_THREE),
                                                 &mut (&FOUR / &FOUR),
                                                 &mut (&M_FOUR / &M_FOUR),
                                                 &mut (&NINE / &NINE),
                                                 &mut (&M_NINE / &M_NINE),
                                                 &mut (&SIXTEEN / &SIXTEEN),
                                                 &mut (&M_SIXTEEN / &M_SIXTEEN) ];

        let l1_twos: [&mut Mod_e382_105; 10] = [ &mut (&TWO / &ONE),
                                                 &mut (&M_TWO / &M_ONE),
                                                 &mut (&FOUR / &TWO),
                                                 &mut (&M_FOUR / &M_TWO),
                                                 &mut (&SIX / &THREE),
                                                 &mut (&M_SIX / &M_THREE),
                                                 &mut (&EIGHT / &FOUR),
                                                 &mut (&M_EIGHT / &M_FOUR),
                                                 &mut (&SIXTEEN / &EIGHT),
                                                 &mut (&M_SIXTEEN / &M_EIGHT) ];

        let l1_threes: [&mut Mod_e382_105; 6] = [ &mut (&THREE / &ONE),
                                                  &mut (&M_THREE / &M_ONE),
                                                  &mut (&SIX / &TWO),
                                                  &mut (&M_SIX / &M_TWO),
                                                  &mut (&NINE / &THREE),
                                                  &mut (&M_NINE / &M_THREE) ];

        let l1_fours: [&mut Mod_e382_105; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e382_105; 12] = [ &mut (&ONE / &M_ONE),
                                                  &mut (&M_ONE / &ONE),
                                                  &mut (&TWO / &M_TWO),
                                                  &mut (&M_TWO / &TWO),
                                                  &mut (&THREE / &M_THREE),
                                                  &mut (&M_THREE / &THREE),
                                                  &mut (&FOUR / &M_FOUR),
                                                  &mut (&M_FOUR / &FOUR),
                                                  &mut (&NINE / &M_NINE),
                                                  &mut (&M_NINE / &NINE),
                                                  &mut (&SIXTEEN / &M_SIXTEEN),
                                                  &mut (&M_SIXTEEN / &SIXTEEN) ];

        let l1_mtwos: [&mut Mod_e382_105; 10] = [ &mut (&TWO / &M_ONE),
                                                  &mut (&M_TWO / &ONE),
                                                  &mut (&FOUR / &M_TWO),
                                                  &mut (&M_FOUR / &TWO),
                                                  &mut (&SIX / &M_THREE),
                                                  &mut (&M_SIX / &THREE),
                                                  &mut (&EIGHT / &M_FOUR),
                                                  &mut (&M_EIGHT / &FOUR),
                                                  &mut (&SIXTEEN / &M_EIGHT),
                                                  &mut (&M_SIXTEEN / &EIGHT) ];

        let l1_mthrees: [&mut Mod_e382_105; 6] = [ &mut (&THREE / &M_ONE),
                                                   &mut (&M_THREE / &ONE),
                                                   &mut (&SIX / &M_TWO),
                                                   &mut (&M_SIX / &TWO),
                                                   &mut (&NINE / &M_THREE),
                                                   &mut (&M_NINE / &THREE) ];

        let l1_mfours: [&mut Mod_e382_105; 6] = [ &mut (&FOUR / &M_ONE),
                                                  &mut (&M_FOUR / &ONE),
                                                  &mut (&EIGHT / &M_TWO),
                                                  &mut (&M_EIGHT / &TWO),
                                                  &mut (&SIXTEEN / &M_FOUR),
                                                  &mut (&M_SIXTEEN / &FOUR) ];

        for i in 0..12 {
            assert!(ONE.normalize_eq(l1_ones[i]));
        }

        for i in 0..10 {
            assert!(TWO.normalize_eq(l1_twos[i]));
        }

        for i in 0..6 {
            assert!(THREE.normalize_eq(l1_threes[i]));
        }

        for i in 0..6 {
            assert!(FOUR.normalize_eq(l1_fours[i]));
        }

        for i in 0..12 {
            assert!(M_ONE.normalize_eq(l1_mones[i]));
        }

        for i in 0..10 {
            assert!(M_TWO.normalize_eq(l1_mtwos[i]));
        }

        for i in 0..6 {
            assert!(M_THREE.normalize_eq(l1_mthrees[i]));
        }

        for i in 0..6 {
            assert!(M_FOUR.normalize_eq(l1_mfours[i]));
        }
    }

    #[test]
    fn test_legendre() {
        let l1_zeros: [&mut Mod_e382_105; 10] = [ &mut (&ZERO * &ZERO),
                                                  &mut (&ONE * &ZERO),
                                                  &mut (&TWO * &ZERO),
                                                  &mut (&M_ONE * &ZERO),
                                                  &mut (&M_TWO * &ZERO),
                                                  &mut (&ZERO * &ONE),
                                                  &mut (&ZERO * &TWO),
                                                  &mut (&ZERO * &M_ONE),
                                                  &mut (&ZERO * &M_TWO),
                                                  &mut ZERO.squared() ];

        let l1_ones: [&mut Mod_e382_105; 12] = [ &mut (&ONE / &ONE),
                                                 &mut (&M_ONE / &M_ONE),
                                                 &mut (&TWO / &TWO),
                                                 &mut (&M_TWO / &M_TWO),
                                                 &mut (&THREE / &THREE),
                                                 &mut (&M_THREE / &M_THREE),
                                                 &mut (&FOUR / &FOUR),
                                                 &mut (&M_FOUR / &M_FOUR),
                                                 &mut (&NINE / &NINE),
                                                 &mut (&M_NINE / &M_NINE),
                                                 &mut (&SIXTEEN / &SIXTEEN),
                                                 &mut (&M_SIXTEEN / &M_SIXTEEN) ];

        let l1_twos: [&mut Mod_e382_105; 10] = [ &mut (&TWO / &ONE),
                                                 &mut (&M_TWO / &M_ONE),
                                                 &mut (&FOUR / &TWO),
                                                 &mut (&M_FOUR / &M_TWO),
                                                 &mut (&SIX / &THREE),
                                                 &mut (&M_SIX / &M_THREE),
                                                 &mut (&EIGHT / &FOUR),
                                                 &mut (&M_EIGHT / &M_FOUR),
                                                 &mut (&SIXTEEN / &EIGHT),
                                                 &mut (&M_SIXTEEN / &M_EIGHT) ];

        let l1_threes: [&mut Mod_e382_105; 6] = [ &mut (&THREE / &ONE),
                                                  &mut (&M_THREE / &M_ONE),
                                                  &mut (&SIX / &TWO),
                                                  &mut (&M_SIX / &M_TWO),
                                                  &mut (&NINE / &THREE),
                                                  &mut (&M_NINE / &M_THREE) ];

        let l1_fours: [&mut Mod_e382_105; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e382_105; 12] = [ &mut (&ONE / &M_ONE),
                                                  &mut (&M_ONE / &ONE),
                                                  &mut (&TWO / &M_TWO),
                                                  &mut (&M_TWO / &TWO),
                                                  &mut (&THREE / &M_THREE),
                                                  &mut (&M_THREE / &THREE),
                                                  &mut (&FOUR / &M_FOUR),
                                                  &mut (&M_FOUR / &FOUR),
                                                  &mut (&NINE / &M_NINE),
                                                  &mut (&M_NINE / &NINE),
                                                  &mut (&SIXTEEN / &M_SIXTEEN),
                                                  &mut (&M_SIXTEEN / &SIXTEEN) ];

        let l1_mtwos: [&mut Mod_e382_105; 10] = [ &mut (&TWO / &M_ONE),
                                                  &mut (&M_TWO / &ONE),
                                                  &mut (&FOUR / &M_TWO),
                                                  &mut (&M_FOUR / &TWO),
                                                  &mut (&SIX / &M_THREE),
                                                  &mut (&M_SIX / &THREE),
                                                  &mut (&EIGHT / &M_FOUR),
                                                  &mut (&M_EIGHT / &FOUR),
                                                  &mut (&SIXTEEN / &M_EIGHT),
                                                  &mut (&M_SIXTEEN / &EIGHT) ];

        let l1_mthrees: [&mut Mod_e382_105; 6] = [ &mut (&THREE / &M_ONE),
                                                   &mut (&M_THREE / &ONE),
                                                   &mut (&SIX / &M_TWO),
                                                   &mut (&M_SIX / &TWO),
                                                   &mut (&NINE / &M_THREE),
                                                   &mut (&M_NINE / &THREE) ];

        let l1_mfours: [&mut Mod_e382_105; 6] = [ &mut (&FOUR / &M_ONE),
                                                 &mut (&M_FOUR / &ONE),
                                                 &mut (&EIGHT / &M_TWO),
                                                 &mut (&M_EIGHT / &TWO),
                                                 &mut (&SIXTEEN / &M_FOUR),
                                                 &mut (&M_SIXTEEN / &FOUR) ];

        for i in 0..10 {
            let mut val = l1_zeros[i].legendre();

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..12 {
            let mut val = l1_ones[i].legendre();

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..10 {
            let mut val = l1_twos[i].legendre();

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..6 {
            let mut val = l1_threes[i].legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..6 {
            let mut val = l1_fours[i].legendre();

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..12 {
            let mut val = l1_mones[i].legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..10 {
            let mut val = l1_mtwos[i].legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..6 {
            let mut val = l1_mthrees[i].legendre();

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..6 {
            let mut val = l1_mfours[i].legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }
    }

    #[test]
    fn test_sqrt() {
        let l1_twos: [&mut Mod_e382_105; 10] = [ &mut (&TWO / &ONE),
                                                 &mut (&M_TWO / &M_ONE),
                                                 &mut (&FOUR / &TWO),
                                                 &mut (&M_FOUR / &M_TWO),
                                                 &mut (&SIX / &THREE),
                                                 &mut (&M_SIX / &M_THREE),
                                                 &mut (&EIGHT / &FOUR),
                                                 &mut (&M_EIGHT / &M_FOUR),
                                                 &mut (&SIXTEEN / &EIGHT),
                                                 &mut (&M_SIXTEEN / &M_EIGHT) ];


        let l1_fours: [&mut Mod_e382_105; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mthrees: [&mut Mod_e382_105; 6] = [ &mut (&THREE / &M_ONE),
                                                   &mut (&M_THREE / &ONE),
                                                   &mut (&SIX / &M_TWO),
                                                   &mut (&M_SIX / &TWO),
                                                   &mut (&NINE / &M_THREE),
                                                   &mut (&M_NINE / &THREE) ];

        for i in 0..10 {
            let val = l1_twos[i].sqrt();

            assert!(val.squared().normalize_eq(l1_twos[i]));
        }

        for i in 0..6 {
            let val = l1_fours[i].sqrt();

            assert!(val.squared().normalize_eq(l1_fours[i]));
        }

        for i in 0..6 {
            let val = l1_mthrees[i].sqrt();

            assert!(val.squared().normalize_eq(l1_mthrees[i]));
        }
    }

    #[test]
    fn test_small_add() {
        let l1_zeros: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_add(0),
                                                 &mut M_ONE.small_add(1),
                                                 &mut ONE.small_add(-1),
                                                 &mut M_TWO.small_add(2),
                                                 &mut TWO.small_add(-2) ];

        let l1_ones: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_add(1),
                                                &mut M_ONE.small_add(2),
                                                &mut ONE.small_add(0),
                                                &mut M_TWO.small_add(3),
                                                &mut TWO.small_add(-1) ];

        let l1_twos: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_add(2),
                                                &mut ONE.small_add(1),
                                                &mut M_ONE.small_add(3),
                                                &mut TWO.small_add(0),
                                                &mut M_TWO.small_add(4) ];

        let l1_mones: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_add(-1),
                                                 &mut M_ONE.small_add(0),
                                                 &mut ONE.small_add(-2),
                                                 &mut M_TWO.small_add(1),
                                                 &mut TWO.small_add(-3) ];

        let l1_mtwos: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_add(-2),
                                                 &mut M_ONE.small_add(-1),
                                                 &mut ONE.small_add(-3),
                                                 &mut M_TWO.small_add(0),
                                                 &mut TWO.small_add(-4) ];

        for i in 0..5 {
            assert!(ZERO.normalize_eq(l1_zeros[i]));
        }

        for i in 0..5 {
            assert!(ONE.normalize_eq(l1_ones[i]));
        }

        for i in 0..5 {
            assert!(TWO.normalize_eq(l1_twos[i]));
        }

        for i in 0..5 {
            assert!(M_ONE.normalize_eq(l1_mones[i]));
        }

        for i in 0..5 {
            assert!(M_TWO.normalize_eq(l1_mtwos[i]));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_add(0);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_add(-1);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_add(-2);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_add(1);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_add(2);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_add(1);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_add(0);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_add(-1);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_add(2);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_add(3);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_add(2);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_add(1);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_add(0);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_add(3);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_add(4);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_add(-1);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_add(-2);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_add(-3);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_add(0);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_add(1);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_add(-2);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_add(-3);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_add(-4);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_add(-1);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_add(0);

            assert!(M_TWO.normalize_eq(&mut val));
        }
    }

    #[test]
    fn test_small_sub() {
        let l1_zeros: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_sub(0),
                                                 &mut M_ONE.small_sub(-1),
                                                 &mut ONE.small_sub(1),
                                                 &mut M_TWO.small_sub(-2),
                                                 &mut TWO.small_sub(2) ];

        let l1_ones: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_sub(-1),
                                                &mut M_ONE.small_sub(-2),
                                                &mut ONE.small_sub(0),
                                                &mut M_TWO.small_sub(-3),
                                                &mut TWO.small_sub(1) ];

        let l1_twos: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_sub(-2),
                                                &mut ONE.small_sub(-1),
                                                &mut M_ONE.small_sub(-3),
                                                &mut TWO.small_sub(0),
                                                &mut M_TWO.small_sub(-4) ];

        let l1_mones: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_sub(1),
                                                 &mut M_ONE.small_sub(0),
                                                 &mut ONE.small_sub(2),
                                                 &mut M_TWO.small_sub(-1),
                                                 &mut TWO.small_sub(3) ];

        let l1_mtwos: [&mut Mod_e382_105; 5] = [ &mut ZERO.small_sub(2),
                                                 &mut M_ONE.small_sub(1),
                                                 &mut ONE.small_sub(3),
                                                 &mut M_TWO.small_sub(0),
                                                 &mut TWO.small_sub(4) ];

        for i in 0..5 {
            assert!(ZERO.normalize_eq(l1_zeros[i]));
        }

        for i in 0..5 {
            assert!(ONE.normalize_eq(l1_ones[i]));
        }

        for i in 0..5 {
            assert!(TWO.normalize_eq(l1_twos[i]));
        }

        for i in 0..5 {
            assert!(M_ONE.normalize_eq(l1_mones[i]));
        }

        for i in 0..5 {
            assert!(M_TWO.normalize_eq(l1_mtwos[i]));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_sub(0);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_sub(1);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_sub(2);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_sub(-1);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_sub(-2);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_sub(-1);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_sub(0);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_sub(1);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_sub(-2);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_sub(-3);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_sub(-2);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_sub(-1);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_sub(0);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_sub(-3);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_sub(-4);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_sub(1);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_sub(2);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_sub(3);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_sub(0);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_sub(-1);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_zeros[i].small_sub(2);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_ones[i].small_sub(3);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_twos[i].small_sub(4);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mones[i].small_sub(1);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..5 {
            let mut val = l1_mtwos[i].small_sub(0);

            assert!(M_TWO.normalize_eq(&mut val));
        }
    }

    #[test]
    fn test_small_mul() {

        let l1_zeros: [&mut Mod_e382_105; 9] = [ &mut ZERO.small_mul(0),
                                                 &mut ONE.small_mul(0),
                                                 &mut TWO.small_mul(0),
                                                 &mut M_ONE.small_mul(0),
                                                 &mut M_TWO.small_mul(0),
                                                 &mut ZERO.small_mul(1),
                                                 &mut ZERO.small_mul(2),
                                                 &mut ZERO.small_mul(-1),
                                                 &mut ZERO.small_mul(-2) ];

        let l1_ones: [&mut Mod_e382_105; 2] = [ &mut ONE.small_mul(1),
                                                &mut M_ONE.small_mul(-1) ];

        let l1_twos: [&mut Mod_e382_105; 4] = [ &mut ONE.small_mul(2),
                                                &mut TWO.small_mul(1),
                                                &mut M_ONE.small_mul(-2),
                                                &mut M_TWO.small_mul(-1) ];

        let l1_fours: [&mut Mod_e382_105; 2] = [ &mut TWO.small_mul(2),
                                                 &mut M_TWO.small_mul(-2) ];

        let l1_mones: [&mut Mod_e382_105; 2] = [ &mut ONE.small_mul(-1),
                                                 &mut M_ONE.small_mul(1) ];

        let l1_mtwos: [&mut Mod_e382_105; 4] = [ &mut ONE.small_mul(-2),
                                                 &mut TWO.small_mul(-1),
                                                 &mut M_ONE.small_mul(2),
                                                 &mut M_TWO.small_mul(1) ];

        let l1_mfours: [&mut Mod_e382_105; 2] = [ &mut TWO.small_mul(-2),
                                                  &mut M_TWO.small_mul(2) ];

        for i in 0..9 {
            assert!(ZERO.normalize_eq(l1_zeros[i]));
        }

        for i in 0..2 {
            assert!(ONE.normalize_eq(l1_ones[i]));
        }

        for i in 0..4 {
            assert!(TWO.normalize_eq(l1_twos[i]));
        }

        for i in 0..2 {
            assert!(FOUR.normalize_eq(l1_fours[i]));
        }

        for i in 0..2 {
            assert!(M_ONE.normalize_eq(l1_mones[i]));
        }

        for i in 0..4 {
            assert!(M_TWO.normalize_eq(l1_mtwos[i]));
        }

        for i in 0..2 {
            assert!(M_FOUR.normalize_eq(l1_mfours[i]));
        }

        for i in 0..9 {
            let mut val = l1_zeros[i].small_mul(0);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_ones[i].small_mul(0);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_twos[i].small_mul(0);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_mones[i].small_mul(0);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_mtwos[i].small_mul(0);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..9 {
            let mut val = l1_zeros[i].small_mul(1);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..9 {
            let mut val = l1_zeros[i].small_mul(2);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..9 {
            let mut val = l1_zeros[i].small_mul(-1);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..9 {
            let mut val = l1_zeros[i].small_mul(-2);

            assert!(ZERO.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_ones[i].small_mul(1);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_mones[i].small_mul(-1);

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_ones[i].small_mul(2);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_twos[i].small_mul(1);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_mones[i].small_mul(-2);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_mtwos[i].small_mul(-1);

            assert!(TWO.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_twos[i].small_mul(2);

            assert!(FOUR.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_mtwos[i].small_mul(-2);

            assert!(FOUR.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_ones[i].small_mul(-1);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_mones[i].small_mul(1);

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_ones[i].small_mul(-2);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_twos[i].small_mul(-1);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_mones[i].small_mul(2);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..4 {
            let mut val = l1_mtwos[i].small_mul(1);

            assert!(M_TWO.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_twos[i].small_mul(-2);

            assert!(M_FOUR.normalize_eq(&mut val));
        }

        for i in 0..2 {
            let mut val = l1_mtwos[i].small_mul(2);

            assert!(M_FOUR.normalize_eq(&mut val));
        }
    }
}
