use field::prime_field::PrimeField;
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
use std::ops::SubAssign;
use std::ops::Sub;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Neg;

/// Elements of the finite field mod 2^521 - 1.  Used by the E-521
/// curve.
///
/// This is represented using twenty 27-bit digits, stored in a
/// ten-element i64 array with two digits per word.  This combined
/// representation allows many operations to be faster.  The leftover
/// bits in each digit are used to capture carry values.  The internal
/// representation is lazily normalized: it may leave carry values in
/// the highest-order digit, and it may hold a value greater than the
/// modulus.  All operations are guaranteed to work on non-normal values
/// of this kind.

#[derive(Copy, Clone)]
pub struct Mod_e521_1([i64; 10]);

/// The normalized representation of the value 0.
pub const ZERO: Mod_e521_1 = Mod_e521_1([ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

/// The normalized representation of the value 1.
pub const ONE: Mod_e521_1 = Mod_e521_1([ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

const C_VAL: i64 = 1;

/// The normalized representation of the value -1.
pub const M_ONE: Mod_e521_1 =
    Mod_e521_1([ 0x003ffffffffffffe, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x00000007ffffffff ]);

/// The normalized representation of the modulus 2^521 - 1.
pub const MODULUS: Mod_e521_1 =
    Mod_e521_1([ 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x00000007ffffffff ]);

pub const E521_D: Mod_e521_1 =
    Mod_e521_1([ 0x003ffffffffa4331, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x00000007ffffffff ]);

pub const E521_BASE_X: Mod_e521_1 =
    Mod_e521_1([ 0x002a940a2f19ba6c, 0x002c4cd920e2a8c0,
                 0x0028fc99c6059d0f, 0x00090d2c6ba52755,
                 0x003913f6ecc5ccc7, 0x00322fcf27031880,
                 0x003d9f42fc6c818e, 0x002c8a5aca1e28ef,
                 0x0008648b189df90c, 0x00000001d4b2d171 ]);

pub const E521_BASE_Y: Mod_e521_1 =
    Mod_e521_1([ 0xc, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

pub const E521_ORDER: Mod_e521_1 =
    Mod_e521_1([ 0x002a2435f5180d6b, 0x00115a6a3c7d1503,
                 0x0027ec53f04fbd8c, 0x003f217dcdae2bd7,
                 0x003ffd15b6c64746, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x003fffffffffffff,
                 0x003fffffffffffff, 0x00000001ffffffff ]);

impl Debug for Mod_e521_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "Mod_e521_1: [ {:x}", &self[0]));

        for i in 1..10 {
            try!(write!(f, ", {:x}", &self[i]));
        }

        write!(f, " ]")
    }
}

impl LowerHex for Mod_e521_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.packed();

        for i in 0..66 {
            try!(write!(f, "{:02x}", bytes[65 - i]));
        }

        Ok(())
    }
}

impl UpperHex for Mod_e521_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.packed();

        for i in 0..66 {
            try!(write!(f, "{:02X}", bytes[65 - i]));
        }

        Ok(())
    }
}

impl Mod_e521_1 {
    /// Get the carry-in value.  We use the highest carry slot to
    /// stash the carry-out value of each operation, and feed that
    /// back into the next one.
    fn carry_out(&self) -> i64 {
        self[9] >> 35
    }

    /// Serialize a value as a little-endian byte array.  This has the
    /// effect of normalizing the representation.
    pub fn packed(&mut self) -> [u8; 66] {
        let mut out = [0u8; 66];
        self.pack(&mut out);
        out
    }
}

impl IndexMut<usize> for Mod_e521_1 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut i64 {
        let ret: &'a mut i64 = &mut(self.0[idx]);
        ret
    }
}

impl Index<usize> for Mod_e521_1 {
    type Output = i64;

    fn index<'a>(&'a self, idx: usize) -> &'a i64 {
        let ret: &'a i64 = &(self.0[idx]);
        ret
    }
}

impl Neg for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn neg(self) -> Mod_e521_1 {
        let mut out = self;

        out += &MODULUS;
        out
    }
}

impl<'b> AddAssign<&'b Mod_e521_1> for Mod_e521_1 {
    fn add_assign(&mut self, rhs: &'b Mod_e521_1) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3];
        let a4 = self[4];
        let a5 = self[5];
        let a6 = self[6];
        let a7 = self[7];
        let a8 = self[8];
        let a9 = self[9] & 0x00000007ffffffff;

        let b0 = rhs[0];
        let b1 = rhs[1];
        let b2 = rhs[2];
        let b3 = rhs[3];
        let b4 = rhs[4];
        let b5 = rhs[5];
        let b6 = rhs[6];
        let b7 = rhs[7];
        let b8 = rhs[8];
        let b9 = rhs[9] & 0x00000007ffffffff;

        let cin = self.carry_out() + rhs.carry_out();
        let s0 = a0 + b0 + (cin * C_VAL);
        let c0 = s0 >> 54;
        let s1 = a1 + b1 + c0;
        let c1 = s1 >> 54;
        let s2 = a2 + b2 + c1;
        let c2 = s2 >> 54;
        let s3 = a3 + b3 + c2;
        let c3 = s3 >> 54;
        let s4 = a4 + b4 + c3;
        let c4 = s4 >> 54;
        let s5 = a5 + b5 + c4;
        let c5 = s5 >> 54;
        let s6 = a6 + b6 + c5;
        let c6 = s6 >> 54;
        let s7 = a7 + b7 + c6;
        let c7 = s7 >> 54;
        let s8 = a8 + b8 + c7;
        let c8 = s8 >> 54;
        let s9 = a9 + b9 + c8;

        self[0] = s0 & 0x003fffffffffffff;
        self[1] = s1 & 0x003fffffffffffff;
        self[2] = s2 & 0x003fffffffffffff;
        self[3] = s3 & 0x003fffffffffffff;
        self[4] = s4 & 0x003fffffffffffff;
        self[5] = s5 & 0x003fffffffffffff;
        self[6] = s6 & 0x003fffffffffffff;
        self[7] = s7 & 0x003fffffffffffff;
        self[8] = s8 & 0x003fffffffffffff;
        self[9] = s9;
    }
}

impl AddAssign<Mod_e521_1> for Mod_e521_1 {
    fn add_assign(&mut self, rhs: Mod_e521_1) {
        *self += &rhs;
    }
}

impl AddAssign<i32> for Mod_e521_1 {
    fn add_assign(&mut self, rhs: i32) {
        self.small_add_assign(rhs);
    }
}

impl AddAssign<i16> for Mod_e521_1 {
    fn add_assign(&mut self, rhs: i16) {
        self.small_add_assign(rhs as i32);
    }
}

impl AddAssign<i8> for Mod_e521_1 {
    fn add_assign(&mut self, rhs: i8) {
        self.small_add_assign(rhs as i32);
    }
}

impl<'a, 'b> Add<&'b Mod_e521_1> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: &'b Mod_e521_1) -> Mod_e521_1 {
        let mut out = self.clone();
        out += a;
        out
    }
}

impl Add<Mod_e521_1> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: Mod_e521_1) -> Mod_e521_1 {
        &self + &a
    }
}

impl<'a> Add<&'a Mod_e521_1> for i32 {
    type Output = Mod_e521_1;

    fn add(self, a: &'a Mod_e521_1) -> Mod_e521_1 {
        a.small_add(self)
    }
}

impl<'a> Add<&'a Mod_e521_1> for i16 {
    type Output = Mod_e521_1;

    fn add(self, a: &'a Mod_e521_1) -> Mod_e521_1 {
        a.small_add(self as i32)
    }
}

impl<'a> Add<&'a Mod_e521_1> for i8 {
    type Output = Mod_e521_1;

    fn add(self, a: &'a Mod_e521_1) -> Mod_e521_1 {
        a.small_add(self as i32)
    }
}

impl<'a> Add<i32> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: i32) -> Mod_e521_1 {
        self.small_add(a)
    }
}

impl<'a> Add<i16> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: i16) -> Mod_e521_1 {
        self.small_add(a as i32)
    }
}

impl<'a> Add<i8> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: i8) -> Mod_e521_1 {
        self.small_add(a as i32)
    }
}

impl Add<i32> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: i32) -> Mod_e521_1 {
        &self + a
    }
}

impl Add<i16> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: i16) -> Mod_e521_1 {
        &self + a
    }
}

impl Add<i8> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn add(self, a: i8) -> Mod_e521_1 {
        &self + a
    }
}

impl<'b> DivAssign<&'b Mod_e521_1> for Mod_e521_1 {
    fn div_assign(&mut self, rhs: &'b Mod_e521_1) {
        *self *= &rhs.inverted();
    }
}

impl DivAssign<Mod_e521_1> for Mod_e521_1 {
    fn div_assign(&mut self, rhs: Mod_e521_1) {
        *self /= &rhs;
    }
}

impl<'a, 'b> Div<&'b Mod_e521_1> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn div(self, a: &'b Mod_e521_1) -> Mod_e521_1 {
        let mut out = self.clone();
        out /= a;
        out
    }
}

impl Div<Mod_e521_1> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn div(self, a: Mod_e521_1) -> Mod_e521_1 {
        &self / &a
    }
}

impl<'b> SubAssign<&'b Mod_e521_1> for Mod_e521_1 {
    fn sub_assign(&mut self, rhs: &'b Mod_e521_1) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3];
        let a4 = self[4];
        let a5 = self[5];
        let a6 = self[6];
        let a7 = self[7];
        let a8 = self[8];
        let a9 = self[9] & 0x00000007ffffffff;

        let b0 = rhs[0];
        let b1 = rhs[1];
        let b2 = rhs[2];
        let b3 = rhs[3];
        let b4 = rhs[4];
        let b5 = rhs[5];
        let b6 = rhs[6];
        let b7 = rhs[7];
        let b8 = rhs[8];
        let b9 = rhs[9] & 0x00000007ffffffff;

        let cin = self.carry_out() + rhs.carry_out();
        let s0 = a0 - b0 + (cin * C_VAL);
        let c0 = s0 >> 54;
        let s1 = a1 - b1 + c0;
        let c1 = s1 >> 54;
        let s2 = a2 - b2 + c1;
        let c2 = s2 >> 54;
        let s3 = a3 - b3 + c2;
        let c3 = s3 >> 54;
        let s4 = a4 - b4 + c3;
        let c4 = s4 >> 54;
        let s5 = a5 - b5 + c4;
        let c5 = s5 >> 54;
        let s6 = a6 - b6 + c5;
        let c6 = s6 >> 54;
        let s7 = a7 - b7 + c6;
        let c7 = s7 >> 54;
        let s8 = a8 - b8 + c7;
        let c8 = s8 >> 54;
        let s9 = a9 - b9 + c8;

        self[0] = s0 & 0x003fffffffffffff;
        self[1] = s1 & 0x003fffffffffffff;
        self[2] = s2 & 0x003fffffffffffff;
        self[3] = s3 & 0x003fffffffffffff;
        self[4] = s4 & 0x003fffffffffffff;
        self[5] = s5 & 0x003fffffffffffff;
        self[6] = s6 & 0x003fffffffffffff;
        self[7] = s7 & 0x003fffffffffffff;
        self[8] = s8 & 0x003fffffffffffff;
        self[9] = s9;
    }
}

impl SubAssign<Mod_e521_1> for Mod_e521_1 {
    fn sub_assign(&mut self, rhs: Mod_e521_1) {
        *self -= &rhs
    }
}

impl SubAssign<i32> for Mod_e521_1 {
    fn sub_assign(&mut self, rhs: i32) {
        self.small_sub_assign(rhs);
    }
}

impl SubAssign<i16> for Mod_e521_1 {
    fn sub_assign(&mut self, rhs: i16) {
        self.small_sub_assign(rhs as i32);
    }
}

impl SubAssign<i8> for Mod_e521_1 {
    fn sub_assign(&mut self, rhs: i8) {
        self.small_sub_assign(rhs as i32);
    }
}

impl Sub<i32> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: i32) -> Mod_e521_1 {
        &self - a
    }
}

impl Sub<i16> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: i16) -> Mod_e521_1 {
        &self - a
    }
}

impl Sub<i8> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: i8) -> Mod_e521_1 {
        &self - a
    }
}

impl<'a, 'b> Sub<&'b Mod_e521_1> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: &'b Mod_e521_1) -> Mod_e521_1 {
        let mut out = self.clone();
        out -= a;
        out
    }
}

impl Sub<Mod_e521_1> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: Mod_e521_1) -> Mod_e521_1 {
        &self - &a
    }
}

impl<'a> Sub<i32> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: i32) -> Mod_e521_1 {
        self.small_sub(a)
    }
}

impl<'a> Sub<i16> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: i16) -> Mod_e521_1 {
        self.small_sub(a as i32)
    }
}

impl<'a> Sub<i8> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn sub(self, a: i8) -> Mod_e521_1 {
        self.small_sub(a as i32)
    }
}

impl MulAssign<i32> for Mod_e521_1 {
    fn mul_assign(&mut self, rhs: i32) {
        self.small_mul_assign(rhs);
    }
}

impl MulAssign<i16> for Mod_e521_1 {
    fn mul_assign(&mut self, rhs: i16) {
        self.small_mul_assign(rhs as i32);
    }
}

impl MulAssign<i8> for Mod_e521_1 {
    fn mul_assign(&mut self, rhs: i8) {
        self.small_mul_assign(rhs as i32);
    }
}

impl<'b> MulAssign<&'b Mod_e521_1> for Mod_e521_1 {
    fn mul_assign(&mut self, rhs: &'b Mod_e521_1) {
        let a0 = self[0] & 0x07ffffff;
        let a1 = self[0] >> 27;
        let a2 = self[1] & 0x07ffffff;
        let a3 = self[1] >> 27;
        let a4 = self[2] & 0x07ffffff;
        let a5 = self[2] >> 27;
        let a6 = self[3] & 0x07ffffff;
        let a7 = self[3] >> 27;
        let a8 = self[4] & 0x07ffffff;
        let a9 = self[4] >> 27;
        let a10 = self[5] & 0x07ffffff;
        let a11 = self[5] >> 27;
        let a12 = self[6] & 0x07ffffff;
        let a13 = self[6] >> 27;
        let a14 = self[7] & 0x07ffffff;
        let a15 = self[7] >> 27;
        let a16 = self[8] & 0x07ffffff;
        let a17 = self[8] >> 27;
        let a18 = self[9] & 0x07ffffff;
        let a19 = self[9] >> 27;

        let b0 = rhs[0] & 0x07ffffff;
        let b1 = rhs[0] >> 27;
        let b2 = rhs[1] & 0x07ffffff;
        let b3 = rhs[1] >> 27;
        let b4 = rhs[2] & 0x07ffffff;
        let b5 = rhs[2] >> 27;
        let b6 = rhs[3] & 0x07ffffff;
        let b7 = rhs[3] >> 27;
        let b8 = rhs[4] & 0x07ffffff;
        let b9 = rhs[4] >> 27;
        let b10 = rhs[5] & 0x07ffffff;
        let b11 = rhs[5] >> 27;
        let b12 = rhs[6] & 0x07ffffff;
        let b13 = rhs[6] >> 27;
        let b14 = rhs[7] & 0x07ffffff;
        let b15 = rhs[7] >> 27;
        let b16 = rhs[8] & 0x07ffffff;
        let b17 = rhs[8] >> 27;
        let b18 = rhs[9] & 0x07ffffff;
        let b19 = rhs[9] >> 27;

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
        let m_0_14 = a0 * b14;
        let m_0_15 = a0 * b15;
        let m_0_16 = a0 * b16;
        let m_0_17 = a0 * b17;
        let m_0_18 = a0 * b18;
        let m_0_19 = a0 * b19;
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
        let m_1_14 = a1 * b14;
        let m_1_15 = a1 * b15;
        let m_1_16 = a1 * b16;
        let m_1_17 = a1 * b17;
        let m_1_18 = a1 * b18;
        let m_1_19 = a1 * b19;
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
        let m_2_14 = a2 * b14;
        let m_2_15 = a2 * b15;
        let m_2_16 = a2 * b16;
        let m_2_17 = a2 * b17;
        let m_2_18 = a2 * b18;
        let m_2_19 = a2 * b19;
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
        let m_3_14 = a3 * b14;
        let m_3_15 = a3 * b15;
        let m_3_16 = a3 * b16;
        let m_3_17 = a3 * b17;
        let m_3_18 = a3 * b18;
        let m_3_19 = a3 * b19;
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
        let m_4_14 = a4 * b14;
        let m_4_15 = a4 * b15;
        let m_4_16 = a4 * b16;
        let m_4_17 = a4 * b17;
        let m_4_18 = a4 * b18;
        let m_4_19 = a4 * b19;
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
        let m_5_14 = a5 * b14;
        let m_5_15 = a5 * b15;
        let m_5_16 = a5 * b16;
        let m_5_17 = a5 * b17;
        let m_5_18 = a5 * b18;
        let m_5_19 = a5 * b19;
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
        let m_6_14 = a6 * b14;
        let m_6_15 = a6 * b15;
        let m_6_16 = a6 * b16;
        let m_6_17 = a6 * b17;
        let m_6_18 = a6 * b18;
        let m_6_19 = a6 * b19;
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
        let m_7_14 = a7 * b14;
        let m_7_15 = a7 * b15;
        let m_7_16 = a7 * b16;
        let m_7_17 = a7 * b17;
        let m_7_18 = a7 * b18;
        let m_7_19 = a7 * b19;
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
        let m_8_14 = a8 * b14;
        let m_8_15 = a8 * b15;
        let m_8_16 = a8 * b16;
        let m_8_17 = a8 * b17;
        let m_8_18 = a8 * b18;
        let m_8_19 = a8 * b19;
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
        let m_9_14 = a9 * b14;
        let m_9_15 = a9 * b15;
        let m_9_16 = a9 * b16;
        let m_9_17 = a9 * b17;
        let m_9_18 = a9 * b18;
        let m_9_19 = a9 * b19;
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
        let m_10_14 = a10 * b14;
        let m_10_15 = a10 * b15;
        let m_10_16 = a10 * b16;
        let m_10_17 = a10 * b17;
        let m_10_18 = a10 * b18;
        let m_10_19 = a10 * b19;
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
        let m_11_14 = a11 * b14;
        let m_11_15 = a11 * b15;
        let m_11_16 = a11 * b16;
        let m_11_17 = a11 * b17;
        let m_11_18 = a11 * b18;
        let m_11_19 = a11 * b19;
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
        let m_12_14 = a12 * b14;
        let m_12_15 = a12 * b15;
        let m_12_16 = a12 * b16;
        let m_12_17 = a12 * b17;
        let m_12_18 = a12 * b18;
        let m_12_19 = a12 * b19;
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
        let m_13_14 = a13 * b14;
        let m_13_15 = a13 * b15;
        let m_13_16 = a13 * b16;
        let m_13_17 = a13 * b17;
        let m_13_18 = a13 * b18;
        let m_13_19 = a13 * b19;
        let m_14_0 = a14 * b0;
        let m_14_1 = a14 * b1;
        let m_14_2 = a14 * b2;
        let m_14_3 = a14 * b3;
        let m_14_4 = a14 * b4;
        let m_14_5 = a14 * b5;
        let m_14_6 = a14 * b6;
        let m_14_7 = a14 * b7;
        let m_14_8 = a14 * b8;
        let m_14_9 = a14 * b9;
        let m_14_10 = a14 * b10;
        let m_14_11 = a14 * b11;
        let m_14_12 = a14 * b12;
        let m_14_13 = a14 * b13;
        let m_14_14 = a14 * b14;
        let m_14_15 = a14 * b15;
        let m_14_16 = a14 * b16;
        let m_14_17 = a14 * b17;
        let m_14_18 = a14 * b18;
        let m_14_19 = a14 * b19;
        let m_15_0 = a15 * b0;
        let m_15_1 = a15 * b1;
        let m_15_2 = a15 * b2;
        let m_15_3 = a15 * b3;
        let m_15_4 = a15 * b4;
        let m_15_5 = a15 * b5;
        let m_15_6 = a15 * b6;
        let m_15_7 = a15 * b7;
        let m_15_8 = a15 * b8;
        let m_15_9 = a15 * b9;
        let m_15_10 = a15 * b10;
        let m_15_11 = a15 * b11;
        let m_15_12 = a15 * b12;
        let m_15_13 = a15 * b13;
        let m_15_14 = a15 * b14;
        let m_15_15 = a15 * b15;
        let m_15_16 = a15 * b16;
        let m_15_17 = a15 * b17;
        let m_15_18 = a15 * b18;
        let m_15_19 = a15 * b19;
        let m_16_0 = a16 * b0;
        let m_16_1 = a16 * b1;
        let m_16_2 = a16 * b2;
        let m_16_3 = a16 * b3;
        let m_16_4 = a16 * b4;
        let m_16_5 = a16 * b5;
        let m_16_6 = a16 * b6;
        let m_16_7 = a16 * b7;
        let m_16_8 = a16 * b8;
        let m_16_9 = a16 * b9;
        let m_16_10 = a16 * b10;
        let m_16_11 = a16 * b11;
        let m_16_12 = a16 * b12;
        let m_16_13 = a16 * b13;
        let m_16_14 = a16 * b14;
        let m_16_15 = a16 * b15;
        let m_16_16 = a16 * b16;
        let m_16_17 = a16 * b17;
        let m_16_18 = a16 * b18;
        let m_16_19 = a16 * b19;
        let m_17_0 = a17 * b0;
        let m_17_1 = a17 * b1;
        let m_17_2 = a17 * b2;
        let m_17_3 = a17 * b3;
        let m_17_4 = a17 * b4;
        let m_17_5 = a17 * b5;
        let m_17_6 = a17 * b6;
        let m_17_7 = a17 * b7;
        let m_17_8 = a17 * b8;
        let m_17_9 = a17 * b9;
        let m_17_10 = a17 * b10;
        let m_17_11 = a17 * b11;
        let m_17_12 = a17 * b12;
        let m_17_13 = a17 * b13;
        let m_17_14 = a17 * b14;
        let m_17_15 = a17 * b15;
        let m_17_16 = a17 * b16;
        let m_17_17 = a17 * b17;
        let m_17_18 = a17 * b18;
        let m_17_19 = a17 * b19;
        let m_18_0 = a18 * b0;
        let m_18_1 = a18 * b1;
        let m_18_2 = a18 * b2;
        let m_18_3 = a18 * b3;
        let m_18_4 = a18 * b4;
        let m_18_5 = a18 * b5;
        let m_18_6 = a18 * b6;
        let m_18_7 = a18 * b7;
        let m_18_8 = a18 * b8;
        let m_18_9 = a18 * b9;
        let m_18_10 = a18 * b10;
        let m_18_11 = a18 * b11;
        let m_18_12 = a18 * b12;
        let m_18_13 = a18 * b13;
        let m_18_14 = a18 * b14;
        let m_18_15 = a18 * b15;
        let m_18_16 = a18 * b16;
        let m_18_17 = a18 * b17;
        let m_18_18 = a18 * b18;
        let m_18_19 = a18 * b19;
        let m_19_0 = a19 * b0;
        let m_19_1 = a19 * b1;
        let m_19_2 = a19 * b2;
        let m_19_3 = a19 * b3;
        let m_19_4 = a19 * b4;
        let m_19_5 = a19 * b5;
        let m_19_6 = a19 * b6;
        let m_19_7 = a19 * b7;
        let m_19_8 = a19 * b8;
        let m_19_9 = a19 * b9;
        let m_19_10 = a19 * b10;
        let m_19_11 = a19 * b11;
        let m_19_12 = a19 * b12;
        let m_19_13 = a19 * b13;
        let m_19_14 = a19 * b14;
        let m_19_15 = a19 * b15;
        let m_19_16 = a19 * b16;
        let m_19_17 = a19 * b17;
        let m_19_18 = a19 * b18;
        let m_19_19 = a19 * b19;

        // Compute the 40-digit combined product using 64-bit operations.
        let d0 = m_0_0 + ((m_0_1 & 0x07ffffff) << 27) +
                 ((m_1_0 & 0x07ffffff) << 27);
        let c0 = d0 >> 54;
        let d1 = (m_0_1 >> 27) + m_0_2 + ((m_0_3 & 0x07ffffff) << 27) +
                 (m_1_0 >> 27) + m_1_1 + ((m_1_2 & 0x07ffffff) << 27) +
                 m_2_0 + ((m_2_1 & 0x07ffffff) << 27) +
                 ((m_3_0 & 0x07ffffff) << 27) + c0;
        let c1 = d1 >> 54;
        let d2 = (m_0_3 >> 27) + m_0_4 + ((m_0_5 & 0x07ffffff) << 27) +
                 (m_1_2 >> 27) + m_1_3 + ((m_1_4 & 0x07ffffff) << 27) +
                 (m_2_1 >> 27) + m_2_2 + ((m_2_3 & 0x07ffffff) << 27) +
                 (m_3_0 >> 27) + m_3_1 + ((m_3_2 & 0x07ffffff) << 27) +
                 m_4_0 + ((m_4_1 & 0x07ffffff) << 27) +
                 ((m_5_0 & 0x07ffffff) << 27) + c1;
        let c2 = d2 >> 54;
        let d3 = (m_0_5 >> 27) + m_0_6 + ((m_0_7 & 0x07ffffff) << 27) +
                 (m_1_4 >> 27) + m_1_5 + ((m_1_6 & 0x07ffffff) << 27) +
                 (m_2_3 >> 27) + m_2_4 + ((m_2_5 & 0x07ffffff) << 27) +
                 (m_3_2 >> 27) + m_3_3 + ((m_3_4 & 0x07ffffff) << 27) +
                 (m_4_1 >> 27) + m_4_2 + ((m_4_3 & 0x07ffffff) << 27) +
                 (m_5_0 >> 27) + m_5_1 + ((m_5_2 & 0x07ffffff) << 27) +
                 m_6_0 + ((m_6_1 & 0x07ffffff) << 27) +
                 ((m_7_0 & 0x07ffffff) << 27) + c2;
        let c3 = d3 >> 54;
        let d4 = (m_0_7 >> 27) + m_0_8 + ((m_0_9 & 0x07ffffff) << 27) +
                 (m_1_6 >> 27) + m_1_7 + ((m_1_8 & 0x07ffffff) << 27) +
                 (m_2_5 >> 27) + m_2_6 + ((m_2_7 & 0x07ffffff) << 27) +
                 (m_3_4 >> 27) + m_3_5 + ((m_3_6 & 0x07ffffff) << 27) +
                 (m_4_3 >> 27) + m_4_4 + ((m_4_5 & 0x07ffffff) << 27) +
                 (m_5_2 >> 27) + m_5_3 + ((m_5_4 & 0x07ffffff) << 27) +
                 (m_6_1 >> 27) + m_6_2 + ((m_6_3 & 0x07ffffff) << 27) +
                 (m_7_0 >> 27) + m_7_1 + ((m_7_2 & 0x07ffffff) << 27) +
                 m_8_0 + ((m_8_1 & 0x07ffffff) << 27) +
                 ((m_9_0 & 0x07ffffff) << 27) + c3;
        let c4 = d4 >> 54;
        let d5 = (m_0_9 >> 27) + m_0_10 + ((m_0_11 & 0x07ffffff) << 27) +
                 (m_1_8 >> 27) + m_1_9 + ((m_1_10 & 0x07ffffff) << 27) +
                 (m_2_7 >> 27) + m_2_8 + ((m_2_9 & 0x07ffffff) << 27) +
                 (m_3_6 >> 27) + m_3_7 + ((m_3_8 & 0x07ffffff) << 27) +
                 (m_4_5 >> 27) + m_4_6 + ((m_4_7 & 0x07ffffff) << 27) +
                 (m_5_4 >> 27) + m_5_5 + ((m_5_6 & 0x07ffffff) << 27) +
                 (m_6_3 >> 27) + m_6_4 + ((m_6_5 & 0x07ffffff) << 27) +
                 (m_7_2 >> 27) + m_7_3 + ((m_7_4 & 0x07ffffff) << 27) +
                 (m_8_1 >> 27) + m_8_2 + ((m_8_3 & 0x07ffffff) << 27) +
                 (m_9_0 >> 27) + m_9_1 + ((m_9_2 & 0x07ffffff) << 27) +
                 m_10_0 + ((m_10_1 & 0x07ffffff) << 27) +
                 ((m_11_0 & 0x07ffffff) << 27) + c4;
        let c5 = d5 >> 54;
        let d6 = (m_0_11 >> 27) + m_0_12 + ((m_0_13 & 0x07ffffff) << 27) +
                 (m_1_10 >> 27) + m_1_11 + ((m_1_12 & 0x07ffffff) << 27) +
                 (m_2_9 >> 27) + m_2_10 + ((m_2_11 & 0x07ffffff) << 27) +
                 (m_3_8 >> 27) + m_3_9 + ((m_3_10 & 0x07ffffff) << 27) +
                 (m_4_7 >> 27) + m_4_8 + ((m_4_9 & 0x07ffffff) << 27) +
                 (m_5_6 >> 27) + m_5_7 + ((m_5_8 & 0x07ffffff) << 27) +
                 (m_6_5 >> 27) + m_6_6 + ((m_6_7 & 0x07ffffff) << 27) +
                 (m_7_4 >> 27) + m_7_5 + ((m_7_6 & 0x07ffffff) << 27) +
                 (m_8_3 >> 27) + m_8_4 + ((m_8_5 & 0x07ffffff) << 27) +
                 (m_9_2 >> 27) + m_9_3 + ((m_9_4 & 0x07ffffff) << 27) +
                 (m_10_1 >> 27) + m_10_2 + ((m_10_3 & 0x07ffffff) << 27) +
                 (m_11_0 >> 27) + m_11_1 + ((m_11_2 & 0x07ffffff) << 27) +
                 m_12_0 + ((m_12_1 & 0x07ffffff) << 27) +
                 ((m_13_0 & 0x07ffffff) << 27) + c5;
        let c6 = d6 >> 54;
        let d7 = (m_0_13 >> 27) + m_0_14 + ((m_0_15 & 0x07ffffff) << 27) +
                 (m_1_12 >> 27) + m_1_13 + ((m_1_14 & 0x07ffffff) << 27) +
                 (m_2_11 >> 27) + m_2_12 + ((m_2_13 & 0x07ffffff) << 27) +
                 (m_3_10 >> 27) + m_3_11 + ((m_3_12 & 0x07ffffff) << 27) +
                 (m_4_9 >> 27) + m_4_10 + ((m_4_11 & 0x07ffffff) << 27) +
                 (m_5_8 >> 27) + m_5_9 + ((m_5_10 & 0x07ffffff) << 27) +
                 (m_6_7 >> 27) + m_6_8 + ((m_6_9 & 0x07ffffff) << 27) +
                 (m_7_6 >> 27) + m_7_7 + ((m_7_8 & 0x07ffffff) << 27) +
                 (m_8_5 >> 27) + m_8_6 + ((m_8_7 & 0x07ffffff) << 27) +
                 (m_9_4 >> 27) + m_9_5 + ((m_9_6 & 0x07ffffff) << 27) +
                 (m_10_3 >> 27) + m_10_4 + ((m_10_5 & 0x07ffffff) << 27) +
                 (m_11_2 >> 27) + m_11_3 + ((m_11_4 & 0x07ffffff) << 27) +
                 (m_12_1 >> 27) + m_12_2 + ((m_12_3 & 0x07ffffff) << 27) +
                 (m_13_0 >> 27) + m_13_1 + ((m_13_2 & 0x07ffffff) << 27) +
                 m_14_0 + ((m_14_1 & 0x07ffffff) << 27) +
                 ((m_15_0 & 0x07ffffff) << 27) + c6;
        let c7 = d7 >> 54;
        let d8 = (m_0_15 >> 27) + m_0_16 + ((m_0_17 & 0x07ffffff) << 27) +
                 (m_1_14 >> 27) + m_1_15 + ((m_1_16 & 0x07ffffff) << 27) +
                 (m_2_13 >> 27) + m_2_14 + ((m_2_15 & 0x07ffffff) << 27) +
                 (m_3_12 >> 27) + m_3_13 + ((m_3_14 & 0x07ffffff) << 27) +
                 (m_4_11 >> 27) + m_4_12 + ((m_4_13 & 0x07ffffff) << 27) +
                 (m_5_10 >> 27) + m_5_11 + ((m_5_12 & 0x07ffffff) << 27) +
                 (m_6_9 >> 27) + m_6_10 + ((m_6_11 & 0x07ffffff) << 27) +
                 (m_7_8 >> 27) + m_7_9 + ((m_7_10 & 0x07ffffff) << 27) +
                 (m_8_7 >> 27) + m_8_8 + ((m_8_9 & 0x07ffffff) << 27) +
                 (m_9_6 >> 27) + m_9_7 + ((m_9_8 & 0x07ffffff) << 27) +
                 (m_10_5 >> 27) + m_10_6 + ((m_10_7 & 0x07ffffff) << 27) +
                 (m_11_4 >> 27) + m_11_5 + ((m_11_6 & 0x07ffffff) << 27) +
                 (m_12_3 >> 27) + m_12_4 + ((m_12_5 & 0x07ffffff) << 27) +
                 (m_13_2 >> 27) + m_13_3 + ((m_13_4 & 0x07ffffff) << 27) +
                 (m_14_1 >> 27) + m_14_2 + ((m_14_3 & 0x07ffffff) << 27) +
                 (m_15_0 >> 27) + m_15_1 + ((m_15_2 & 0x07ffffff) << 27) +
                 m_16_0 + ((m_16_1 & 0x07ffffff) << 27) +
                 ((m_17_0 & 0x07ffffff) << 27) + c7;
        let c8 = d8 >> 54;
        let d9 = (m_0_17 >> 27) + m_0_18 + ((m_0_19 & 0x07ffffff) << 27) +
                 (m_1_16 >> 27) + m_1_17 + ((m_1_18 & 0x07ffffff) << 27) +
                 (m_2_15 >> 27) + m_2_16 + ((m_2_17 & 0x07ffffff) << 27) +
                 (m_3_14 >> 27) + m_3_15 + ((m_3_16 & 0x07ffffff) << 27) +
                 (m_4_13 >> 27) + m_4_14 + ((m_4_15 & 0x07ffffff) << 27) +
                 (m_5_12 >> 27) + m_5_13 + ((m_5_14 & 0x07ffffff) << 27) +
                 (m_6_11 >> 27) + m_6_12 + ((m_6_13 & 0x07ffffff) << 27) +
                 (m_7_10 >> 27) + m_7_11 + ((m_7_12 & 0x07ffffff) << 27) +
                 (m_8_9 >> 27) + m_8_10 + ((m_8_11 & 0x07ffffff) << 27) +
                 (m_9_8 >> 27) + m_9_9 + ((m_9_10 & 0x07ffffff) << 27) +
                 (m_10_7 >> 27) + m_10_8 + ((m_10_9 & 0x07ffffff) << 27) +
                 (m_11_6 >> 27) + m_11_7 + ((m_11_8 & 0x07ffffff) << 27) +
                 (m_12_5 >> 27) + m_12_6 + ((m_12_7 & 0x07ffffff) << 27) +
                 (m_13_4 >> 27) + m_13_5 + ((m_13_6 & 0x07ffffff) << 27) +
                 (m_14_3 >> 27) + m_14_4 + ((m_14_5 & 0x07ffffff) << 27) +
                 (m_15_2 >> 27) + m_15_3 + ((m_15_4 & 0x07ffffff) << 27) +
                 (m_16_1 >> 27) + m_16_2 + ((m_16_3 & 0x07ffffff) << 27) +
                 (m_17_0 >> 27) + m_17_1 + ((m_17_2 & 0x07ffffff) << 27) +
                 m_18_0 + ((m_18_1 & 0x07ffffff) << 27) +
                 ((m_19_0 & 0x07ffffff) << 27) + c8;
        let c9 = d9 >> 54;
        let d10 = (m_0_19 >> 27) +
                  (m_1_18 >> 27) + m_1_19 +
                  (m_2_17 >> 27) + m_2_18 + ((m_2_19 & 0x07ffffff) << 27) +
                  (m_3_16 >> 27) + m_3_17 + ((m_3_18 & 0x07ffffff) << 27) +
                  (m_4_15 >> 27) + m_4_16 + ((m_4_17 & 0x07ffffff) << 27) +
                  (m_5_14 >> 27) + m_5_15 + ((m_5_16 & 0x07ffffff) << 27) +
                  (m_6_13 >> 27) + m_6_14 + ((m_6_15 & 0x07ffffff) << 27) +
                  (m_7_12 >> 27) + m_7_13 + ((m_7_14 & 0x07ffffff) << 27) +
                  (m_8_11 >> 27) + m_8_12 + ((m_8_13 & 0x07ffffff) << 27) +
                  (m_9_10 >> 27) + m_9_11 + ((m_9_12 & 0x07ffffff) << 27) +
                  (m_10_9 >> 27) + m_10_10 + ((m_10_11 & 0x07ffffff) << 27) +
                  (m_11_8 >> 27) + m_11_9 + ((m_11_10 & 0x07ffffff) << 27) +
                  (m_12_7 >> 27) + m_12_8 + ((m_12_9 & 0x07ffffff) << 27) +
                  (m_13_6 >> 27) + m_13_7 + ((m_13_8 & 0x07ffffff) << 27) +
                  (m_14_5 >> 27) + m_14_6 + ((m_14_7 & 0x07ffffff) << 27) +
                  (m_15_4 >> 27) + m_15_5 + ((m_15_6 & 0x07ffffff) << 27) +
                  (m_16_3 >> 27) + m_16_4 + ((m_16_5 & 0x07ffffff) << 27) +
                  (m_17_2 >> 27) + m_17_3 + ((m_17_4 & 0x07ffffff) << 27) +
                  (m_18_1 >> 27) + m_18_2 + ((m_18_3 & 0x07ffffff) << 27) +
                  (m_19_0 >> 27) + m_19_1 + ((m_19_2 & 0x07ffffff) << 27) + c9;
        let c10 = d10 >> 54;
        let d11 = (m_2_19 >> 27) +
                  (m_3_18 >> 27) + m_3_19 +
                  (m_4_17 >> 27) + m_4_18 + ((m_4_19 & 0x07ffffff) << 27) +
                  (m_5_16 >> 27) + m_5_17 + ((m_5_18 & 0x07ffffff) << 27) +
                  (m_6_15 >> 27) + m_6_16 + ((m_6_17 & 0x07ffffff) << 27) +
                  (m_7_14 >> 27) + m_7_15 + ((m_7_16 & 0x07ffffff) << 27) +
                  (m_8_13 >> 27) + m_8_14 + ((m_8_15 & 0x07ffffff) << 27) +
                  (m_9_12 >> 27) + m_9_13 + ((m_9_14 & 0x07ffffff) << 27) +
                  (m_10_11 >> 27) + m_10_12 + ((m_10_13 & 0x07ffffff) << 27) +
                  (m_11_10 >> 27) + m_11_11 + ((m_11_12 & 0x07ffffff) << 27) +
                  (m_12_9 >> 27) + m_12_10 + ((m_12_11 & 0x07ffffff) << 27) +
                  (m_13_8 >> 27) + m_13_9 + ((m_13_10 & 0x07ffffff) << 27) +
                  (m_14_7 >> 27) + m_14_8 + ((m_14_9 & 0x07ffffff) << 27) +
                  (m_15_6 >> 27) + m_15_7 + ((m_15_8 & 0x07ffffff) << 27) +
                  (m_16_5 >> 27) + m_16_6 + ((m_16_7 & 0x07ffffff) << 27) +
                  (m_17_4 >> 27) + m_17_5 + ((m_17_6 & 0x07ffffff) << 27) +
                  (m_18_3 >> 27) + m_18_4 + ((m_18_5 & 0x07ffffff) << 27) +
                  (m_19_2 >> 27) + m_19_3 + ((m_19_4 & 0x07ffffff) << 27) + c10;
        let c11 = d11 >> 54;
        let d12 = (m_4_19 >> 27) +
                  (m_5_18 >> 27) + m_5_19 +
                  (m_6_17 >> 27) + m_6_18 + ((m_6_19 & 0x07ffffff) << 27) +
                  (m_7_16 >> 27) + m_7_17 + ((m_7_18 & 0x07ffffff) << 27) +
                  (m_8_15 >> 27) + m_8_16 + ((m_8_17 & 0x07ffffff) << 27) +
                  (m_9_14 >> 27) + m_9_15 + ((m_9_16 & 0x07ffffff) << 27) +
                  (m_10_13 >> 27) + m_10_14 + ((m_10_15 & 0x07ffffff) << 27) +
                  (m_11_12 >> 27) + m_11_13 + ((m_11_14 & 0x07ffffff) << 27) +
                  (m_12_11 >> 27) + m_12_12 + ((m_12_13 & 0x07ffffff) << 27) +
                  (m_13_10 >> 27) + m_13_11 + ((m_13_12 & 0x07ffffff) << 27) +
                  (m_14_9 >> 27) + m_14_10 + ((m_14_11 & 0x07ffffff) << 27) +
                  (m_15_8 >> 27) + m_15_9 + ((m_15_10 & 0x07ffffff) << 27) +
                  (m_16_7 >> 27) + m_16_8 + ((m_16_9 & 0x07ffffff) << 27) +
                  (m_17_6 >> 27) + m_17_7 + ((m_17_8 & 0x07ffffff) << 27) +
                  (m_18_5 >> 27) + m_18_6 + ((m_18_7 & 0x07ffffff) << 27) +
                  (m_19_4 >> 27) + m_19_5 + ((m_19_6 & 0x07ffffff) << 27) + c11;
        let c12 = d12 >> 54;
        let d13 = (m_6_19 >> 27) +
                  (m_7_18 >> 27) + m_7_19 +
                  (m_8_17 >> 27) + m_8_18 + ((m_8_19 & 0x07ffffff) << 27) +
                  (m_9_16 >> 27) + m_9_17 + ((m_9_18 & 0x07ffffff) << 27) +
                  (m_10_15 >> 27) + m_10_16 + ((m_10_17 & 0x07ffffff) << 27) +
                  (m_11_14 >> 27) + m_11_15 + ((m_11_16 & 0x07ffffff) << 27) +
                  (m_12_13 >> 27) + m_12_14 + ((m_12_15 & 0x07ffffff) << 27) +
                  (m_13_12 >> 27) + m_13_13 + ((m_13_14 & 0x07ffffff) << 27) +
                  (m_14_11 >> 27) + m_14_12 + ((m_14_13 & 0x07ffffff) << 27) +
                  (m_15_10 >> 27) + m_15_11 + ((m_15_12 & 0x07ffffff) << 27) +
                  (m_16_9 >> 27) + m_16_10 + ((m_16_11 & 0x07ffffff) << 27) +
                  (m_17_8 >> 27) + m_17_9 + ((m_17_10 & 0x07ffffff) << 27) +
                  (m_18_7 >> 27) + m_18_8 + ((m_18_9 & 0x07ffffff) << 27) +
                  (m_19_6 >> 27) + m_19_7 + ((m_19_8 & 0x07ffffff) << 27) + c12;
        let c13 = d13 >> 54;
        let d14 = (m_8_19 >> 27) +
                  (m_9_18 >> 27) + m_9_19 +
                  (m_10_17 >> 27) + m_10_18 + ((m_10_19 & 0x07ffffff) << 27) +
                  (m_11_16 >> 27) + m_11_17 + ((m_11_18 & 0x07ffffff) << 27) +
                  (m_12_15 >> 27) + m_12_16 + ((m_12_17 & 0x07ffffff) << 27) +
                  (m_13_14 >> 27) + m_13_15 + ((m_13_16 & 0x07ffffff) << 27) +
                  (m_14_13 >> 27) + m_14_14 + ((m_14_15 & 0x07ffffff) << 27) +
                  (m_15_12 >> 27) + m_15_13 + ((m_15_14 & 0x07ffffff) << 27) +
                  (m_16_11 >> 27) + m_16_12 + ((m_16_13 & 0x07ffffff) << 27) +
                  (m_17_10 >> 27) + m_17_11 + ((m_17_12 & 0x07ffffff) << 27) +
                  (m_18_9 >> 27) + m_18_10 + ((m_18_11 & 0x07ffffff) << 27) +
                  (m_19_8 >> 27) + m_19_9 + ((m_19_10 & 0x07ffffff) << 27) +
                  c13;
        let c14 = d14 >> 54;
        let d15 = (m_10_19 >> 27) +
                  (m_11_18 >> 27) + m_11_19 +
                  (m_12_17 >> 27) + m_12_18 + ((m_12_19 & 0x07ffffff) << 27) +
                  (m_13_16 >> 27) + m_13_17 + ((m_13_18 & 0x07ffffff) << 27) +
                  (m_14_15 >> 27) + m_14_16 + ((m_14_17 & 0x07ffffff) << 27) +
                  (m_15_14 >> 27) + m_15_15 + ((m_15_16 & 0x07ffffff) << 27) +
                  (m_16_13 >> 27) + m_16_14 + ((m_16_15 & 0x07ffffff) << 27) +
                  (m_17_12 >> 27) + m_17_13 + ((m_17_14 & 0x07ffffff) << 27) +
                  (m_18_11 >> 27) + m_18_12 + ((m_18_13 & 0x07ffffff) << 27) +
                  (m_19_10 >> 27) + m_19_11 + ((m_19_12 & 0x07ffffff) << 27) +
                  c14;
        let c15 = d15 >> 54;
        let d16 = (m_12_19 >> 27) +
                  (m_13_18 >> 27) + m_13_19 +
                  (m_14_17 >> 27) + m_14_18 + ((m_14_19 & 0x07ffffff) << 27) +
                  (m_15_16 >> 27) + m_15_17 + ((m_15_18 & 0x07ffffff) << 27) +
                  (m_16_15 >> 27) + m_16_16 + ((m_16_17 & 0x07ffffff) << 27) +
                  (m_17_14 >> 27) + m_17_15 + ((m_17_16 & 0x07ffffff) << 27) +
                  (m_18_13 >> 27) + m_18_14 + ((m_18_15 & 0x07ffffff) << 27) +
                  (m_19_12 >> 27) + m_19_13 + ((m_19_14 & 0x07ffffff) << 27) +
                  c15;
        let c16 = d16 >> 54;
        let d17 = (m_14_19 >> 27) +
                  (m_15_18 >> 27) + m_15_19 +
                  (m_16_17 >> 27) + m_16_18 + ((m_16_19 & 0x07ffffff) << 27) +
                  (m_17_16 >> 27) + m_17_17 + ((m_17_18 & 0x07ffffff) << 27) +
                  (m_18_15 >> 27) + m_18_16 + ((m_18_17 & 0x07ffffff) << 27) +
                  (m_19_14 >> 27) + m_19_15 + ((m_19_16 & 0x07ffffff) << 27) +
                  c16;
        let c17 = d17 >> 54;
        let d18 = (m_16_19 >> 27) +
                  (m_17_18 >> 27) + m_17_19 +
                  (m_18_17 >> 27) + m_18_18 + ((m_18_19 & 0x07ffffff) << 27) +
                  (m_19_16 >> 27) + m_19_17 + ((m_19_18 & 0x07ffffff) << 27) +
                  c17;
        let c18 = d18 >> 54;
        let d19 = (m_18_19 >> 27) +
                  (m_19_18 >> 27) + m_19_19 + c18;

        // Modular reduction by a pseudo-mersenne prime of the form 2^n - c.

        // These are the n low-order bits.
        let l0_0 = d0 & 0x003fffffffffffff;
        let l1_0 = d1 & 0x003fffffffffffff;
        let l2_0 = d2 & 0x003fffffffffffff;
        let l3_0 = d3 & 0x003fffffffffffff;
        let l4_0 = d4 & 0x003fffffffffffff;
        let l5_0 = d5 & 0x003fffffffffffff;
        let l6_0 = d6 & 0x003fffffffffffff;
        let l7_0 = d7 & 0x003fffffffffffff;
        let l8_0 = d8 & 0x003fffffffffffff;
        let l9_0 = d9 & 0x00000007ffffffff;

        // Shift the high bits down into another n-bit number.
        let h0_0 = ((d9 & 0x003fffffffffffff) >> 35) |
                   ((d10 & 0x00000007ffffffff) << 19);
        let h1_0 = ((d10 & 0x003fffffffffffff) >> 35) |
                   ((d11 & 0x00000007ffffffff) << 19);
        let h2_0 = ((d11 & 0x003fffffffffffff) >> 35) |
                   ((d12 & 0x00000007ffffffff) << 19);
        let h3_0 = ((d12 & 0x003fffffffffffff) >> 35) |
                   ((d13 & 0x00000007ffffffff) << 19);
        let h4_0 = ((d13 & 0x003fffffffffffff) >> 35) |
                   ((d14 & 0x00000007ffffffff) << 19);
        let h5_0 = ((d14 & 0x003fffffffffffff) >> 35) |
                   ((d15 & 0x00000007ffffffff) << 19);
        let h6_0 = ((d15 & 0x003fffffffffffff) >> 35) |
                   ((d16 & 0x00000007ffffffff) << 19);
        let h7_0 = ((d16 & 0x003fffffffffffff) >> 35) |
                   ((d17 & 0x00000007ffffffff) << 19);
        let h8_0 = ((d17 & 0x003fffffffffffff) >> 35) |
                   ((d18 & 0x00000007ffffffff) << 19);
        let h9_0 = ((d18 & 0x003fffffffffffff) >> 35) |
                   (d19 << 19);

        // Normally, we multiply h by c, but since c = 1 here, we skip.

        // Add h and l.

        // Need kin_0
        let kin_0 = h9_0 >> 35;
        let s0_0 = l0_0 + h0_0 + kin_0;
        let k0_0 = s0_0 >> 54;
        let s1_0 = l1_0 + h1_0 + k0_0;
        let k1_0 = s1_0 >> 54;
        let s2_0 = l2_0 + h2_0 + k1_0;
        let k2_0 = s2_0 >> 54;
        let s3_0 = l3_0 + h3_0 + k2_0;
        let k3_0 = s3_0 >> 54;
        let s4_0 = l4_0 + h4_0 + k3_0;
        let k4_0 = s4_0 >> 54;
        let s5_0 = l5_0 + h5_0 + k4_0;
        let k5_0 = s5_0 >> 54;
        let s6_0 = l6_0 + h6_0 + k5_0;
        let k6_0 = s6_0 >> 54;
        let s7_0 = l7_0 + h7_0 + k6_0;
        let k7_0 = s7_0 >> 54;
        let s8_0 = l8_0 + h8_0 + k7_0;
        let k8_0 = s8_0 >> 54;
        let s9_0 = l9_0 + (h9_0 & 0x00000007ffffffff) + k8_0;

        self[0] = s0_0 & 0x003fffffffffffff;
        self[1] = s1_0 & 0x003fffffffffffff;
        self[2] = s2_0 & 0x003fffffffffffff;
        self[3] = s3_0 & 0x003fffffffffffff;
        self[4] = s4_0 & 0x003fffffffffffff;
        self[5] = s5_0 & 0x003fffffffffffff;
        self[6] = s6_0 & 0x003fffffffffffff;
        self[7] = s7_0 & 0x003fffffffffffff;
        self[8] = s8_0 & 0x003fffffffffffff;
        self[9] = s9_0;
    }
}

impl MulAssign<Mod_e521_1> for Mod_e521_1 {
    fn mul_assign(&mut self, rhs: Mod_e521_1) {
        *self *= &rhs;
    }
}

impl<'a> Mul<&'a Mod_e521_1> for i32 {
    type Output = Mod_e521_1;

    fn mul(self, a: &'a Mod_e521_1) -> Mod_e521_1 {
        a.small_mul(self)
    }
}

impl<'a> Mul<&'a Mod_e521_1> for i16 {
    type Output = Mod_e521_1;

    fn mul(self, a: &'a Mod_e521_1) -> Mod_e521_1 {
        a.small_mul(self as i32)
    }
}

impl<'a> Mul<&'a Mod_e521_1> for i8 {
    type Output = Mod_e521_1;

    fn mul(self, a: &'a Mod_e521_1) -> Mod_e521_1 {
        a.small_mul(self as i32)
    }
}

impl<'a> Mul<i32> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: i32) -> Mod_e521_1 {
        self.small_mul(a)
    }
}

impl<'a> Mul<i16> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: i16) -> Mod_e521_1 {
        self.small_mul(a as i32)
    }
}

impl<'a> Mul<i8> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: i8) -> Mod_e521_1 {
        self.small_mul(a as i32)
    }
}

impl Mul<i32> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: i32) -> Mod_e521_1 {
        &self * a
    }
}

impl Mul<i16> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: i16) -> Mod_e521_1 {
        &self * a
    }
}

impl Mul<i8> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: i8) -> Mod_e521_1 {
        &self * a
    }
}

impl<'a, 'b> Mul<&'b Mod_e521_1> for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: &'b Mod_e521_1) -> Mod_e521_1 {
        let mut out = self.clone();
        out *= a;
        out
    }
}

impl Mul<Mod_e521_1> for Mod_e521_1 {
    type Output = Mod_e521_1;

    fn mul(self, a: Mod_e521_1) -> Mod_e521_1 {
        &self * &a
    }
}

impl Rand for Mod_e521_1 {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut out = Mod_e521_1([0i64; 10]);

        for i in 0..10 {
            out[i] = rng.gen_range(0, MODULUS[i]);
        }

        out
    }
}

impl Pack for Mod_e521_1 {
    fn unpack(&mut self, bytes: &[u8]) {
        self[0] = ((bytes[0] as i64) & 0x00000000000000ff) |
                  (((bytes[1] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[2] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[3] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[4] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[5] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[6] as i64) << 48) & 0x003f000000000000);
        self[1] = (((bytes[6] as i64) >> 6) & 0x0000000000000003) |
                  (((bytes[7] as i64) << 2) & 0x00000000000003fc) |
                  (((bytes[8] as i64) << 10) & 0x000000000003fc00) |
                  (((bytes[9] as i64) << 18) & 0x0000000003fc0000) |
                  (((bytes[10] as i64) << 26) & 0x00000003fc000000) |
                  (((bytes[11] as i64) << 34) & 0x000003fc00000000) |
                  (((bytes[12] as i64) << 42) & 0x0003fc0000000000) |
                  (((bytes[13] as i64) << 50) & 0x003c000000000000);
        self[2] = (((bytes[13] as i64) >> 4) & 0x000000000000000f) |
                  (((bytes[14] as i64) << 4) & 0x0000000000000ff0) |
                  (((bytes[15] as i64) << 12) & 0x00000000000ff000) |
                  (((bytes[16] as i64) << 20) & 0x000000000ff00000) |
                  (((bytes[17] as i64) << 28) & 0x0000000ff0000000) |
                  (((bytes[18] as i64) << 36) & 0x00000ff000000000) |
                  (((bytes[19] as i64) << 44) & 0x000ff00000000000) |
                  (((bytes[20] as i64) << 52) & 0x0030000000000000);
        self[3] = (((bytes[20] as i64) >> 2) & 0x000000000000003f) |
                  (((bytes[21] as i64) << 6) & 0x0000000000003fc0) |
                  (((bytes[22] as i64) << 14) & 0x00000000003fc000) |
                  (((bytes[23] as i64) << 22) & 0x000000003fc00000) |
                  (((bytes[24] as i64) << 30) & 0x0000003fc0000000) |
                  (((bytes[25] as i64) << 38) & 0x00003fc000000000) |
                  (((bytes[26] as i64) << 46) & 0x003fc00000000000);
        self[4] = ((bytes[27] as i64) & 0x00000000000000ff) |
                  (((bytes[28] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[29] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[30] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[31] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[32] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[33] as i64) << 48) & 0x003f000000000000);
        self[5] = (((bytes[33] as i64) >> 6) & 0x0000000000000003) |
                  (((bytes[34] as i64) << 2) & 0x00000000000003fc) |
                  (((bytes[35] as i64) << 10) & 0x000000000003fc00) |
                  (((bytes[36] as i64) << 18) & 0x0000000003fc0000) |
                  (((bytes[37] as i64) << 26) & 0x00000003fc000000) |
                  (((bytes[38] as i64) << 34) & 0x000003fc00000000) |
                  (((bytes[39] as i64) << 42) & 0x0003fc0000000000) |
                  (((bytes[40] as i64) << 50) & 0x003c000000000000);
        self[6] = (((bytes[40] as i64) >> 4) & 0x000000000000000f) |
                  (((bytes[41] as i64) << 4) & 0x0000000000000ff0) |
                  (((bytes[42] as i64) << 12) & 0x00000000000ff000) |
                  (((bytes[43] as i64) << 20) & 0x000000000ff00000) |
                  (((bytes[44] as i64) << 28) & 0x0000000ff0000000) |
                  (((bytes[45] as i64) << 36) & 0x00000ff000000000) |
                  (((bytes[46] as i64) << 44) & 0x000ff00000000000) |
                  (((bytes[47] as i64) << 52) & 0x0030000000000000);
        self[7] = (((bytes[47] as i64) >> 2) & 0x000000000000003f) |
                  (((bytes[48] as i64) << 6) & 0x0000000000003fc0) |
                  (((bytes[49] as i64) << 14) & 0x00000000003fc000) |
                  (((bytes[50] as i64) << 22) & 0x000000003fc00000) |
                  (((bytes[51] as i64) << 30) & 0x0000003fc0000000) |
                  (((bytes[52] as i64) << 38) & 0x00003fc000000000) |
                  (((bytes[53] as i64) << 46) & 0x003fc00000000000);
        self[8] = ((bytes[54] as i64) & 0x00000000000000ff) |
                  (((bytes[55] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[56] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[57] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[58] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[59] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[60] as i64) << 48) & 0x003f000000000000);
        self[9] = (((bytes[60] as i64) >> 6) &  0x0000000000000003) |
                  (((bytes[61] as i64) << 2) &  0x00000000000003fc) |
                  (((bytes[62] as i64) << 10) & 0x000000000003fc00) |
                  (((bytes[63] as i64) << 18) & 0x0000000003fc0000) |
                  (((bytes[64] as i64) << 26) & 0x00000003fc000000) |
                  (((bytes[65] as i64) << 34) & 0x0000000400000000);
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
        bytes[0] = (self[0] & 0b11111111) as u8;
        bytes[1] = ((self[0] >> 8) & 0b11111111) as u8;
        bytes[2] = ((self[0] >> 16) & 0b11111111) as u8;
        bytes[3] = ((self[0] >> 24) & 0b11111111) as u8;
        bytes[4] = ((self[0] >> 32) & 0b11111111) as u8;
        bytes[5] = ((self[0] >> 40) & 0b11111111) as u8;
        bytes[6] = (((self[0] >> 48) & 0b00111111) |
                    ((self[1] << 6) & 0b11000000)) as u8;
        bytes[7] = ((self[1] >> 2) & 0b11111111) as u8;
        bytes[8] = ((self[1] >> 10) & 0b11111111) as u8;
        bytes[9] = ((self[1] >> 18) & 0b11111111) as u8;
        bytes[10] = ((self[1] >> 26) & 0b11111111) as u8;
        bytes[11] = ((self[1] >> 34) & 0b11111111) as u8;
        bytes[12] = ((self[1] >> 42) & 0b11111111) as u8;
        bytes[13] = (((self[1] >> 50) & 0b00001111) |
                     ((self[2] << 4) & 0b11110000)) as u8;
        bytes[14] = ((self[2] >> 4) & 0b11111111) as u8;
        bytes[15] = ((self[2] >> 12) & 0b11111111) as u8;
        bytes[16] = ((self[2] >> 20) & 0b11111111) as u8;
        bytes[17] = ((self[2] >> 28) & 0b11111111) as u8;
        bytes[18] = ((self[2] >> 36) & 0b11111111) as u8;
        bytes[19] = ((self[2] >> 44) & 0b11111111) as u8;
        bytes[20] = (((self[2] >> 52) & 0b00000011) |
                     ((self[3] << 2) & 0b11111100)) as u8;
        bytes[21] = ((self[3] >> 6) & 0b11111111) as u8;
        bytes[22] = ((self[3] >> 14) & 0b11111111) as u8;
        bytes[23] = ((self[3] >> 22) & 0b11111111) as u8;
        bytes[24] = ((self[3] >> 30) & 0b11111111) as u8;
        bytes[25] = ((self[3] >> 38) & 0b11111111) as u8;
        bytes[26] = ((self[3] >> 46) & 0b11111111) as u8;
        bytes[27] = (self[4] & 0b11111111) as u8;
        bytes[28] = ((self[4] >> 8) & 0b11111111) as u8;
        bytes[29] = ((self[4] >> 16) & 0b11111111) as u8;
        bytes[30] = ((self[4] >> 24) & 0b11111111) as u8;
        bytes[31] = ((self[4] >> 32) & 0b11111111) as u8;
        bytes[32] = ((self[4] >> 40) & 0b11111111) as u8;
        bytes[33] = (((self[4] >> 48) & 0b00111111) |
                     ((self[5] << 6) & 0b11000000)) as u8;
        bytes[34] = ((self[5] >> 2) & 0b11111111) as u8;
        bytes[35] = ((self[5] >> 10) & 0b11111111) as u8;
        bytes[36] = ((self[5] >> 18) & 0b11111111) as u8;
        bytes[37] = ((self[5] >> 26) & 0b11111111) as u8;
        bytes[38] = ((self[5] >> 34) & 0b11111111) as u8;
        bytes[39] = ((self[5] >> 42) & 0b11111111) as u8;
        bytes[40] = (((self[5] >> 50) & 0b00001111) |
                     ((self[6] << 4) & 0b11110000)) as u8;
        bytes[41] = ((self[6] >> 4) & 0b11111111) as u8;
        bytes[42] = ((self[6] >> 12) & 0b11111111) as u8;
        bytes[43] = ((self[6] >> 20) & 0b11111111) as u8;
        bytes[44] = ((self[6] >> 28) & 0b11111111) as u8;
        bytes[45] = ((self[6] >> 36) & 0b11111111) as u8;
        bytes[46] = ((self[6] >> 44) & 0b11111111) as u8;
        bytes[47] = (((self[6] >> 52) & 0b00000011) |
                     ((self[7] << 2) & 0b11111100)) as u8;
        bytes[48] = ((self[7] >> 6) & 0b11111111) as u8;
        bytes[49] = ((self[7] >> 14) & 0b11111111) as u8;
        bytes[50] = ((self[7] >> 22) & 0b11111111) as u8;
        bytes[51] = ((self[7] >> 30) & 0b11111111) as u8;
        bytes[52] = ((self[7] >> 38) & 0b11111111) as u8;
        bytes[53] = ((self[7] >> 46) & 0b11111111) as u8;
        bytes[54] = (self[8] & 0b11111111) as u8;
        bytes[55] = ((self[8] >> 8) & 0b11111111) as u8;
        bytes[56] = ((self[8] >> 16) & 0b11111111) as u8;
        bytes[57] = ((self[8] >> 24) & 0b11111111) as u8;
        bytes[58] = ((self[8] >> 32) & 0b11111111) as u8;
        bytes[59] = ((self[8] >> 40) & 0b11111111) as u8;
        bytes[60] = (((self[8] >> 48) & 0b00111111) |
                     ((self[9] << 6) & 0b11000000)) as u8;
        bytes[61] = ((self[9] >> 2) & 0b11111111) as u8;
        bytes[62] = ((self[9] >> 10) & 0b11111111) as u8;
        bytes[63] = ((self[9] >> 18) & 0b11111111) as u8;
        bytes[64] = ((self[9] >> 26) & 0b11111111) as u8;
        bytes[65] = ((self[9] >> 34) & 0b00000001) as u8;
    }

    fn nbytes() -> i32 {
        66
    }
}

impl PrimeField for Mod_e521_1 {
    fn fill(&mut self, bit: bool) {
        let mut mask = bit as i64;

        mask |= mask << 1;
        mask |= mask << 2;
        mask |= mask << 4;
        mask |= mask << 8;
        mask |= mask << 16;
        mask |= mask << 32;

        self[0] = mask;
        self[1] = mask;
        self[2] = mask;
        self[3] = mask;
        self[4] = mask;
        self[5] = mask;
        self[6] = mask;
        self[7] = mask;
        self[8] = mask;
        self[9] = mask;
    }

    fn filled(bit: bool) -> Self {
        let mut mask = bit as i64;

        mask |= mask << 1;
        mask |= mask << 2;
        mask |= mask << 4;
        mask |= mask << 8;
        mask |= mask << 16;
        mask |= mask << 32;

        Mod_e521_1([mask; 10])
    }

    fn nbits() -> i32 {
        521
    }

    fn bit_normalized(&self, idx: usize) -> bool {
        let byte = idx / 54;
        let bit = idx % 54;

        (self[byte] >> bit) & 0x1 == 0x1
    }

    fn bit(&mut self, idx: usize) -> bool {
        self.normalize();
        self.bit_normalized(idx)
    }

    fn normalize(&mut self) {
        let plusone = &*self + (C_VAL as i32);
        let offset = &MODULUS * (plusone.carry_out() as i32);
        *self -= &offset;
    }

    fn normalize_self_eq(&mut self, other: &Self) -> bool {
        let mut are_equal: bool = true;

        self.normalize();

        are_equal &= self[0] == other[0];
        are_equal &= self[1] == other[1];
        are_equal &= self[2] == other[2];
        are_equal &= self[3] == other[3];
        are_equal &= self[4] == other[4];
        are_equal &= self[5] == other[5];
        are_equal &= self[6] == other[6];
        are_equal &= self[7] == other[7];
        are_equal &= self[8] == other[8];
        are_equal &= self[9] == other[9];

        are_equal
    }

    fn normalize_eq(&mut self, other: &mut Self) -> bool {
        let mut are_equal: bool = true;

        self.normalize();
        other.normalize();

        for i in 0..10 {
            are_equal &= self[i] == other[i];
        }

        are_equal
    }

    fn zero() -> Mod_e521_1 {
        return ZERO;
    }

    fn one() -> Mod_e521_1 {
        return ONE;
    }

    fn m_one() -> Mod_e521_1 {
        return M_ONE;
    }

    fn modulus() -> Mod_e521_1 {
        return MODULUS;
    }

    fn square(&mut self) {
        let a0 = self[0] & 0x07ffffff;
        let a1 = self[0] >> 27;
        let a2 = self[1] & 0x07ffffff;
        let a3 = self[1] >> 27;
        let a4 = self[2] & 0x07ffffff;
        let a5 = self[2] >> 27;
        let a6 = self[3] & 0x07ffffff;
        let a7 = self[3] >> 27;
        let a8 = self[4] & 0x07ffffff;
        let a9 = self[4] >> 27;
        let a10 = self[5] & 0x07ffffff;
        let a11 = self[5] >> 27;
        let a12 = self[6] & 0x07ffffff;
        let a13 = self[6] >> 27;
        let a14 = self[7] & 0x07ffffff;
        let a15 = self[7] >> 27;
        let a16 = self[8] & 0x07ffffff;
        let a17 = self[8] >> 27;
        let a18 = self[9] & 0x07ffffff;
        let a19 = self[9] >> 27;

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
        let m_0_14 = a0 * a14;
        let m_0_15 = a0 * a15;
        let m_0_16 = a0 * a16;
        let m_0_17 = a0 * a17;
        let m_0_18 = a0 * a18;
        let m_0_19 = a0 * a19;
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
        let m_1_14 = a1 * a14;
        let m_1_15 = a1 * a15;
        let m_1_16 = a1 * a16;
        let m_1_17 = a1 * a17;
        let m_1_18 = a1 * a18;
        let m_1_19 = a1 * a19;
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
        let m_2_14 = a2 * a14;
        let m_2_15 = a2 * a15;
        let m_2_16 = a2 * a16;
        let m_2_17 = a2 * a17;
        let m_2_18 = a2 * a18;
        let m_2_19 = a2 * a19;
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
        let m_3_14 = a3 * a14;
        let m_3_15 = a3 * a15;
        let m_3_16 = a3 * a16;
        let m_3_17 = a3 * a17;
        let m_3_18 = a3 * a18;
        let m_3_19 = a3 * a19;
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
        let m_4_14 = a4 * a14;
        let m_4_15 = a4 * a15;
        let m_4_16 = a4 * a16;
        let m_4_17 = a4 * a17;
        let m_4_18 = a4 * a18;
        let m_4_19 = a4 * a19;
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
        let m_5_14 = a5 * a14;
        let m_5_15 = a5 * a15;
        let m_5_16 = a5 * a16;
        let m_5_17 = a5 * a17;
        let m_5_18 = a5 * a18;
        let m_5_19 = a5 * a19;
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
        let m_6_14 = a6 * a14;
        let m_6_15 = a6 * a15;
        let m_6_16 = a6 * a16;
        let m_6_17 = a6 * a17;
        let m_6_18 = a6 * a18;
        let m_6_19 = a6 * a19;
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
        let m_7_14 = a7 * a14;
        let m_7_15 = a7 * a15;
        let m_7_16 = a7 * a16;
        let m_7_17 = a7 * a17;
        let m_7_18 = a7 * a18;
        let m_7_19 = a7 * a19;
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
        let m_8_14 = a8 * a14;
        let m_8_15 = a8 * a15;
        let m_8_16 = a8 * a16;
        let m_8_17 = a8 * a17;
        let m_8_18 = a8 * a18;
        let m_8_19 = a8 * a19;
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
        let m_9_14 = a9 * a14;
        let m_9_15 = a9 * a15;
        let m_9_16 = a9 * a16;
        let m_9_17 = a9 * a17;
        let m_9_18 = a9 * a18;
        let m_9_19 = a9 * a19;
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
        let m_10_14 = a10 * a14;
        let m_10_15 = a10 * a15;
        let m_10_16 = a10 * a16;
        let m_10_17 = a10 * a17;
        let m_10_18 = a10 * a18;
        let m_10_19 = a10 * a19;
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
        let m_11_14 = a11 * a14;
        let m_11_15 = a11 * a15;
        let m_11_16 = a11 * a16;
        let m_11_17 = a11 * a17;
        let m_11_18 = a11 * a18;
        let m_11_19 = a11 * a19;
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
        let m_12_14 = a12 * a14;
        let m_12_15 = a12 * a15;
        let m_12_16 = a12 * a16;
        let m_12_17 = a12 * a17;
        let m_12_18 = a12 * a18;
        let m_12_19 = a12 * a19;
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
        let m_13_14 = a13 * a14;
        let m_13_15 = a13 * a15;
        let m_13_16 = a13 * a16;
        let m_13_17 = a13 * a17;
        let m_13_18 = a13 * a18;
        let m_13_19 = a13 * a19;
        let m_14_0 = m_0_14;
        let m_14_1 = m_1_14;
        let m_14_2 = m_2_14;
        let m_14_3 = m_3_14;
        let m_14_4 = m_4_14;
        let m_14_5 = m_5_14;
        let m_14_6 = m_6_14;
        let m_14_7 = m_7_14;
        let m_14_8 = m_8_14;
        let m_14_9 = m_9_14;
        let m_14_10 = m_10_14;
        let m_14_11 = m_11_14;
        let m_14_12 = m_12_14;
        let m_14_13 = m_13_14;
        let m_14_14 = a14 * a14;
        let m_14_15 = a14 * a15;
        let m_14_16 = a14 * a16;
        let m_14_17 = a14 * a17;
        let m_14_18 = a14 * a18;
        let m_14_19 = a14 * a19;
        let m_15_0 = m_0_15;
        let m_15_1 = m_1_15;
        let m_15_2 = m_2_15;
        let m_15_3 = m_3_15;
        let m_15_4 = m_4_15;
        let m_15_5 = m_5_15;
        let m_15_6 = m_6_15;
        let m_15_7 = m_7_15;
        let m_15_8 = m_8_15;
        let m_15_9 = m_9_15;
        let m_15_10 = m_10_15;
        let m_15_11 = m_11_15;
        let m_15_12 = m_12_15;
        let m_15_13 = m_13_15;
        let m_15_14 = m_14_15;
        let m_15_15 = a15 * a15;
        let m_15_16 = a15 * a16;
        let m_15_17 = a15 * a17;
        let m_15_18 = a15 * a18;
        let m_15_19 = a15 * a19;
        let m_16_0 = m_0_16;
        let m_16_1 = m_1_16;
        let m_16_2 = m_2_16;
        let m_16_3 = m_3_16;
        let m_16_4 = m_4_16;
        let m_16_5 = m_5_16;
        let m_16_6 = m_6_16;
        let m_16_7 = m_7_16;
        let m_16_8 = m_8_16;
        let m_16_9 = m_9_16;
        let m_16_10 = m_10_16;
        let m_16_11 = m_11_16;
        let m_16_12 = m_12_16;
        let m_16_13 = m_13_16;
        let m_16_14 = m_14_16;
        let m_16_15 = m_15_16;
        let m_16_16 = a16 * a16;
        let m_16_17 = a16 * a17;
        let m_16_18 = a16 * a18;
        let m_16_19 = a16 * a19;
        let m_17_0 = m_0_17;
        let m_17_1 = m_1_17;
        let m_17_2 = m_2_17;
        let m_17_3 = m_3_17;
        let m_17_4 = m_4_17;
        let m_17_5 = m_5_17;
        let m_17_6 = m_6_17;
        let m_17_7 = m_7_17;
        let m_17_8 = m_8_17;
        let m_17_9 = m_9_17;
        let m_17_10 = m_10_17;
        let m_17_11 = m_11_17;
        let m_17_12 = m_12_17;
        let m_17_13 = m_13_17;
        let m_17_14 = m_14_17;
        let m_17_15 = m_15_17;
        let m_17_16 = m_16_17;
        let m_17_17 = a17 * a17;
        let m_17_18 = a17 * a18;
        let m_17_19 = a17 * a19;
        let m_18_0 = m_0_18;
        let m_18_1 = m_1_18;
        let m_18_2 = m_2_18;
        let m_18_3 = m_3_18;
        let m_18_4 = m_4_18;
        let m_18_5 = m_5_18;
        let m_18_6 = m_6_18;
        let m_18_7 = m_7_18;
        let m_18_8 = m_8_18;
        let m_18_9 = m_9_18;
        let m_18_10 = m_10_18;
        let m_18_11 = m_11_18;
        let m_18_12 = m_12_18;
        let m_18_13 = m_13_18;
        let m_18_14 = m_14_18;
        let m_18_15 = m_15_18;
        let m_18_16 = m_16_18;
        let m_18_17 = m_17_18;
        let m_18_18 = a18 * a18;
        let m_18_19 = a18 * a19;
        let m_19_0 = m_0_19;
        let m_19_1 = m_1_19;
        let m_19_2 = m_2_19;
        let m_19_3 = m_3_19;
        let m_19_4 = m_4_19;
        let m_19_5 = m_5_19;
        let m_19_6 = m_6_19;
        let m_19_7 = m_7_19;
        let m_19_8 = m_8_19;
        let m_19_9 = m_9_19;
        let m_19_10 = m_10_19;
        let m_19_11 = m_11_19;
        let m_19_12 = m_12_19;
        let m_19_13 = m_13_19;
        let m_19_14 = m_14_19;
        let m_19_15 = m_15_19;
        let m_19_16 = m_16_19;
        let m_19_17 = m_17_19;
        let m_19_18 = m_18_19;
        let m_19_19 = a19 * a19;

        // Compute the 40-digit combined product using 64-bit operations.
        let d0 = m_0_0 + ((m_0_1 & 0x07ffffff) << 27) +
                 ((m_1_0 & 0x07ffffff) << 27);
        let c0 = d0 >> 54;
        let d1 = (m_0_1 >> 27) + m_0_2 + ((m_0_3 & 0x07ffffff) << 27) +
                 (m_1_0 >> 27) + m_1_1 + ((m_1_2 & 0x07ffffff) << 27) +
                 m_2_0 + ((m_2_1 & 0x07ffffff) << 27) +
                 ((m_3_0 & 0x07ffffff) << 27) + c0;
        let c1 = d1 >> 54;
        let d2 = (m_0_3 >> 27) + m_0_4 + ((m_0_5 & 0x07ffffff) << 27) +
                 (m_1_2 >> 27) + m_1_3 + ((m_1_4 & 0x07ffffff) << 27) +
                 (m_2_1 >> 27) + m_2_2 + ((m_2_3 & 0x07ffffff) << 27) +
                 (m_3_0 >> 27) + m_3_1 + ((m_3_2 & 0x07ffffff) << 27) +
                 m_4_0 + ((m_4_1 & 0x07ffffff) << 27) +
                 ((m_5_0 & 0x07ffffff) << 27) + c1;
        let c2 = d2 >> 54;
        let d3 = (m_0_5 >> 27) + m_0_6 + ((m_0_7 & 0x07ffffff) << 27) +
                 (m_1_4 >> 27) + m_1_5 + ((m_1_6 & 0x07ffffff) << 27) +
                 (m_2_3 >> 27) + m_2_4 + ((m_2_5 & 0x07ffffff) << 27) +
                 (m_3_2 >> 27) + m_3_3 + ((m_3_4 & 0x07ffffff) << 27) +
                 (m_4_1 >> 27) + m_4_2 + ((m_4_3 & 0x07ffffff) << 27) +
                 (m_5_0 >> 27) + m_5_1 + ((m_5_2 & 0x07ffffff) << 27) +
                 m_6_0 + ((m_6_1 & 0x07ffffff) << 27) +
                 ((m_7_0 & 0x07ffffff) << 27) + c2;
        let c3 = d3 >> 54;
        let d4 = (m_0_7 >> 27) + m_0_8 + ((m_0_9 & 0x07ffffff) << 27) +
                 (m_1_6 >> 27) + m_1_7 + ((m_1_8 & 0x07ffffff) << 27) +
                 (m_2_5 >> 27) + m_2_6 + ((m_2_7 & 0x07ffffff) << 27) +
                 (m_3_4 >> 27) + m_3_5 + ((m_3_6 & 0x07ffffff) << 27) +
                 (m_4_3 >> 27) + m_4_4 + ((m_4_5 & 0x07ffffff) << 27) +
                 (m_5_2 >> 27) + m_5_3 + ((m_5_4 & 0x07ffffff) << 27) +
                 (m_6_1 >> 27) + m_6_2 + ((m_6_3 & 0x07ffffff) << 27) +
                 (m_7_0 >> 27) + m_7_1 + ((m_7_2 & 0x07ffffff) << 27) +
                 m_8_0 + ((m_8_1 & 0x07ffffff) << 27) +
                 ((m_9_0 & 0x07ffffff) << 27) + c3;
        let c4 = d4 >> 54;
        let d5 = (m_0_9 >> 27) + m_0_10 + ((m_0_11 & 0x07ffffff) << 27) +
                 (m_1_8 >> 27) + m_1_9 + ((m_1_10 & 0x07ffffff) << 27) +
                 (m_2_7 >> 27) + m_2_8 + ((m_2_9 & 0x07ffffff) << 27) +
                 (m_3_6 >> 27) + m_3_7 + ((m_3_8 & 0x07ffffff) << 27) +
                 (m_4_5 >> 27) + m_4_6 + ((m_4_7 & 0x07ffffff) << 27) +
                 (m_5_4 >> 27) + m_5_5 + ((m_5_6 & 0x07ffffff) << 27) +
                 (m_6_3 >> 27) + m_6_4 + ((m_6_5 & 0x07ffffff) << 27) +
                 (m_7_2 >> 27) + m_7_3 + ((m_7_4 & 0x07ffffff) << 27) +
                 (m_8_1 >> 27) + m_8_2 + ((m_8_3 & 0x07ffffff) << 27) +
                 (m_9_0 >> 27) + m_9_1 + ((m_9_2 & 0x07ffffff) << 27) +
                 m_10_0 + ((m_10_1 & 0x07ffffff) << 27) +
                 ((m_11_0 & 0x07ffffff) << 27) + c4;
        let c5 = d5 >> 54;
        let d6 = (m_0_11 >> 27) + m_0_12 + ((m_0_13 & 0x07ffffff) << 27) +
                 (m_1_10 >> 27) + m_1_11 + ((m_1_12 & 0x07ffffff) << 27) +
                 (m_2_9 >> 27) + m_2_10 + ((m_2_11 & 0x07ffffff) << 27) +
                 (m_3_8 >> 27) + m_3_9 + ((m_3_10 & 0x07ffffff) << 27) +
                 (m_4_7 >> 27) + m_4_8 + ((m_4_9 & 0x07ffffff) << 27) +
                 (m_5_6 >> 27) + m_5_7 + ((m_5_8 & 0x07ffffff) << 27) +
                 (m_6_5 >> 27) + m_6_6 + ((m_6_7 & 0x07ffffff) << 27) +
                 (m_7_4 >> 27) + m_7_5 + ((m_7_6 & 0x07ffffff) << 27) +
                 (m_8_3 >> 27) + m_8_4 + ((m_8_5 & 0x07ffffff) << 27) +
                 (m_9_2 >> 27) + m_9_3 + ((m_9_4 & 0x07ffffff) << 27) +
                 (m_10_1 >> 27) + m_10_2 + ((m_10_3 & 0x07ffffff) << 27) +
                 (m_11_0 >> 27) + m_11_1 + ((m_11_2 & 0x07ffffff) << 27) +
                 m_12_0 + ((m_12_1 & 0x07ffffff) << 27) +
                 ((m_13_0 & 0x07ffffff) << 27) + c5;
        let c6 = d6 >> 54;
        let d7 = (m_0_13 >> 27) + m_0_14 + ((m_0_15 & 0x07ffffff) << 27) +
                 (m_1_12 >> 27) + m_1_13 + ((m_1_14 & 0x07ffffff) << 27) +
                 (m_2_11 >> 27) + m_2_12 + ((m_2_13 & 0x07ffffff) << 27) +
                 (m_3_10 >> 27) + m_3_11 + ((m_3_12 & 0x07ffffff) << 27) +
                 (m_4_9 >> 27) + m_4_10 + ((m_4_11 & 0x07ffffff) << 27) +
                 (m_5_8 >> 27) + m_5_9 + ((m_5_10 & 0x07ffffff) << 27) +
                 (m_6_7 >> 27) + m_6_8 + ((m_6_9 & 0x07ffffff) << 27) +
                 (m_7_6 >> 27) + m_7_7 + ((m_7_8 & 0x07ffffff) << 27) +
                 (m_8_5 >> 27) + m_8_6 + ((m_8_7 & 0x07ffffff) << 27) +
                 (m_9_4 >> 27) + m_9_5 + ((m_9_6 & 0x07ffffff) << 27) +
                 (m_10_3 >> 27) + m_10_4 + ((m_10_5 & 0x07ffffff) << 27) +
                 (m_11_2 >> 27) + m_11_3 + ((m_11_4 & 0x07ffffff) << 27) +
                 (m_12_1 >> 27) + m_12_2 + ((m_12_3 & 0x07ffffff) << 27) +
                 (m_13_0 >> 27) + m_13_1 + ((m_13_2 & 0x07ffffff) << 27) +
                 m_14_0 + ((m_14_1 & 0x07ffffff) << 27) +
                 ((m_15_0 & 0x07ffffff) << 27) + c6;
        let c7 = d7 >> 54;
        let d8 = (m_0_15 >> 27) + m_0_16 + ((m_0_17 & 0x07ffffff) << 27) +
                 (m_1_14 >> 27) + m_1_15 + ((m_1_16 & 0x07ffffff) << 27) +
                 (m_2_13 >> 27) + m_2_14 + ((m_2_15 & 0x07ffffff) << 27) +
                 (m_3_12 >> 27) + m_3_13 + ((m_3_14 & 0x07ffffff) << 27) +
                 (m_4_11 >> 27) + m_4_12 + ((m_4_13 & 0x07ffffff) << 27) +
                 (m_5_10 >> 27) + m_5_11 + ((m_5_12 & 0x07ffffff) << 27) +
                 (m_6_9 >> 27) + m_6_10 + ((m_6_11 & 0x07ffffff) << 27) +
                 (m_7_8 >> 27) + m_7_9 + ((m_7_10 & 0x07ffffff) << 27) +
                 (m_8_7 >> 27) + m_8_8 + ((m_8_9 & 0x07ffffff) << 27) +
                 (m_9_6 >> 27) + m_9_7 + ((m_9_8 & 0x07ffffff) << 27) +
                 (m_10_5 >> 27) + m_10_6 + ((m_10_7 & 0x07ffffff) << 27) +
                 (m_11_4 >> 27) + m_11_5 + ((m_11_6 & 0x07ffffff) << 27) +
                 (m_12_3 >> 27) + m_12_4 + ((m_12_5 & 0x07ffffff) << 27) +
                 (m_13_2 >> 27) + m_13_3 + ((m_13_4 & 0x07ffffff) << 27) +
                 (m_14_1 >> 27) + m_14_2 + ((m_14_3 & 0x07ffffff) << 27) +
                 (m_15_0 >> 27) + m_15_1 + ((m_15_2 & 0x07ffffff) << 27) +
                 m_16_0 + ((m_16_1 & 0x07ffffff) << 27) +
                 ((m_17_0 & 0x07ffffff) << 27) + c7;
        let c8 = d8 >> 54;
        let d9 = (m_0_17 >> 27) + m_0_18 + ((m_0_19 & 0x07ffffff) << 27) +
                 (m_1_16 >> 27) + m_1_17 + ((m_1_18 & 0x07ffffff) << 27) +
                 (m_2_15 >> 27) + m_2_16 + ((m_2_17 & 0x07ffffff) << 27) +
                 (m_3_14 >> 27) + m_3_15 + ((m_3_16 & 0x07ffffff) << 27) +
                 (m_4_13 >> 27) + m_4_14 + ((m_4_15 & 0x07ffffff) << 27) +
                 (m_5_12 >> 27) + m_5_13 + ((m_5_14 & 0x07ffffff) << 27) +
                 (m_6_11 >> 27) + m_6_12 + ((m_6_13 & 0x07ffffff) << 27) +
                 (m_7_10 >> 27) + m_7_11 + ((m_7_12 & 0x07ffffff) << 27) +
                 (m_8_9 >> 27) + m_8_10 + ((m_8_11 & 0x07ffffff) << 27) +
                 (m_9_8 >> 27) + m_9_9 + ((m_9_10 & 0x07ffffff) << 27) +
                 (m_10_7 >> 27) + m_10_8 + ((m_10_9 & 0x07ffffff) << 27) +
                 (m_11_6 >> 27) + m_11_7 + ((m_11_8 & 0x07ffffff) << 27) +
                 (m_12_5 >> 27) + m_12_6 + ((m_12_7 & 0x07ffffff) << 27) +
                 (m_13_4 >> 27) + m_13_5 + ((m_13_6 & 0x07ffffff) << 27) +
                 (m_14_3 >> 27) + m_14_4 + ((m_14_5 & 0x07ffffff) << 27) +
                 (m_15_2 >> 27) + m_15_3 + ((m_15_4 & 0x07ffffff) << 27) +
                 (m_16_1 >> 27) + m_16_2 + ((m_16_3 & 0x07ffffff) << 27) +
                 (m_17_0 >> 27) + m_17_1 + ((m_17_2 & 0x07ffffff) << 27) +
                 m_18_0 + ((m_18_1 & 0x07ffffff) << 27) +
                 ((m_19_0 & 0x07ffffff) << 27) + c8;
        let c9 = d9 >> 54;
        let d10 = (m_0_19 >> 27) +
                  (m_1_18 >> 27) + m_1_19 +
                  (m_2_17 >> 27) + m_2_18 + ((m_2_19 & 0x07ffffff) << 27) +
                  (m_3_16 >> 27) + m_3_17 + ((m_3_18 & 0x07ffffff) << 27) +
                  (m_4_15 >> 27) + m_4_16 + ((m_4_17 & 0x07ffffff) << 27) +
                  (m_5_14 >> 27) + m_5_15 + ((m_5_16 & 0x07ffffff) << 27) +
                  (m_6_13 >> 27) + m_6_14 + ((m_6_15 & 0x07ffffff) << 27) +
                  (m_7_12 >> 27) + m_7_13 + ((m_7_14 & 0x07ffffff) << 27) +
                  (m_8_11 >> 27) + m_8_12 + ((m_8_13 & 0x07ffffff) << 27) +
                  (m_9_10 >> 27) + m_9_11 + ((m_9_12 & 0x07ffffff) << 27) +
                  (m_10_9 >> 27) + m_10_10 + ((m_10_11 & 0x07ffffff) << 27) +
                  (m_11_8 >> 27) + m_11_9 + ((m_11_10 & 0x07ffffff) << 27) +
                  (m_12_7 >> 27) + m_12_8 + ((m_12_9 & 0x07ffffff) << 27) +
                  (m_13_6 >> 27) + m_13_7 + ((m_13_8 & 0x07ffffff) << 27) +
                  (m_14_5 >> 27) + m_14_6 + ((m_14_7 & 0x07ffffff) << 27) +
                  (m_15_4 >> 27) + m_15_5 + ((m_15_6 & 0x07ffffff) << 27) +
                  (m_16_3 >> 27) + m_16_4 + ((m_16_5 & 0x07ffffff) << 27) +
                  (m_17_2 >> 27) + m_17_3 + ((m_17_4 & 0x07ffffff) << 27) +
                  (m_18_1 >> 27) + m_18_2 + ((m_18_3 & 0x07ffffff) << 27) +
                  (m_19_0 >> 27) + m_19_1 + ((m_19_2 & 0x07ffffff) << 27) + c9;
        let c10 = d10 >> 54;
        let d11 = (m_2_19 >> 27) +
                  (m_3_18 >> 27) + m_3_19 +
                  (m_4_17 >> 27) + m_4_18 + ((m_4_19 & 0x07ffffff) << 27) +
                  (m_5_16 >> 27) + m_5_17 + ((m_5_18 & 0x07ffffff) << 27) +
                  (m_6_15 >> 27) + m_6_16 + ((m_6_17 & 0x07ffffff) << 27) +
                  (m_7_14 >> 27) + m_7_15 + ((m_7_16 & 0x07ffffff) << 27) +
                  (m_8_13 >> 27) + m_8_14 + ((m_8_15 & 0x07ffffff) << 27) +
                  (m_9_12 >> 27) + m_9_13 + ((m_9_14 & 0x07ffffff) << 27) +
                  (m_10_11 >> 27) + m_10_12 + ((m_10_13 & 0x07ffffff) << 27) +
                  (m_11_10 >> 27) + m_11_11 + ((m_11_12 & 0x07ffffff) << 27) +
                  (m_12_9 >> 27) + m_12_10 + ((m_12_11 & 0x07ffffff) << 27) +
                  (m_13_8 >> 27) + m_13_9 + ((m_13_10 & 0x07ffffff) << 27) +
                  (m_14_7 >> 27) + m_14_8 + ((m_14_9 & 0x07ffffff) << 27) +
                  (m_15_6 >> 27) + m_15_7 + ((m_15_8 & 0x07ffffff) << 27) +
                  (m_16_5 >> 27) + m_16_6 + ((m_16_7 & 0x07ffffff) << 27) +
                  (m_17_4 >> 27) + m_17_5 + ((m_17_6 & 0x07ffffff) << 27) +
                  (m_18_3 >> 27) + m_18_4 + ((m_18_5 & 0x07ffffff) << 27) +
                  (m_19_2 >> 27) + m_19_3 + ((m_19_4 & 0x07ffffff) << 27) + c10;
        let c11 = d11 >> 54;
        let d12 = (m_4_19 >> 27) +
                  (m_5_18 >> 27) + m_5_19 +
                  (m_6_17 >> 27) + m_6_18 + ((m_6_19 & 0x07ffffff) << 27) +
                  (m_7_16 >> 27) + m_7_17 + ((m_7_18 & 0x07ffffff) << 27) +
                  (m_8_15 >> 27) + m_8_16 + ((m_8_17 & 0x07ffffff) << 27) +
                  (m_9_14 >> 27) + m_9_15 + ((m_9_16 & 0x07ffffff) << 27) +
                  (m_10_13 >> 27) + m_10_14 + ((m_10_15 & 0x07ffffff) << 27) +
                  (m_11_12 >> 27) + m_11_13 + ((m_11_14 & 0x07ffffff) << 27) +
                  (m_12_11 >> 27) + m_12_12 + ((m_12_13 & 0x07ffffff) << 27) +
                  (m_13_10 >> 27) + m_13_11 + ((m_13_12 & 0x07ffffff) << 27) +
                  (m_14_9 >> 27) + m_14_10 + ((m_14_11 & 0x07ffffff) << 27) +
                  (m_15_8 >> 27) + m_15_9 + ((m_15_10 & 0x07ffffff) << 27) +
                  (m_16_7 >> 27) + m_16_8 + ((m_16_9 & 0x07ffffff) << 27) +
                  (m_17_6 >> 27) + m_17_7 + ((m_17_8 & 0x07ffffff) << 27) +
                  (m_18_5 >> 27) + m_18_6 + ((m_18_7 & 0x07ffffff) << 27) +
                  (m_19_4 >> 27) + m_19_5 + ((m_19_6 & 0x07ffffff) << 27) + c11;
        let c12 = d12 >> 54;
        let d13 = (m_6_19 >> 27) +
                  (m_7_18 >> 27) + m_7_19 +
                  (m_8_17 >> 27) + m_8_18 + ((m_8_19 & 0x07ffffff) << 27) +
                  (m_9_16 >> 27) + m_9_17 + ((m_9_18 & 0x07ffffff) << 27) +
                  (m_10_15 >> 27) + m_10_16 + ((m_10_17 & 0x07ffffff) << 27) +
                  (m_11_14 >> 27) + m_11_15 + ((m_11_16 & 0x07ffffff) << 27) +
                  (m_12_13 >> 27) + m_12_14 + ((m_12_15 & 0x07ffffff) << 27) +
                  (m_13_12 >> 27) + m_13_13 + ((m_13_14 & 0x07ffffff) << 27) +
                  (m_14_11 >> 27) + m_14_12 + ((m_14_13 & 0x07ffffff) << 27) +
                  (m_15_10 >> 27) + m_15_11 + ((m_15_12 & 0x07ffffff) << 27) +
                  (m_16_9 >> 27) + m_16_10 + ((m_16_11 & 0x07ffffff) << 27) +
                  (m_17_8 >> 27) + m_17_9 + ((m_17_10 & 0x07ffffff) << 27) +
                  (m_18_7 >> 27) + m_18_8 + ((m_18_9 & 0x07ffffff) << 27) +
                  (m_19_6 >> 27) + m_19_7 + ((m_19_8 & 0x07ffffff) << 27) + c12;
        let c13 = d13 >> 54;
        let d14 = (m_8_19 >> 27) +
                  (m_9_18 >> 27) + m_9_19 +
                  (m_10_17 >> 27) + m_10_18 + ((m_10_19 & 0x07ffffff) << 27) +
                  (m_11_16 >> 27) + m_11_17 + ((m_11_18 & 0x07ffffff) << 27) +
                  (m_12_15 >> 27) + m_12_16 + ((m_12_17 & 0x07ffffff) << 27) +
                  (m_13_14 >> 27) + m_13_15 + ((m_13_16 & 0x07ffffff) << 27) +
                  (m_14_13 >> 27) + m_14_14 + ((m_14_15 & 0x07ffffff) << 27) +
                  (m_15_12 >> 27) + m_15_13 + ((m_15_14 & 0x07ffffff) << 27) +
                  (m_16_11 >> 27) + m_16_12 + ((m_16_13 & 0x07ffffff) << 27) +
                  (m_17_10 >> 27) + m_17_11 + ((m_17_12 & 0x07ffffff) << 27) +
                  (m_18_9 >> 27) + m_18_10 + ((m_18_11 & 0x07ffffff) << 27) +
                  (m_19_8 >> 27) + m_19_9 + ((m_19_10 & 0x07ffffff) << 27) +
                  c13;
        let c14 = d14 >> 54;
        let d15 = (m_10_19 >> 27) +
                  (m_11_18 >> 27) + m_11_19 +
                  (m_12_17 >> 27) + m_12_18 + ((m_12_19 & 0x07ffffff) << 27) +
                  (m_13_16 >> 27) + m_13_17 + ((m_13_18 & 0x07ffffff) << 27) +
                  (m_14_15 >> 27) + m_14_16 + ((m_14_17 & 0x07ffffff) << 27) +
                  (m_15_14 >> 27) + m_15_15 + ((m_15_16 & 0x07ffffff) << 27) +
                  (m_16_13 >> 27) + m_16_14 + ((m_16_15 & 0x07ffffff) << 27) +
                  (m_17_12 >> 27) + m_17_13 + ((m_17_14 & 0x07ffffff) << 27) +
                  (m_18_11 >> 27) + m_18_12 + ((m_18_13 & 0x07ffffff) << 27) +
                  (m_19_10 >> 27) + m_19_11 + ((m_19_12 & 0x07ffffff) << 27) +
                  c14;
        let c15 = d15 >> 54;
        let d16 = (m_12_19 >> 27) +
                  (m_13_18 >> 27) + m_13_19 +
                  (m_14_17 >> 27) + m_14_18 + ((m_14_19 & 0x07ffffff) << 27) +
                  (m_15_16 >> 27) + m_15_17 + ((m_15_18 & 0x07ffffff) << 27) +
                  (m_16_15 >> 27) + m_16_16 + ((m_16_17 & 0x07ffffff) << 27) +
                  (m_17_14 >> 27) + m_17_15 + ((m_17_16 & 0x07ffffff) << 27) +
                  (m_18_13 >> 27) + m_18_14 + ((m_18_15 & 0x07ffffff) << 27) +
                  (m_19_12 >> 27) + m_19_13 + ((m_19_14 & 0x07ffffff) << 27) +
                  c15;
        let c16 = d16 >> 54;
        let d17 = (m_14_19 >> 27) +
                  (m_15_18 >> 27) + m_15_19 +
                  (m_16_17 >> 27) + m_16_18 + ((m_16_19 & 0x07ffffff) << 27) +
                  (m_17_16 >> 27) + m_17_17 + ((m_17_18 & 0x07ffffff) << 27) +
                  (m_18_15 >> 27) + m_18_16 + ((m_18_17 & 0x07ffffff) << 27) +
                  (m_19_14 >> 27) + m_19_15 + ((m_19_16 & 0x07ffffff) << 27) +
                  c16;
        let c17 = d17 >> 54;
        let d18 = (m_16_19 >> 27) +
                  (m_17_18 >> 27) + m_17_19 +
                  (m_18_17 >> 27) + m_18_18 + ((m_18_19 & 0x07ffffff) << 27) +
                  (m_19_16 >> 27) + m_19_17 + ((m_19_18 & 0x07ffffff) << 27) +
                  c17;
        let c18 = d18 >> 54;
        let d19 = (m_18_19 >> 27) +
                  (m_19_18 >> 27) + m_19_19 + c18;

        // Modular reduction by a pseudo-mersenne prime of the form 2^n - c.

        // These are the n low-order bits.
        let l0_0 = d0 & 0x003fffffffffffff;
        let l1_0 = d1 & 0x003fffffffffffff;
        let l2_0 = d2 & 0x003fffffffffffff;
        let l3_0 = d3 & 0x003fffffffffffff;
        let l4_0 = d4 & 0x003fffffffffffff;
        let l5_0 = d5 & 0x003fffffffffffff;
        let l6_0 = d6 & 0x003fffffffffffff;
        let l7_0 = d7 & 0x003fffffffffffff;
        let l8_0 = d8 & 0x003fffffffffffff;
        let l9_0 = d9 & 0x00000007ffffffff;

        // Shift the high bits down into another n-bit number.
        let h0_0 = ((d9 & 0x003fffffffffffff) >> 35) |
                   ((d10 & 0x00000007ffffffff) << 19);
        let h1_0 = ((d10 & 0x003fffffffffffff) >> 35) |
                   ((d11 & 0x00000007ffffffff) << 19);
        let h2_0 = ((d11 & 0x003fffffffffffff) >> 35) |
                   ((d12 & 0x00000007ffffffff) << 19);
        let h3_0 = ((d12 & 0x003fffffffffffff) >> 35) |
                   ((d13 & 0x00000007ffffffff) << 19);
        let h4_0 = ((d13 & 0x003fffffffffffff) >> 35) |
                   ((d14 & 0x00000007ffffffff) << 19);
        let h5_0 = ((d14 & 0x003fffffffffffff) >> 35) |
                   ((d15 & 0x00000007ffffffff) << 19);
        let h6_0 = ((d15 & 0x003fffffffffffff) >> 35) |
                   ((d16 & 0x00000007ffffffff) << 19);
        let h7_0 = ((d16 & 0x003fffffffffffff) >> 35) |
                   ((d17 & 0x00000007ffffffff) << 19);
        let h8_0 = ((d17 & 0x003fffffffffffff) >> 35) |
                   ((d18 & 0x00000007ffffffff) << 19);
        let h9_0 = ((d18 & 0x003fffffffffffff) >> 35) |
                   (d19 << 19);

        // Normally, we multiply h by c, but since c = 1 here, we skip.

        // Add h and l.
        let kin_0 = h9_0 >> 35;
        let s0_0 = l0_0 + h0_0 + kin_0;
        let k0_0 = s0_0 >> 54;
        let s1_0 = l1_0 + h1_0 + k0_0;
        let k1_0 = s1_0 >> 54;
        let s2_0 = l2_0 + h2_0 + k1_0;
        let k2_0 = s2_0 >> 54;
        let s3_0 = l3_0 + h3_0 + k2_0;
        let k3_0 = s3_0 >> 54;
        let s4_0 = l4_0 + h4_0 + k3_0;
        let k4_0 = s4_0 >> 54;
        let s5_0 = l5_0 + h5_0 + k4_0;
        let k5_0 = s5_0 >> 54;
        let s6_0 = l6_0 + h6_0 + k5_0;
        let k6_0 = s6_0 >> 54;
        let s7_0 = l7_0 + h7_0 + k6_0;
        let k7_0 = s7_0 >> 54;
        let s8_0 = l8_0 + h8_0 + k7_0;
        let k8_0 = s8_0 >> 54;
        let s9_0 = l9_0 + (h9_0 & 0x00000007ffffffff) + k8_0;

        self[0] = s0_0 & 0x003fffffffffffff;
        self[1] = s1_0 & 0x003fffffffffffff;
        self[2] = s2_0 & 0x003fffffffffffff;
        self[3] = s3_0 & 0x003fffffffffffff;
        self[4] = s4_0 & 0x003fffffffffffff;
        self[5] = s5_0 & 0x003fffffffffffff;
        self[6] = s6_0 & 0x003fffffffffffff;
        self[7] = s7_0 & 0x003fffffffffffff;
        self[8] = s8_0 & 0x003fffffffffffff;
        self[9] = s9_0;
    }

    fn squared(&self) -> Self {
        let mut out = self.clone();

        out.square();

        out
    }

    fn invert(&mut self) {
        let mut sqval = self.clone();

        // Skip second digit (which is a 0).
        sqval.square();

        // All the remaining digits are 1.
        for _ in 2..521 {
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

        // All the remaining digits are 1.
        for _ in 1..520 {
            sqval.square();
            out *= &sqval;
        }

        out
    }

    fn sqrt(&self) -> Self {
        let mut out = self.clone();

        // All digits are 0 except the last.
        for _ in 0..519 {
            out.square();
        }

        out
    }

    fn small_add_assign(&mut self, rhs: i32) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3];
        let a4 = self[4];
        let a5 = self[5];
        let a6 = self[6];
        let a7 = self[7];
        let a8 = self[8];
        let a9 = self[9] & 0x00000007ffffffff;

        let b = i64::from(rhs);

        let cin = self.carry_out();
        let s0 = a0 + b + (cin * C_VAL);
        let c0 = s0 >> 54;
        let s1 = a1 + c0;
        let c1 = s1 >> 54;
        let s2 = a2 + c1;
        let c2 = s2 >> 54;
        let s3 = a3 + c2;
        let c3 = s3 >> 54;
        let s4 = a4 + c3;
        let c4 = s4 >> 54;
        let s5 = a5 + c4;
        let c5 = s5 >> 54;
        let s6 = a6 + c5;
        let c6 = s6 >> 54;
        let s7 = a7 + c6;
        let c7 = s7 >> 54;
        let s8 = a8 + c7;
        let c8 = s8 >> 54;
        let s9 = a9 + c8;

        self[0] = s0 & 0x003fffffffffffff;
        self[1] = s1 & 0x003fffffffffffff;
        self[2] = s2 & 0x003fffffffffffff;
        self[3] = s3 & 0x003fffffffffffff;
        self[4] = s4 & 0x003fffffffffffff;
        self[5] = s5 & 0x003fffffffffffff;
        self[6] = s6 & 0x003fffffffffffff;
        self[7] = s7 & 0x003fffffffffffff;
        self[8] = s8 & 0x003fffffffffffff;
        self[9] = s9;
    }

    fn small_add(&self, rhs: i32) -> Mod_e521_1 {
        let mut out = self.clone();

        out.small_add_assign(rhs);

        out
    }

    fn small_sub_assign(&mut self, rhs: i32) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3];
        let a4 = self[4];
        let a5 = self[5];
        let a6 = self[6];
        let a7 = self[7];
        let a8 = self[8];
        let a9 = self[9] & 0x00000007ffffffff;

        let b = i64::from(rhs);

        let cin = self.carry_out();
        let s0 = a0 - b + (cin * C_VAL);
        let c0 = s0 >> 54;
        let s1 = a1 + c0;
        let c1 = s1 >> 54;
        let s2 = a2 + c1;
        let c2 = s2 >> 54;
        let s3 = a3 + c2;
        let c3 = s3 >> 54;
        let s4 = a4 + c3;
        let c4 = s4 >> 54;
        let s5 = a5 + c4;
        let c5 = s5 >> 54;
        let s6 = a6 + c5;
        let c6 = s6 >> 54;
        let s7 = a7 + c6;
        let c7 = s7 >> 54;
        let s8 = a8 + c7;
        let c8 = s8 >> 54;
        let s9 = a9 + c8;

        self[0] = s0 & 0x003fffffffffffff;
        self[1] = s1 & 0x003fffffffffffff;
        self[2] = s2 & 0x003fffffffffffff;
        self[3] = s3 & 0x003fffffffffffff;
        self[4] = s4 & 0x003fffffffffffff;
        self[5] = s5 & 0x003fffffffffffff;
        self[6] = s6 & 0x003fffffffffffff;
        self[7] = s7 & 0x003fffffffffffff;
        self[8] = s8 & 0x003fffffffffffff;
        self[9] = s9;
    }

    fn small_sub(&self, rhs: i32) -> Mod_e521_1 {
        let mut out = self.clone();

        out.small_sub_assign(rhs);

        out
    }

    fn small_mul_assign(&mut self, rhs: i32) {
        let a0 = self[0] & 0x07ffffff;
        let a1 = self[0] >> 27;
        let a2 = self[1] & 0x07ffffff;
        let a3 = self[1] >> 27;
        let a4 = self[2] & 0x07ffffff;
        let a5 = self[2] >> 27;
        let a6 = self[3] & 0x07ffffff;
        let a7 = self[3] >> 27;
        let a8 = self[4] & 0x07ffffff;
        let a9 = self[4] >> 27;
        let a10 = self[5] & 0x07ffffff;
        let a11 = self[5] >> 27;
        let a12 = self[6] & 0x07ffffff;
        let a13 = self[6] >> 27;
        let a14 = self[7] & 0x07ffffff;
        let a15 = self[7] >> 27;
        let a16 = self[8] & 0x07ffffff;
        let a17 = self[8] >> 27;
        let a18 = self[9] & 0x07ffffff;
        let a19 = self[9] >> 27;

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
        let m14 = a14 * b;
        let m15 = a15 * b;
        let m16 = a16 * b;
        let m17 = a17 * b;
        let m18 = a18 * b;
        let m19 = a19 * b;

        let cin = self.carry_out();
        let d0 = m0 + ((m1 & 0x07ffffff) << 27) + (cin * C_VAL);
        let c0 = d0 >> 54;
        let d1 = (m1 >> 27) + m2 + ((m3 & 0x07ffffff) << 27) + c0;
        let c1 = d1 >> 54;
        let d2 = (m3 >> 27) + m4 + ((m5 & 0x07ffffff) << 27) + c1;
        let c2 = d2 >> 54;
        let d3 = (m5 >> 27) + m6 + ((m7 & 0x07ffffff) << 27) + c2;
        let c3 = d3 >> 54;
        let d4 = (m7 >> 27) + m8 + ((m9 & 0x07ffffff) << 27) + c3;
        let c4 = d4 >> 54;
        let d5 = (m9 >> 27) + m10 + ((m11 & 0x07ffffff) << 27) + c4;
        let c5 = d5 >> 54;
        let d6 = (m11 >> 27) + m12 + ((m13 & 0x07ffffff) << 27) + c5;
        let c6 = d5 >> 54;
        let d7 = (m13 >> 27) + m14 + ((m15 & 0x07ffffff) << 27) + c6;
        let c7 = d6 >> 54;
        let d8 = (m15 >> 27) + m16 + ((m17 & 0x07ffffff) << 27) + c7;
        let c8 = d7 >> 54;
        let d9 = (m17 >> 27) + m18 + (m19 << 27) + c8;

        self[0] = d0 & 0x003fffffffffffff;
        self[1] = d1 & 0x003fffffffffffff;
        self[2] = d2 & 0x003fffffffffffff;
        self[3] = d3 & 0x003fffffffffffff;
        self[4] = d4 & 0x003fffffffffffff;
        self[5] = d5 & 0x003fffffffffffff;
        self[6] = d6 & 0x003fffffffffffff;
        self[7] = d7 & 0x003fffffffffffff;
        self[8] = d8 & 0x003fffffffffffff;
        self[9] = d9;
    }

    fn small_mul(&self, b: i32) -> Mod_e521_1 {
        let mut out = self.clone();

        out.small_mul_assign(b);

        out
    }
}

#[cfg(test)]
mod tests {
    use field::prime_field::*;
    use field::mod_e521_1::*;

    const TWO: Mod_e521_1 = Mod_e521_1([ 2, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    const M_TWO: Mod_e521_1 =
        Mod_e521_1([ 0x003ffffffffffffd, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x00000007ffffffff ]);

    const THREE: Mod_e521_1 = Mod_e521_1([ 3, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    const M_THREE: Mod_e521_1 =
        Mod_e521_1([ 0x003ffffffffffffc, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x00000007ffffffff ]);

    const FOUR: Mod_e521_1 = Mod_e521_1([ 4, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    const M_FOUR: Mod_e521_1 =
        Mod_e521_1([ 0x003ffffffffffffb, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x00000007ffffffff ]);

    const SIX: Mod_e521_1 = Mod_e521_1([ 6, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    const M_SIX: Mod_e521_1 =
        Mod_e521_1([ 0x003ffffffffffff9, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x00000007ffffffff ]);

    const EIGHT: Mod_e521_1 = Mod_e521_1([ 8, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    const M_EIGHT: Mod_e521_1 =
        Mod_e521_1([ 0x003ffffffffffff7, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x00000007ffffffff ]);

    const NINE: Mod_e521_1 = Mod_e521_1([ 9, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    const M_NINE: Mod_e521_1 =
        Mod_e521_1([ 0x003ffffffffffff6, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x00000007ffffffff ]);

    const SIXTEEN: Mod_e521_1 = Mod_e521_1([ 16, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    const M_SIXTEEN: Mod_e521_1 =
        Mod_e521_1([ 0x003fffffffffffef, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x003fffffffffffff,
                     0x003fffffffffffff, 0x00000007ffffffff ]);

    fn test_pack_unpack(expected: &[u8; 66]) {
        let mut unpacked = Mod_e521_1::unpacked(expected);
        let actual = unpacked.packed();

        for i in 0..66 {
            assert!(expected[i] == actual[i]);
        }
    }

    fn test_unpack_pack(expected: &mut Mod_e521_1) {
        let bytes = expected.packed();
        let actual = Mod_e521_1::unpacked(&bytes);

        for i in 0..10 {
            assert!(expected[i] == actual[i]);
        }
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
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00, 0xff, 0x00,
                           0xff, 0x00]);
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
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0x01]);
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
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0x00]);
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
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x55, 0xaa, 0x55,
                           0xaa, 0x01]);
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
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0xaa, 0x00, 0xaa,
                           0x00, 0x00]);
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
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x00, 0xaa, 0x00,
                           0xaa, 0x01]);
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
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0xff, 0x55, 0xff,
                           0x55, 0x00]);
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
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x55, 0xff, 0x55,
                           0xff, 0x01]);
    }

    #[test]
    fn unpack_pack_test() {
        test_unpack_pack(&mut ZERO.clone());
        test_unpack_pack(&mut ONE.clone());
        test_unpack_pack(&mut M_ONE.clone());
        test_unpack_pack(&mut Mod_e521_1([ 0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x003fffffffffffff,
                                           0x0000000000000000,
                                           0x00000007ffffffff ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000555555555 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x002aaaaaaaaaaaaa,
                                           0x0055555555555555,
                                           0x00000002aaaaaaaa ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000555555555 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x002aaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x00000002aaaaaaaa ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0000000555555555 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x007fffffffffffff,
                                           0x0055555555555555,
                                           0x00000007ffffffff ]));
    }

    #[test]
    fn test_add() {
        let l1_zeros: [&mut Mod_e521_1; 5] = [ &mut (&ZERO + &ZERO),
                                               &mut (&M_ONE + &ONE),
                                               &mut (&ONE + &M_ONE),
                                               &mut (&M_TWO + &TWO),
                                               &mut (&TWO + &M_TWO) ];

        let l1_ones: [&mut Mod_e521_1; 4] = [ &mut (&ZERO + &ONE),
                                              &mut (&ONE + &ZERO),
                                              &mut (&M_ONE + &TWO),
                                              &mut (&TWO + &M_ONE) ];

        let l1_twos: [&mut Mod_e521_1; 3] = [ &mut (&ZERO + &TWO),
                                              &mut (&ONE + &ONE),
                                              &mut (&TWO + &ZERO) ];

        let l1_mones: [&mut Mod_e521_1; 4] = [ &mut (&ZERO + &M_ONE),
                                               &mut (&M_ONE + &ZERO),
                                               &mut (&M_TWO + &ONE),
                                               &mut (&ONE + &M_TWO) ];

        let l1_mtwos: [&mut Mod_e521_1; 3] = [ &mut (&ZERO + &M_TWO),
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
        let l1_zeros: [&mut Mod_e521_1; 3] = [ &mut (&ZERO - &ZERO),
                                               &mut (&ONE - &ONE),
                                               &mut (&TWO - &TWO) ];

        let l1_ones: [&mut Mod_e521_1; 4] = [ &mut (&ZERO - &M_ONE),
                                              &mut (&ONE - &ZERO),
                                              &mut (&M_ONE - &M_TWO),
                                              &mut (&TWO - &ONE) ];

        let l1_twos: [&mut Mod_e521_1; 3] = [ &mut (&ZERO - &M_TWO),
                                              &mut (&ONE - &M_ONE),
                                              &mut (&TWO - &ZERO) ];

        let l1_mones: [&mut Mod_e521_1; 4] = [ &mut (&ZERO - &ONE),
                                               &mut (&M_ONE - &ZERO),
                                               &mut (&M_TWO - &M_ONE),
                                               &mut (&ONE - &TWO) ];

        let l1_mtwos: [&mut Mod_e521_1; 3] = [ &mut (&ZERO - &TWO),
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
        let l1_zeros: [&mut Mod_e521_1; 9] = [ &mut (&ZERO * &ZERO),
                                               &mut (&ONE * &ZERO),
                                               &mut (&TWO * &ZERO),
                                               &mut (&M_ONE * &ZERO),
                                               &mut (&M_TWO * &ZERO),
                                               &mut (&ZERO * &ONE),
                                               &mut (&ZERO * &TWO),
                                               &mut (&ZERO * &M_ONE),
                                               &mut (&ZERO * &M_TWO) ];

        let l1_ones: [&mut Mod_e521_1; 2] = [ &mut (&ONE * &ONE),
                                              &mut (&M_ONE * &M_ONE) ];

        let l1_twos: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &TWO),
                                              &mut (&TWO * &ONE),
                                              &mut (&M_ONE * &M_TWO),
                                              &mut (&M_TWO * &M_ONE) ];

        let l1_fours: [&mut Mod_e521_1; 2] = [ &mut (&TWO * &TWO),
                                               &mut (&M_TWO * &M_TWO) ];

        let l1_mones: [&mut Mod_e521_1; 2] = [ &mut (&ONE * &M_ONE),
                                               &mut (&M_ONE * &ONE) ];

        let l1_mtwos: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &M_TWO),
                                               &mut (&TWO * &M_ONE),
                                               &mut (&M_ONE * &TWO),
                                               &mut (&M_TWO * &ONE) ];

        let l1_mfours: [&mut Mod_e521_1; 2] = [ &mut (&TWO * &M_TWO),
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
        let l1_zeros: [&mut Mod_e521_1; 10] = [ &mut (&ZERO * &ZERO),
                                                &mut (&ONE * &ZERO),
                                                &mut (&TWO * &ZERO),
                                                &mut (&M_ONE * &ZERO),
                                                &mut (&M_TWO * &ZERO),
                                                &mut (&ZERO * &ONE),
                                                &mut (&ZERO * &TWO),
                                                &mut (&ZERO * &M_ONE),
                                                &mut (&ZERO * &M_TWO),
                                                &mut ZERO.squared() ];

        let l1_ones: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &ONE),
                                              &mut (&M_ONE * &M_ONE),
                                              &mut ONE.squared(),
                                              &mut M_ONE.squared() ];

        let l1_twos: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &TWO),
                                              &mut (&TWO * &ONE),
                                              &mut (&M_ONE * &M_TWO),
                                              &mut (&M_TWO * &M_ONE) ];

        let l1_threes: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &THREE),
                                                &mut (&THREE * &ONE),
                                                &mut (&M_ONE * &M_THREE),
                                                &mut (&M_THREE * &M_ONE) ];

        let l1_fours: [&mut Mod_e521_1; 4] = [ &mut (&TWO * &TWO),
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
        let l1_ones: [&mut Mod_e521_1; 2] = [ &mut (&ONE * &ONE),
                                              &mut (&M_ONE * &M_ONE) ];

        let l1_twos: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &TWO),
                                              &mut (&TWO * &ONE),
                                              &mut (&M_ONE * &M_TWO),
                                              &mut (&M_TWO * &M_ONE) ];

        let l1_threes: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &THREE),
                                                &mut (&THREE * &ONE),
                                                &mut (&M_ONE * &M_THREE),
                                                &mut (&M_THREE * &M_ONE) ];

        let l1_fours: [&mut Mod_e521_1; 2] = [ &mut (&TWO * &TWO),
                                               &mut (&M_TWO * &M_TWO) ];

        let l1_mones: [&mut Mod_e521_1; 2] = [ &mut (&ONE * &M_ONE),
                                               &mut (&M_ONE * &ONE) ];

        let l1_mtwos: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &M_TWO),
                                               &mut (&TWO * &M_ONE),
                                               &mut (&M_ONE * &TWO),
                                               &mut (&M_TWO * &ONE) ];

        let l1_mthrees: [&mut Mod_e521_1; 4] = [ &mut (&ONE * &M_THREE),
                                                 &mut (&THREE * &M_ONE),
                                                 &mut (&M_ONE * &THREE),
                                                 &mut (&M_THREE * &ONE) ];

        let l1_mfours: [&mut Mod_e521_1; 2] = [ &mut (&TWO * &M_TWO),
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
        let l1_ones: [&mut Mod_e521_1; 12] = [ &mut (&ONE / &ONE),
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

        let l1_twos: [&mut Mod_e521_1; 10] = [ &mut (&TWO / &ONE),
                                               &mut (&M_TWO / &M_ONE),
                                               &mut (&FOUR / &TWO),
                                               &mut (&M_FOUR / &M_TWO),
                                               &mut (&SIX / &THREE),
                                               &mut (&M_SIX / &M_THREE),
                                               &mut (&EIGHT / &FOUR),
                                               &mut (&M_EIGHT / &M_FOUR),
                                               &mut (&SIXTEEN / &EIGHT),
                                               &mut (&M_SIXTEEN / &M_EIGHT) ];

        let l1_threes: [&mut Mod_e521_1; 6] = [ &mut (&THREE / &ONE),
                                                &mut (&M_THREE / &M_ONE),
                                                &mut (&SIX / &TWO),
                                                &mut (&M_SIX / &M_TWO),
                                                &mut (&NINE / &THREE),
                                                &mut (&M_NINE / &M_THREE) ];

        let l1_fours: [&mut Mod_e521_1; 6] = [ &mut (&FOUR / &ONE),
                                               &mut (&M_FOUR / &M_ONE),
                                               &mut (&EIGHT / &TWO),
                                               &mut (&M_EIGHT / &M_TWO),
                                               &mut (&SIXTEEN / &FOUR),
                                               &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e521_1; 12] = [ &mut (&ONE / &M_ONE),
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

        let l1_mtwos: [&mut Mod_e521_1; 10] = [ &mut (&TWO / &M_ONE),
                                                &mut (&M_TWO / &ONE),
                                                &mut (&FOUR / &M_TWO),
                                                &mut (&M_FOUR / &TWO),
                                                &mut (&SIX / &M_THREE),
                                                &mut (&M_SIX / &THREE),
                                                &mut (&EIGHT / &M_FOUR),
                                                &mut (&M_EIGHT / &FOUR),
                                                &mut (&SIXTEEN / &M_EIGHT),
                                                &mut (&M_SIXTEEN / &EIGHT) ];

        let l1_mthrees: [&mut Mod_e521_1; 6] = [ &mut (&THREE / &M_ONE),
                                                 &mut (&M_THREE / &ONE),
                                                 &mut (&SIX / &M_TWO),
                                                 &mut (&M_SIX / &TWO),
                                                 &mut (&NINE / &M_THREE),
                                                 &mut (&M_NINE / &THREE) ];

        let l1_mfours: [&mut Mod_e521_1; 6] = [ &mut (&FOUR / &M_ONE),
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
        let l1_zeros: [&mut Mod_e521_1; 10] = [ &mut (&ZERO * &ZERO),
                                                  &mut (&ONE * &ZERO),
                                                  &mut (&TWO * &ZERO),
                                                  &mut (&M_ONE * &ZERO),
                                                  &mut (&M_TWO * &ZERO),
                                                  &mut (&ZERO * &ONE),
                                                  &mut (&ZERO * &TWO),
                                                  &mut (&ZERO * &M_ONE),
                                                  &mut (&ZERO * &M_TWO),
                                                  &mut ZERO.squared() ];

        let l1_ones: [&mut Mod_e521_1; 12] = [ &mut (&ONE / &ONE),
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

        let l1_twos: [&mut Mod_e521_1; 10] = [ &mut (&TWO / &ONE),
                                                 &mut (&M_TWO / &M_ONE),
                                                 &mut (&FOUR / &TWO),
                                                 &mut (&M_FOUR / &M_TWO),
                                                 &mut (&SIX / &THREE),
                                                 &mut (&M_SIX / &M_THREE),
                                                 &mut (&EIGHT / &FOUR),
                                                 &mut (&M_EIGHT / &M_FOUR),
                                                 &mut (&SIXTEEN / &EIGHT),
                                                 &mut (&M_SIXTEEN / &M_EIGHT) ];

        let l1_threes: [&mut Mod_e521_1; 6] = [ &mut (&THREE / &ONE),
                                                  &mut (&M_THREE / &M_ONE),
                                                  &mut (&SIX / &TWO),
                                                  &mut (&M_SIX / &M_TWO),
                                                  &mut (&NINE / &THREE),
                                                  &mut (&M_NINE / &M_THREE) ];

        let l1_fours: [&mut Mod_e521_1; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e521_1; 12] = [ &mut (&ONE / &M_ONE),
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

        let l1_mtwos: [&mut Mod_e521_1; 10] = [ &mut (&TWO / &M_ONE),
                                                  &mut (&M_TWO / &ONE),
                                                  &mut (&FOUR / &M_TWO),
                                                  &mut (&M_FOUR / &TWO),
                                                  &mut (&SIX / &M_THREE),
                                                  &mut (&M_SIX / &THREE),
                                                  &mut (&EIGHT / &M_FOUR),
                                                  &mut (&M_EIGHT / &FOUR),
                                                  &mut (&SIXTEEN / &M_EIGHT),
                                                  &mut (&M_SIXTEEN / &EIGHT) ];

        let l1_mthrees: [&mut Mod_e521_1; 6] = [ &mut (&THREE / &M_ONE),
                                                   &mut (&M_THREE / &ONE),
                                                   &mut (&SIX / &M_TWO),
                                                   &mut (&M_SIX / &TWO),
                                                   &mut (&NINE / &M_THREE),
                                                   &mut (&M_NINE / &THREE) ];

        let l1_mfours: [&mut Mod_e521_1; 6] = [ &mut (&FOUR / &M_ONE),
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

            println!("{:?}", val);
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
        let l1_twos: [&mut Mod_e521_1; 10] = [ &mut (&TWO / &ONE),
                                                 &mut (&M_TWO / &M_ONE),
                                                 &mut (&FOUR / &TWO),
                                                 &mut (&M_FOUR / &M_TWO),
                                                 &mut (&SIX / &THREE),
                                                 &mut (&M_SIX / &M_THREE),
                                                 &mut (&EIGHT / &FOUR),
                                                 &mut (&M_EIGHT / &M_FOUR),
                                                 &mut (&SIXTEEN / &EIGHT),
                                                 &mut (&M_SIXTEEN / &M_EIGHT) ];

        let l1_fours: [&mut Mod_e521_1; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];


        let l1_mthrees: [&mut Mod_e521_1; 6] = [ &mut (&THREE / &M_ONE),
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
        let l1_zeros: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_add(0),
                                               &mut M_ONE.small_add(1),
                                               &mut ONE.small_add(-1),
                                               &mut M_TWO.small_add(2),
                                               &mut TWO.small_add(-2) ];

        let l1_ones: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_add(1),
                                              &mut M_ONE.small_add(2),
                                              &mut ONE.small_add(0),
                                              &mut M_TWO.small_add(3),
                                              &mut TWO.small_add(-1) ];

        let l1_twos: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_add(2),
                                              &mut ONE.small_add(1),
                                              &mut M_ONE.small_add(3),
                                              &mut TWO.small_add(0),
                                              &mut M_TWO.small_add(4) ];

        let l1_mones: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_add(-1),
                                               &mut M_ONE.small_add(0),
                                               &mut ONE.small_add(-2),
                                               &mut M_TWO.small_add(1),
                                               &mut TWO.small_add(-3) ];

        let l1_mtwos: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_add(-2),
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
        let l1_zeros: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_sub(0),
                                               &mut M_ONE.small_sub(-1),
                                               &mut ONE.small_sub(1),
                                               &mut M_TWO.small_sub(-2),
                                               &mut TWO.small_sub(2) ];

        let l1_ones: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_sub(-1),
                                              &mut M_ONE.small_sub(-2),
                                              &mut ONE.small_sub(0),
                                              &mut M_TWO.small_sub(-3),
                                              &mut TWO.small_sub(1) ];

        let l1_twos: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_sub(-2),
                                              &mut ONE.small_sub(-1),
                                              &mut M_ONE.small_sub(-3),
                                              &mut TWO.small_sub(0),
                                              &mut M_TWO.small_sub(-4) ];

        let l1_mones: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_sub(1),
                                               &mut M_ONE.small_sub(0),
                                               &mut ONE.small_sub(2),
                                               &mut M_TWO.small_sub(-1),
                                               &mut TWO.small_sub(3) ];

        let l1_mtwos: [&mut Mod_e521_1; 5] = [ &mut ZERO.small_sub(2),
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

        let l1_zeros: [&mut Mod_e521_1; 9] = [ &mut ZERO.small_mul(0),
                                               &mut ONE.small_mul(0),
                                               &mut TWO.small_mul(0),
                                               &mut M_ONE.small_mul(0),
                                               &mut M_TWO.small_mul(0),
                                               &mut ZERO.small_mul(1),
                                               &mut ZERO.small_mul(2),
                                               &mut ZERO.small_mul(-1),
                                               &mut ZERO.small_mul(-2) ];

        let l1_ones: [&mut Mod_e521_1; 2] = [ &mut ONE.small_mul(1),
                                              &mut M_ONE.small_mul(-1) ];

        let l1_twos: [&mut Mod_e521_1; 4] = [ &mut ONE.small_mul(2),
                                              &mut TWO.small_mul(1),
                                              &mut M_ONE.small_mul(-2),
                                              &mut M_TWO.small_mul(-1) ];

        let l1_fours: [&mut Mod_e521_1; 2] = [ &mut TWO.small_mul(2),
                                               &mut M_TWO.small_mul(-2) ];

        let l1_mones: [&mut Mod_e521_1; 2] = [ &mut ONE.small_mul(-1),
                                               &mut M_ONE.small_mul(1) ];

        let l1_mtwos: [&mut Mod_e521_1; 4] = [ &mut ONE.small_mul(-2),
                                               &mut TWO.small_mul(-1),
                                               &mut M_ONE.small_mul(2),
                                               &mut M_TWO.small_mul(1) ];

        let l1_mfours: [&mut Mod_e521_1; 2] = [ &mut TWO.small_mul(-2),
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
