use field::prime_field::PrimeField;
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

/// Elements of the finite field of integers mod 2^221 - 3.  Used by
/// the M-221 curve.
///
/// This is represented using eight 29-bit digits, stored in a
/// four-element i64 array with two digits per word.  This combined
/// representation allows many operations to be faster.  The leftover
/// bits in each digit are used to capture carry values.  The internal
/// representation is lazily normalized: it may leave carry values in
/// the highest-order digit, and it may hold a value greater than the
/// modulus.  All operations are guaranteed to work on non-normal values
/// of this kind.

#[derive(Copy, Clone)]
pub struct Mod_e221_3([i64; 4]);

const C_VAL: i64 = 3;

/// The normalized representation of the value 0.
pub const ZERO: Mod_e221_3 = Mod_e221_3([ 0, 0, 0, 0 ]);

/// The normalized representation of the value 1.
pub const ONE: Mod_e221_3 = Mod_e221_3([ 1, 0, 0, 0 ]);

/// The normalized representation of the value -1.
pub const M_ONE: Mod_e221_3 =
    Mod_e221_3([ 0x03fffffffffffffc, 0x03ffffffffffffff,
                 0x03ffffffffffffff, 0x00007fffffffffff ]);

/// The normalized representation of the modulus 2^221 - 3.
pub const MODULUS: Mod_e221_3 =
    Mod_e221_3([ 0x03fffffffffffffd, 0x03ffffffffffffff,
                 0x03ffffffffffffff, 0x00007fffffffffff ]);

/// The normalized representation of the value -1/2.
pub const M_HALF: Mod_e221_3 =
    Mod_e221_3([ 0x03fffffffffffffe, 0x03ffffffffffffff,
                 0x03ffffffffffffff, 0x00003fffffffffff ]);

const COEFF: Mod_e221_3 =
    Mod_e221_3([ 0x02b158a371015617, 0x0393918499ec8f01,
                 0x00dd6e73800ee6c4, 0x00002301b5e2218f ]);

impl Debug for Mod_e221_3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "Mod_e221_3: [ {:x}", &self[0]));

        for i in 1..4 {
            try!(write!(f, ", {:x}", &self[i]));
        }

        write!(f, " ]")
    }
}

impl LowerHex for Mod_e221_3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.packed();

        for i in 0..28 {
            try!(write!(f, "{:02x}", bytes[27 - i]));
        }

        Ok(())
    }
}

impl UpperHex for Mod_e221_3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.packed();

        for i in 0..28 {
            try!(write!(f, "{:02X}", bytes[27 - i]));
        }

        Ok(())
    }
}

impl Mod_e221_3 {
    /// Get the carry-in value.  We use the highest carry slot to
    /// stash the carry-out value of each operation, and feed that
    /// back into the next one.
    fn carry_out(&self) -> i64 {
        self[3] >> 47
    }

    /// Serialize a value as a little-endian byte array.  This has the
    /// effect of normalizing the representation.
    pub fn packed(&mut self) -> [u8; 28] {
        let mut out = [0u8; 28];
        self.pack(&mut out);
        out
    }

    /// Serialize an already normalized number as a little-endian byte
    /// array.  This must only be used on a normalized value.
    fn packed_normalized(&self) -> [u8; 28] {
        let mut out = [0u8; 28];
        self.pack_normalized(&mut out);
        out
    }

    /// Similar to the Legendre symbol, but for quartic
    /// residues/non-residues.
    fn quartic_legendre(&self) -> Self {
        // First digit is 1.
        let mut out = self.clone();
        let mut sqval = out.clone();

        // All the remaining digits are 1.
        for _ in 1..219 {
            sqval.square();
            out *= &sqval;
        }

        out
    }
}

impl IndexMut<usize> for Mod_e221_3 {
    fn index_mut<'a>(&'a mut self, idx : usize) -> &'a mut i64 {
        let ret : &'a mut i64 = &mut(self.0[idx]);
        ret
    }
}

impl Index<usize> for Mod_e221_3 {
    type Output = i64;

    fn index<'a>(&'a self, idx : usize) -> &'a i64 {
        let ret : &'a i64 = &(self.0[idx]);
        ret
    }
}

impl Neg for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn neg(self) -> Mod_e221_3 {
        let mut out = self;

        out += &MODULUS;
        out
    }
}

impl<'b> AddAssign<&'b Mod_e221_3> for Mod_e221_3 {
    fn add_assign(&mut self, rhs: &'b Mod_e221_3) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3] & 0x00007fffffffffff;

        let b0 = rhs[0];
        let b1 = rhs[1];
        let b2 = rhs[2];
        let b3 = rhs[3] & 0x00007fffffffffff;

        let cin = self.carry_out() + rhs.carry_out();
        let s0 = a0 + b0 + (cin * C_VAL);
        let c0 = s0 >> 58;
        let s1 = a1 + b1 + c0;
        let c1 = s1 >> 58;
        let s2 = a2 + b2 + c1;
        let c2 = s2 >> 58;
        let s3 = a3 + b3 + c2;

        self[0] = s0 & 0x03ffffffffffffff;
        self[1] = s1 & 0x03ffffffffffffff;
        self[2] = s2 & 0x03ffffffffffffff;
        self[3] = s3;
    }
}

impl AddAssign<Mod_e221_3> for Mod_e221_3 {
    fn add_assign(&mut self, rhs: Mod_e221_3) {
        *self += &rhs;
    }
}

impl AddAssign<i32> for Mod_e221_3 {
    fn add_assign(&mut self, rhs: i32) {
        self.small_add_assign(rhs);
    }
}

impl AddAssign<i16> for Mod_e221_3 {
    fn add_assign(&mut self, rhs: i16) {
        self.small_add_assign(rhs as i32);
    }
}

impl AddAssign<i8> for Mod_e221_3 {
    fn add_assign(&mut self, rhs: i8) {
        self.small_add_assign(rhs as i32);
    }
}

impl<'a, 'b> Add<&'b Mod_e221_3> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: &'b Mod_e221_3) -> Mod_e221_3 {
        let mut out = self.clone();
        out += a;
        out
    }
}

impl Add<Mod_e221_3> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: Mod_e221_3) -> Mod_e221_3 {
        &self + &a
    }
}

impl<'a> Add<&'a Mod_e221_3> for i32 {
    type Output = Mod_e221_3;

    fn add(self, a: &'a Mod_e221_3) -> Mod_e221_3 {
        a.small_add(self)
    }
}

impl<'a> Add<&'a Mod_e221_3> for i16 {
    type Output = Mod_e221_3;

    fn add(self, a: &'a Mod_e221_3) -> Mod_e221_3 {
        a.small_add(self as i32)
    }
}

impl<'a> Add<&'a Mod_e221_3> for i8 {
    type Output = Mod_e221_3;

    fn add(self, a: &'a Mod_e221_3) -> Mod_e221_3 {
        a.small_add(self as i32)
    }
}

impl<'a> Add<i32> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: i32) -> Mod_e221_3 {
        self.small_add(a)
    }
}

impl<'a> Add<i16> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: i16) -> Mod_e221_3 {
        self.small_add(a as i32)
    }
}

impl<'a> Add<i8> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: i8) -> Mod_e221_3 {
        self.small_add(a as i32)
    }
}

impl Add<i32> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: i32) -> Mod_e221_3 {
        &self + a
    }
}

impl Add<i16> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: i16) -> Mod_e221_3 {
        &self + a
    }
}

impl Add<i8> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn add(self, a: i8) -> Mod_e221_3 {
        &self + a
    }
}

impl<'b> DivAssign<&'b Mod_e221_3> for Mod_e221_3 {
    fn div_assign(&mut self, rhs: &'b Mod_e221_3) {
        *self *= &rhs.inverted();
    }
}

impl DivAssign<Mod_e221_3> for Mod_e221_3 {
    fn div_assign(&mut self, rhs: Mod_e221_3) {
        *self /= &rhs;
    }
}

impl<'a, 'b> Div<&'b Mod_e221_3> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn div(self, a: &'b Mod_e221_3) -> Mod_e221_3 {
        let mut out = self.clone();
        out /= a;
        out
    }
}

impl Div<Mod_e221_3> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn div(self, a: Mod_e221_3) -> Mod_e221_3 {
        &self / &a
    }
}

impl SubAssign<i32> for Mod_e221_3 {
    fn sub_assign(&mut self, rhs: i32) {
        self.small_sub_assign(rhs);
    }
}

impl SubAssign<i16> for Mod_e221_3 {
    fn sub_assign(&mut self, rhs: i16) {
        self.small_sub_assign(rhs as i32);
    }
}

impl SubAssign<i8> for Mod_e221_3 {
    fn sub_assign(&mut self, rhs: i8) {
        self.small_sub_assign(rhs as i32);
    }
}

impl<'b> SubAssign<&'b Mod_e221_3> for Mod_e221_3 {
    fn sub_assign(&mut self, rhs: &'b Mod_e221_3) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3] & 0x00007fffffffffff;

        let b0 = rhs[0];
        let b1 = rhs[1];
        let b2 = rhs[2];
        let b3 = rhs[3] & 0x00007fffffffffff;

        let cin = self.carry_out() + rhs.carry_out();
        let s0 = a0 - b0 + (cin * C_VAL);
        let c0 = s0 >> 58;
        let s1 = a1 - b1 + c0;
        let c1 = s1 >> 58;
        let s2 = a2 - b2 + c1;
        let c2 = s2 >> 58;
        let s3 = a3 - b3 + c2;

        self[0] = s0 & 0x03ffffffffffffff;
        self[1] = s1 & 0x03ffffffffffffff;
        self[2] = s2 & 0x03ffffffffffffff;
        self[3] = s3;
    }
}

impl SubAssign<Mod_e221_3> for Mod_e221_3 {
    fn sub_assign(&mut self, rhs: Mod_e221_3) {
        *self -= &rhs
    }
}

impl<'a> Sub<i32> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: i32) -> Mod_e221_3 {
        self.small_sub(a)
    }
}

impl<'a> Sub<i16> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: i16) -> Mod_e221_3 {
        self.small_sub(a as i32)
    }
}

impl<'a> Sub<i8> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: i8) -> Mod_e221_3 {
        self.small_sub(a as i32)
    }
}

impl Sub<i32> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: i32) -> Mod_e221_3 {
        &self - a
    }
}

impl Sub<i16> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: i16) -> Mod_e221_3 {
        &self - a
    }
}

impl Sub<i8> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: i8) -> Mod_e221_3 {
        &self - a
    }
}

impl<'a, 'b> Sub<&'b Mod_e221_3> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: &'b Mod_e221_3) -> Mod_e221_3 {
        let mut out = self.clone();
        out -= a;
        out
    }
}

impl Sub<Mod_e221_3> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn sub(self, a: Mod_e221_3) -> Mod_e221_3 {
        &self - &a
    }
}

impl MulAssign<i32> for Mod_e221_3 {
    fn mul_assign(&mut self, rhs: i32) {
        self.small_mul_assign(rhs);
    }
}

impl MulAssign<i16> for Mod_e221_3 {
    fn mul_assign(&mut self, rhs: i16) {
        self.small_mul_assign(rhs as i32);
    }
}

impl MulAssign<i8> for Mod_e221_3 {
    fn mul_assign(&mut self, rhs: i8) {
        self.small_mul_assign(rhs as i32);
    }
}

impl<'b> MulAssign<&'b Mod_e221_3> for Mod_e221_3 {
    fn mul_assign(&mut self, rhs: &'b Mod_e221_3) {
        // Expand out to single digits
        let a0 = self[0] & 0x1fffffff;
        let a1 = self[0] >> 29;
        let a2 = self[1] & 0x1fffffff;
        let a3 = self[1] >> 29;
        let a4 = self[2] & 0x1fffffff;
        let a5 = self[2] >> 29;
        let a6 = self[3] & 0x1fffffff;
        let a7 = self[3] >> 29;

        let b0 = rhs[0] & 0x1fffffff;
        let b1 = rhs[0] >> 29;
        let b2 = rhs[1] & 0x1fffffff;
        let b3 = rhs[1] >> 29;
        let b4 = rhs[2] & 0x1fffffff;
        let b5 = rhs[2] >> 29;
        let b6 = rhs[3] & 0x1fffffff;
        let b7 = rhs[3] >> 29;

        // Combined multiples
        let m_0_0 = a0 * b0;
        let m_0_1 = a0 * b1;
        let m_0_2 = a0 * b2;
        let m_0_3 = a0 * b3;
        let m_0_4 = a0 * b4;
        let m_0_5 = a0 * b5;
        let m_0_6 = a0 * b6;
        let m_0_7 = a0 * b7;
        let m_1_0 = a1 * b0;
        let m_1_1 = a1 * b1;
        let m_1_2 = a1 * b2;
        let m_1_3 = a1 * b3;
        let m_1_4 = a1 * b4;
        let m_1_5 = a1 * b5;
        let m_1_6 = a1 * b6;
        let m_1_7 = a1 * b7;
        let m_2_0 = a2 * b0;
        let m_2_1 = a2 * b1;
        let m_2_2 = a2 * b2;
        let m_2_3 = a2 * b3;
        let m_2_4 = a2 * b4;
        let m_2_5 = a2 * b5;
        let m_2_6 = a2 * b6;
        let m_2_7 = a2 * b7;
        let m_3_0 = a3 * b0;
        let m_3_1 = a3 * b1;
        let m_3_2 = a3 * b2;
        let m_3_3 = a3 * b3;
        let m_3_4 = a3 * b4;
        let m_3_5 = a3 * b5;
        let m_3_6 = a3 * b6;
        let m_3_7 = a3 * b7;
        let m_4_0 = a4 * b0;
        let m_4_1 = a4 * b1;
        let m_4_2 = a4 * b2;
        let m_4_3 = a4 * b3;
        let m_4_4 = a4 * b4;
        let m_4_5 = a4 * b5;
        let m_4_6 = a4 * b6;
        let m_4_7 = a4 * b7;
        let m_5_0 = a5 * b0;
        let m_5_1 = a5 * b1;
        let m_5_2 = a5 * b2;
        let m_5_3 = a5 * b3;
        let m_5_4 = a5 * b4;
        let m_5_5 = a5 * b5;
        let m_5_6 = a5 * b6;
        let m_5_7 = a5 * b7;
        let m_6_0 = a6 * b0;
        let m_6_1 = a6 * b1;
        let m_6_2 = a6 * b2;
        let m_6_3 = a6 * b3;
        let m_6_4 = a6 * b4;
        let m_6_5 = a6 * b5;
        let m_6_6 = a6 * b6;
        let m_6_7 = a6 * b7;
        let m_7_0 = a7 * b0;
        let m_7_1 = a7 * b1;
        let m_7_2 = a7 * b2;
        let m_7_3 = a7 * b3;
        let m_7_4 = a7 * b4;
        let m_7_5 = a7 * b5;
        let m_7_6 = a7 * b6;
        let m_7_7 = a7 * b7;

        // Compute the 16-digit combined product using 64-bit operations.
        let d0 = m_0_0 + ((m_0_1 & 0x1fffffff) << 29) +
                 ((m_1_0 & 0x1fffffff) << 29);
        let c0 = d0 >> 58;
        let d1 = (m_0_1 >> 29) + m_0_2 + ((m_0_3 & 0x1fffffff) << 29) +
                 (m_1_0 >> 29) + m_1_1 + ((m_1_2 & 0x1fffffff) << 29) +
                 m_2_0 + ((m_2_1 & 0x1fffffff) << 29) +
                 ((m_3_0 & 0x1fffffff) << 29) + c0;
        let c1 = d1 >> 58;
        let d2 = (m_0_3 >> 29) + m_0_4 + ((m_0_5 & 0x1fffffff) << 29) +
                 (m_1_2 >> 29) + m_1_3 + ((m_1_4 & 0x1fffffff) << 29) +
                 (m_2_1 >> 29) + m_2_2 + ((m_2_3 & 0x1fffffff) << 29) +
                 (m_3_0 >> 29) + m_3_1 + ((m_3_2 & 0x1fffffff) << 29) +
                 m_4_0 + ((m_4_1 & 0x1fffffff) << 29) +
                 ((m_5_0 & 0x1fffffff) << 29) + c1;
        let c2 = d2 >> 58;
        let d3 = (m_0_5 >> 29) + m_0_6 + ((m_0_7 & 0x1fffffff) << 29) +
                 (m_1_4 >> 29) + m_1_5 + ((m_1_6 & 0x1fffffff) << 29) +
                 (m_2_3 >> 29) + m_2_4 + ((m_2_5 & 0x1fffffff) << 29) +
                 (m_3_2 >> 29) + m_3_3 + ((m_3_4 & 0x1fffffff) << 29) +
                 (m_4_1 >> 29) + m_4_2 + ((m_4_3 & 0x1fffffff) << 29) +
                 (m_5_0 >> 29) + m_5_1 + ((m_5_2 & 0x1fffffff) << 29) +
                 m_6_0 + ((m_6_1 & 0x1fffffff) << 29) +
                 ((m_7_0 & 0x1fffffff) << 29) + c2;
        let c3 = d3 >> 58;
        let d4 = (m_0_7 >> 29) +
                 (m_1_6 >> 29) + m_1_7 +
                 (m_2_5 >> 29) + m_2_6 + ((m_2_7 & 0x1fffffff) << 29) +
                 (m_3_4 >> 29) + m_3_5 + ((m_3_6 & 0x1fffffff) << 29) +
                 (m_4_3 >> 29) + m_4_4 + ((m_4_5 & 0x1fffffff) << 29) +
                 (m_5_2 >> 29) + m_5_3 + ((m_5_4 & 0x1fffffff) << 29) +
                 (m_6_1 >> 29) + m_6_2 + ((m_6_3 & 0x1fffffff) << 29) +
                 (m_7_0 >> 29) + m_7_1 + ((m_7_2 & 0x1fffffff) << 29) +
                 c3;
        let c4 = d4 >> 58;
        let d5 = (m_2_7 >> 29) +
                 (m_3_6 >> 29) + m_3_7 +
                 (m_4_5 >> 29) + m_4_6 + ((m_4_7 & 0x1fffffff) << 29) +
                 (m_5_4 >> 29) + m_5_5 + ((m_5_6 & 0x1fffffff) << 29) +
                 (m_6_3 >> 29) + m_6_4 + ((m_6_5 & 0x1fffffff) << 29) +
                 (m_7_2 >> 29) + m_7_3 + ((m_7_4 & 0x1fffffff) << 29) +
                 c4;
        let c5 = d5 >> 58;
        let d6 = (m_4_7 >> 29) +
                 (m_5_6 >> 29) + m_5_7 +
                 (m_6_5 >> 29) + m_6_6 + ((m_6_7 & 0x1fffffff) << 29) +
                 (m_7_4 >> 29) + m_7_5 + ((m_7_6 & 0x1fffffff) << 29) +
                 c5;
        let c6 = d6 >> 58;
        let d7 = (m_6_7 >> 29) +
                 (m_7_6 >> 29) + m_7_7 +
                 c6;

        // Modular reduction by a pseudo-mersenne prime of the form 2^n - c.

        // These are the n low-order
        let l0_0 = d0 & 0x03ffffffffffffff;
        let l1_0 = d1 & 0x03ffffffffffffff;
        let l2_0 = d2 & 0x03ffffffffffffff;
        let l3_0 = d3 & 0x00007fffffffffff;

        // Shift the high bits down into another n-bit number.
        let h0_0 = ((d3 & 0x03ffffffffffffff) >> 47) |
                   ((d4 & 0x00007fffffffffff) << 11);
        let h1_0 = ((d4 & 0x03ffffffffffffff) >> 47) |
                   ((d5 & 0x00007fffffffffff) << 11);
        let h2_0 = ((d5 & 0x03ffffffffffffff) >> 47) |
                   ((d6 & 0x00007fffffffffff) << 11);
        let h3_0 = ((d6 & 0x03ffffffffffffff) >> 47) |
                   (d7 << 11);

        // Multiply by C
        let hc0_0 = h0_0 * C_VAL;
        let hc1_0 = h1_0 * C_VAL;
        let hc2_0 = h2_0 * C_VAL;
        let hc3_0 = h3_0 * C_VAL;

        // Add h and l.
        let kin_0 = hc3_0 >> 47;
        let s0_0 = l0_0 + hc0_0 + (kin_0 * C_VAL);
        let k0_0 = s0_0 >> 58;
        let s1_0 = l1_0 + hc1_0 + k0_0;
        let k1_0 = s1_0 >> 58;
        let s2_0 = l2_0 + hc2_0 + k1_0;
        let k2_0 = s2_0 >> 58;
        let s3_0 = l3_0 + (hc3_0 & 0x00007fffffffffff) + k2_0;

        self[0] = s0_0 & 0x03ffffffffffffff;
        self[1] = s1_0 & 0x03ffffffffffffff;
        self[2] = s2_0 & 0x03ffffffffffffff;
        self[3] = s3_0;
     }
}

impl MulAssign<Mod_e221_3> for Mod_e221_3 {
    fn mul_assign(&mut self, rhs: Mod_e221_3) {
        *self *= &rhs;
    }
}

impl<'a> Mul<&'a Mod_e221_3> for i32 {
    type Output = Mod_e221_3;

    fn mul(self, a: &'a Mod_e221_3) -> Mod_e221_3 {
        a.small_mul(self)
    }
}

impl<'a> Mul<&'a Mod_e221_3> for i16 {
    type Output = Mod_e221_3;

    fn mul(self, a: &'a Mod_e221_3) -> Mod_e221_3 {
        a.small_mul(self as i32)
    }
}

impl<'a> Mul<&'a Mod_e221_3> for i8 {
    type Output = Mod_e221_3;

    fn mul(self, a: &'a Mod_e221_3) -> Mod_e221_3 {
        a.small_mul(self as i32)
    }
}

impl<'a> Mul<i32> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: i32) -> Mod_e221_3 {
        self.small_mul(a)
    }
}

impl<'a> Mul<i16> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: i16) -> Mod_e221_3 {
        self.small_mul(a as i32)
    }
}

impl<'a> Mul<i8> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: i8) -> Mod_e221_3 {
        self.small_mul(a as i32)
    }
}

impl Mul<i32> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: i32) -> Mod_e221_3 {
        &self * a
    }
}

impl Mul<i16> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: i16) -> Mod_e221_3 {
        &self * a
    }
}

impl Mul<i8> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: i8) -> Mod_e221_3 {
        &self * a
    }
}

impl<'a, 'b> Mul<&'b Mod_e221_3> for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: &'b Mod_e221_3) -> Mod_e221_3 {
        let mut out = self.clone();
        out *= a;
        out
    }
}

impl Mul<Mod_e221_3> for Mod_e221_3 {
    type Output = Mod_e221_3;

    fn mul(self, a: Mod_e221_3) -> Mod_e221_3 {
        &self * &a
    }
}

impl Rand for Mod_e221_3 {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut out = Mod_e221_3([0i64; 4]);

        for i in 0..4 {
            out[i] = rng.gen_range(0, MODULUS[i]);
        }

        out
    }
}

impl PrimeField for Mod_e221_3 {
    fn unpack(&mut self, bytes: &[u8]) {
        self[0] = ((bytes[0] as i64) & 0x00000000000000ff) |
                  (((bytes[1] as i64) << 8) & 0x000000000000ff00) |
                  (((bytes[2] as i64) << 16) & 0x0000000000ff0000) |
                  (((bytes[3] as i64) << 24) & 0x00000000ff000000) |
                  (((bytes[4] as i64) << 32) & 0x000000ff00000000) |
                  (((bytes[5] as i64) << 40) & 0x0000ff0000000000) |
                  (((bytes[6] as i64) << 48) & 0x00ff000000000000) |
                  (((bytes[7] as i64) << 56) & 0x0300000000000000);
        self[1] = (((bytes[7] as i64) >> 2) & 0x000000000000003f) |
                  (((bytes[8] as i64) << 6) & 0x0000000000003fc0) |
                  (((bytes[9] as i64) << 14) & 0x00000000003fc000) |
                  (((bytes[10] as i64) << 22) & 0x000000003fc00000) |
                  (((bytes[11] as i64) << 30) & 0x0000003fc0000000) |
                  (((bytes[12] as i64) << 38) & 0x00003fc000000000) |
                  (((bytes[13] as i64) << 46) & 0x003fc00000000000) |
                  (((bytes[14] as i64) << 54) & 0x03c0000000000000);
        self[2] = (((bytes[14] as i64) >> 4) & 0x000000000000000f) |
                  (((bytes[15] as i64) << 4) & 0x0000000000000ff0) |
                  (((bytes[16] as i64) << 12) & 0x00000000000ff000) |
                  (((bytes[17] as i64) << 20) & 0x000000000ff00000) |
                  (((bytes[18] as i64) << 28) & 0x0000000ff0000000) |
                  (((bytes[19] as i64) << 36) & 0x00000ff000000000) |
                  (((bytes[20] as i64) << 44) & 0x000ff00000000000) |
                  (((bytes[21] as i64) << 52) & 0x03f0000000000000);
        self[3] = (((bytes[21] as i64) >> 6) & 0x0000000000000003) |
                  (((bytes[22] as i64) << 2) & 0x00000000000003fc) |
                  (((bytes[23] as i64) << 10) & 0x000000000003fc00) |
                  (((bytes[24] as i64) << 18) & 0x0000000003fc0000) |
                  (((bytes[25] as i64) << 26) & 0x00000003fc000000) |
                  (((bytes[26] as i64) << 34) & 0x000003fc00000000) |
                  (((bytes[27] as i64) << 42) & 0x00007c0000000000);
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
        bytes[6] = ((self[0] >> 48) & 0b11111111) as u8;
        bytes[7] = (((self[0] >> 56) & 0b00000011) as u8) |
                   (((self[1] << 2) & 0b11111100) as u8);
        bytes[8] = ((self[1] >> 6) & 0b11111111) as u8;
        bytes[9] = ((self[1] >> 14) & 0b11111111) as u8;
        bytes[10] = ((self[1] >> 22) & 0b11111111) as u8;
        bytes[11] = ((self[1] >> 30) & 0b11111111) as u8;
        bytes[12] = ((self[1] >> 38) & 0b11111111) as u8;
        bytes[13] = ((self[1] >> 46) & 0b11111111) as u8;
        bytes[14] = (((self[1] >> 54) & 0b00001111) as u8) |
                    (((self[2] << 4) & 0b11110000) as u8);
        bytes[15] = ((self[2] >> 4) & 0b11111111) as u8;
        bytes[16] = ((self[2] >> 12) & 0b11111111) as u8;
        bytes[17] = ((self[2] >> 20) & 0b11111111) as u8;
        bytes[18] = ((self[2] >> 28) & 0b11111111) as u8;
        bytes[19] = ((self[2] >> 36) & 0b11111111) as u8;
        bytes[20] = ((self[2] >> 44) & 0b11111111) as u8;
        bytes[21] = (((self[2] >> 52) & 0b00111111) as u8) |
                    (((self[3] << 6) & 0b11000000) as u8);
        bytes[22] = ((self[3] >> 2) & 0b11111111) as u8;
        bytes[23] = ((self[3] >> 10) & 0b11111111) as u8;
        bytes[24] = ((self[3] >> 18) & 0b11111111) as u8;
        bytes[25] = ((self[3] >> 26) & 0b11111111) as u8;
        bytes[26] = ((self[3] >> 34) & 0b11111111) as u8;
        bytes[27] = ((self[3] >> 42) & 0b00011111) as u8;
    }

    fn nbits() -> i32 {
        221
    }

    fn nbytes() -> i32 {
        28
    }

    fn normalize(&mut self) {
        let plusc = &*self + (C_VAL as i32);
        let offset = &MODULUS * (plusc.carry_out() as i32);
        *self -= &offset;
    }

    fn normalize_self_eq(&mut self, other: &Self) -> bool {
        let self_bytes =  self.packed();
        let other_bytes = other.packed_normalized();
        let mut are_equal: bool = true;

        for i in 0..28 {
            are_equal &= self_bytes[i] == other_bytes[i];
        }

        are_equal
    }

    fn normalize_eq(&mut self, other: &mut Self) -> bool {
        let self_bytes =  self.packed();
        let other_bytes = other.packed();
        let mut are_equal: bool = true;

        for i in 0..28 {
            are_equal &= self_bytes[i] == other_bytes[i];
        }

        are_equal
    }

    fn zero() -> Self {
        return ZERO;
    }

    fn one() -> Self {
        return ONE;
    }

    fn m_one() -> Self {
        return M_ONE;
    }

    fn modulus() -> Self {
        return MODULUS;
    }

    fn square(&mut self) {
        // Expand out to single digits
        let a0 = self[0] & 0x1fffffff;
        let a1 = self[0] >> 29;
        let a2 = self[1] & 0x1fffffff;
        let a3 = self[1] >> 29;
        let a4 = self[2] & 0x1fffffff;
        let a5 = self[2] >> 29;
        let a6 = self[3] & 0x1fffffff;
        let a7 = self[3] >> 29;

        // Combined multiples
        let m_0_0 = a0 * a0;
        let m_0_1 = a0 * a1;
        let m_0_2 = a0 * a2;
        let m_0_3 = a0 * a3;
        let m_0_4 = a0 * a4;
        let m_0_5 = a0 * a5;
        let m_0_6 = a0 * a6;
        let m_0_7 = a0 * a7;
        let m_1_0 = m_0_1;
        let m_1_1 = a1 * a1;
        let m_1_2 = a1 * a2;
        let m_1_3 = a1 * a3;
        let m_1_4 = a1 * a4;
        let m_1_5 = a1 * a5;
        let m_1_6 = a1 * a6;
        let m_1_7 = a1 * a7;
        let m_2_0 = m_0_2;
        let m_2_1 = m_1_2;
        let m_2_2 = a2 * a2;
        let m_2_3 = a2 * a3;
        let m_2_4 = a2 * a4;
        let m_2_5 = a2 * a5;
        let m_2_6 = a2 * a6;
        let m_2_7 = a2 * a7;
        let m_3_0 = m_0_3;
        let m_3_1 = m_1_3;
        let m_3_2 = m_2_3;
        let m_3_3 = a3 * a3;
        let m_3_4 = a3 * a4;
        let m_3_5 = a3 * a5;
        let m_3_6 = a3 * a6;
        let m_3_7 = a3 * a7;
        let m_4_0 = m_0_4;
        let m_4_1 = m_1_4;
        let m_4_2 = m_2_4;
        let m_4_3 = m_3_4;
        let m_4_4 = a4 * a4;
        let m_4_5 = a4 * a5;
        let m_4_6 = a4 * a6;
        let m_4_7 = a4 * a7;
        let m_5_0 = m_0_5;
        let m_5_1 = m_1_5;
        let m_5_2 = m_2_5;
        let m_5_3 = m_3_5;
        let m_5_4 = m_4_5;
        let m_5_5 = a5 * a5;
        let m_5_6 = a5 * a6;
        let m_5_7 = a5 * a7;
        let m_6_0 = m_0_6;
        let m_6_1 = m_1_6;
        let m_6_2 = m_2_6;
        let m_6_3 = m_3_6;
        let m_6_4 = m_4_6;
        let m_6_5 = m_5_6;
        let m_6_6 = a6 * a6;
        let m_6_7 = a6 * a7;
        let m_7_0 = m_0_7;
        let m_7_1 = m_1_7;
        let m_7_2 = m_2_7;
        let m_7_3 = m_3_7;
        let m_7_4 = m_4_7;
        let m_7_5 = m_5_7;
        let m_7_6 = m_6_7;
        let m_7_7 = a7 * a7;

        // Compute the 16-digit combined product using 64-bit operations.
        let d0 = m_0_0 + ((m_0_1 & 0x1fffffff) << 29) +
                 ((m_1_0 & 0x1fffffff) << 29);
        let c0 = d0 >> 58;
        let d1 = (m_0_1 >> 29) + m_0_2 + ((m_0_3 & 0x1fffffff) << 29) +
                 (m_1_0 >> 29) + m_1_1 + ((m_1_2 & 0x1fffffff) << 29) +
                 m_2_0 + ((m_2_1 & 0x1fffffff) << 29) +
                 ((m_3_0 & 0x1fffffff) << 29) + c0;
        let c1 = d1 >> 58;
        let d2 = (m_0_3 >> 29) + m_0_4 + ((m_0_5 & 0x1fffffff) << 29) +
                 (m_1_2 >> 29) + m_1_3 + ((m_1_4 & 0x1fffffff) << 29) +
                 (m_2_1 >> 29) + m_2_2 + ((m_2_3 & 0x1fffffff) << 29) +
                 (m_3_0 >> 29) + m_3_1 + ((m_3_2 & 0x1fffffff) << 29) +
                 m_4_0 + ((m_4_1 & 0x1fffffff) << 29) +
                 ((m_5_0 & 0x1fffffff) << 29) + c1;
        let c2 = d2 >> 58;
        let d3 = (m_0_5 >> 29) + m_0_6 + ((m_0_7 & 0x1fffffff) << 29) +
                 (m_1_4 >> 29) + m_1_5 + ((m_1_6 & 0x1fffffff) << 29) +
                 (m_2_3 >> 29) + m_2_4 + ((m_2_5 & 0x1fffffff) << 29) +
                 (m_3_2 >> 29) + m_3_3 + ((m_3_4 & 0x1fffffff) << 29) +
                 (m_4_1 >> 29) + m_4_2 + ((m_4_3 & 0x1fffffff) << 29) +
                 (m_5_0 >> 29) + m_5_1 + ((m_5_2 & 0x1fffffff) << 29) +
                 m_6_0 + ((m_6_1 & 0x1fffffff) << 29) +
                 ((m_7_0 & 0x1fffffff) << 29) + c2;
        let c3 = d3 >> 58;
        let d4 = (m_0_7 >> 29) +
                 (m_1_6 >> 29) + m_1_7 +
                 (m_2_5 >> 29) + m_2_6 + ((m_2_7 & 0x1fffffff) << 29) +
                 (m_3_4 >> 29) + m_3_5 + ((m_3_6 & 0x1fffffff) << 29) +
                 (m_4_3 >> 29) + m_4_4 + ((m_4_5 & 0x1fffffff) << 29) +
                 (m_5_2 >> 29) + m_5_3 + ((m_5_4 & 0x1fffffff) << 29) +
                 (m_6_1 >> 29) + m_6_2 + ((m_6_3 & 0x1fffffff) << 29) +
                 (m_7_0 >> 29) + m_7_1 + ((m_7_2 & 0x1fffffff) << 29) +
                 c3;
        let c4 = d4 >> 58;
        let d5 = (m_2_7 >> 29) +
                 (m_3_6 >> 29) + m_3_7 +
                 (m_4_5 >> 29) + m_4_6 + ((m_4_7 & 0x1fffffff) << 29) +
                 (m_5_4 >> 29) + m_5_5 + ((m_5_6 & 0x1fffffff) << 29) +
                 (m_6_3 >> 29) + m_6_4 + ((m_6_5 & 0x1fffffff) << 29) +
                 (m_7_2 >> 29) + m_7_3 + ((m_7_4 & 0x1fffffff) << 29) +
                 c4;
        let c5 = d5 >> 58;
        let d6 = (m_4_7 >> 29) +
                 (m_5_6 >> 29) + m_5_7 +
                 (m_6_5 >> 29) + m_6_6 + ((m_6_7 & 0x1fffffff) << 29) +
                 (m_7_4 >> 29) + m_7_5 + ((m_7_6 & 0x1fffffff) << 29) +
                 c5;
        let c6 = d6 >> 58;
        let d7 = (m_6_7 >> 29) +
                 (m_7_6 >> 29) + m_7_7 +
                 c6;

        // Modular reduction by a pseudo-mersenne prime of the form 2^n - c.

        // These are the n low-order
        let l0_0 = d0 & 0x03ffffffffffffff;
        let l1_0 = d1 & 0x03ffffffffffffff;
        let l2_0 = d2 & 0x03ffffffffffffff;
        let l3_0 = d3 & 0x00007fffffffffff;


        // Shift the high bits down into another n-bit number.
        let h0_0 = ((d3 & 0x03ffffffffffffff) >> 47) |
                   ((d4 & 0x00007fffffffffff) << 11);
        let h1_0 = ((d4 & 0x03ffffffffffffff) >> 47) |
                   ((d5 & 0x00007fffffffffff) << 11);
        let h2_0 = ((d5 & 0x03ffffffffffffff) >> 47) |
                   ((d6 & 0x00007fffffffffff) << 11);
        let h3_0 = ((d6 & 0x03ffffffffffffff) >> 47) |
                   (d7 << 11);

        // Multiply by C
        let hc0_0 = h0_0 * C_VAL;
        let hc1_0 = h1_0 * C_VAL;
        let hc2_0 = h2_0 * C_VAL;
        let hc3_0 = h3_0 * C_VAL;

        // Add h and l.
        let kin_0 = hc3_0 >> 47;
        let s0_0 = l0_0 + hc0_0 + (kin_0 * C_VAL);
        let k0_0 = s0_0 >> 58;
        let s1_0 = l1_0 + hc1_0 + k0_0;
        let k1_0 = s1_0 >> 58;
        let s2_0 = l2_0 + hc2_0 + k1_0;
        let k2_0 = s2_0 >> 58;
        let s3_0 = l3_0 + (hc3_0 & 0x00007fffffffffff) + k2_0;

        self[0] = s0_0 & 0x03ffffffffffffff;
        self[1] = s1_0 & 0x03ffffffffffffff;
        self[2] = s2_0 & 0x03ffffffffffffff;
        self[3] = s3_0;
    }

    fn squared(&self) -> Self {
        let mut out = self.clone();

        out.square();

        out
    }

    fn invert(&mut self) {
        // First digit is 1.
        let mut sqval = self.clone();

        // Second digit is 1.
        sqval.square();
        *self *= &sqval;

        // Third digit is 0.
        sqval.square();

        // All the remaining digits are 1.
        for _ in 3..221 {
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
        // First digit is zero

        // Second digit is one
        let mut out = self.squared();
        let mut sqval = out.clone();

        // All the remaining digits are 1.
        for _ in 2..220 {
            sqval.square();
            out *= &sqval;
        }

        out
    }

    fn sqrt(&self) -> Self {
        // Legendre's formula for 5 mod 8 primes.

        let mut out = self.clone();

        // All digits are 0 except the last.
        for _ in 0..218 {
            out.square();
        }

        let mut coeff = self.quartic_legendre();

        coeff -= &ONE;
        coeff *= &M_HALF;
        coeff *= &COEFF;
        coeff += &ONE;

        &out * &coeff
    }

    fn small_add_assign(&mut self, rhs: i32) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3] & 0x00007fffffffffff;

        let b = i64::from(rhs);

        let cin = self.carry_out();
        let s0 = a0 + b + (cin * C_VAL);
        let c0 = s0 >> 58;
        let s1 = a1 + c0;
        let c1 = s1 >> 58;
        let s2 = a2 + c1;
        let c2 = s2 >> 58;
        let s3 = a3 + c2;

        self[0] = s0 & 0x03ffffffffffffff;
        self[1] = s1 & 0x03ffffffffffffff;
        self[2] = s2 & 0x03ffffffffffffff;
        self[3] = s3;
    }

    fn small_add(&self, rhs: i32) -> Self {
        let mut out = self.clone();

        out.small_add_assign(rhs);

        out
    }

    fn small_sub_assign(&mut self, rhs: i32) {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3] & 0x00007fffffffffff;

        let b = i64::from(rhs);

        let cin = self.carry_out();
        let s0 = a0 - b + (cin * C_VAL);
        let c0 = s0 >> 58;
        let s1 = a1 + c0;
        let c1 = s1 >> 58;
        let s2 = a2 + c1;
        let c2 = s2 >> 58;
        let s3 = a3 + c2;

        self[0] = s0 & 0x03ffffffffffffff;
        self[1] = s1 & 0x03ffffffffffffff;
        self[2] = s2 & 0x03ffffffffffffff;
        self[3] = s3;
    }

    fn small_sub(&self, rhs: i32) -> Self {
        let mut out = self.clone();

        out.small_sub_assign(rhs);

        out
    }

    fn small_mul_assign(&mut self, rhs: i32) {
        let a0 = self[0] & 0x1fffffff;
        let a1 = self[0] >> 29;
        let a2 = self[1] & 0x1fffffff;
        let a3 = self[1] >> 29;
        let a4 = self[2] & 0x1fffffff;
        let a5 = self[2] >> 29;
        let a6 = self[3] & 0x1fffffff;
        let a7 = self[3] >> 29;

        let b = i64::from(rhs);

        let m0 = a0 * b;
        let m1 = a1 * b;
        let m2 = a2 * b;
        let m3 = a3 * b;
        let m4 = a4 * b;
        let m5 = a5 * b;
        let m6 = a6 * b;
        let m7 = a7 * b;

        let cin = self.carry_out();
        let d0 = m0 + ((m1 & 0x1fffffff) << 29) + (cin * C_VAL);
        let c0 = d0 >> 58;
        let d1 = (m1 >> 29) + m2 + ((m3 & 0x1fffffff) << 29) + c0;
        let c1 = d1 >> 58;
        let d2 = (m3 >> 29) + m4 + ((m5 & 0x1fffffff) << 29) + c1;
        let c2 = d2 >> 58;
        let d3 = (m5 >> 29) + m6 + (m7 << 29) + c2;

        self[0] = d0 & 0x03ffffffffffffff;
        self[1] = d1 & 0x03ffffffffffffff;
        self[2] = d2 & 0x03ffffffffffffff;
        self[3] = d3;
    }

    fn small_mul(&self, b: i32) -> Self {
        let mut out = self.clone();

        out.small_mul_assign(b);

        out
    }
}

#[cfg(test)]
mod tests {
    use field::prime_field::*;
    use field::mod_e221_3::*;

    const TWO: Mod_e221_3 = Mod_e221_3([ 2, 0, 0, 0 ]);

    const M_TWO: Mod_e221_3 =
        Mod_e221_3([ 0x03fffffffffffffb, 0x03ffffffffffffff,
                     0x03ffffffffffffff, 0x00007fffffffffff ]);

    const THREE: Mod_e221_3 = Mod_e221_3([ 3, 0, 0, 0 ]);

    const M_THREE: Mod_e221_3 =
        Mod_e221_3([ 0x03fffffffffffffa, 0x03ffffffffffffff,
                     0x03ffffffffffffff, 0x00007fffffffffff ]);

    const FOUR: Mod_e221_3 = Mod_e221_3([ 4, 0, 0, 0 ]);

    const M_FOUR: Mod_e221_3 =
        Mod_e221_3([ 0x03fffffffffffff9, 0x03ffffffffffffff,
                     0x03ffffffffffffff, 0x00007fffffffffff ]);

    const SIX: Mod_e221_3 = Mod_e221_3([ 6, 0, 0, 0 ]);

    const M_SIX: Mod_e221_3 =
        Mod_e221_3([ 0x03fffffffffffff7, 0x03ffffffffffffff,
                     0x03ffffffffffffff, 0x00007fffffffffff ]);

    const EIGHT: Mod_e221_3 = Mod_e221_3([ 8, 0, 0, 0 ]);

    const M_EIGHT: Mod_e221_3 =
        Mod_e221_3([ 0x03fffffffffffff5, 0x03ffffffffffffff,
                     0x03ffffffffffffff, 0x00007fffffffffff ]);

    const NINE: Mod_e221_3 = Mod_e221_3([ 9, 0, 0, 0 ]);

    const M_NINE: Mod_e221_3 =
        Mod_e221_3([ 0x03fffffffffffff4, 0x03ffffffffffffff,
                     0x03ffffffffffffff, 0x00007fffffffffff ]);

    const SIXTEEN: Mod_e221_3 = Mod_e221_3([ 16, 0, 0, 0 ]);

    const M_SIXTEEN: Mod_e221_3 =
        Mod_e221_3([ 0x03ffffffffffffed, 0x03ffffffffffffff,
                     0x03ffffffffffffff, 0x00007fffffffffff ]);

    fn test_pack_unpack(expected: &[u8; 28]) {
        let mut unpacked = Mod_e221_3::unpacked(expected);
        let actual = unpacked.packed();

        for i in 0..28 {
            assert!(expected[i] == actual[i]);
        }
    }

    fn test_unpack_pack(expected: &mut Mod_e221_3) {
        let bytes = expected.packed();
        let actual = Mod_e221_3::unpacked(&bytes);

        for i in 0..4 {
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
                           0xff, 0x00, 0xff, 0x00]);
        test_pack_unpack(&[0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0xff,
                           0x00, 0xff, 0x00, 0x1f]);
        test_pack_unpack(&[0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0xaa,
                           0x55, 0xaa, 0x55, 0x0a]);
        test_pack_unpack(&[0xaa, 0x55, 0xaa, 0x55,
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
                           0x00, 0xaa, 0x00, 0x0a]);
        test_pack_unpack(&[0xaa, 0x00, 0xaa, 0x00,
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
                           0x55, 0xff, 0x55, 0x1f]);
        test_pack_unpack(&[0xff, 0x55, 0xff, 0x55,
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
        test_unpack_pack(&mut Mod_e221_3([ 0x03ffffffffffffff,
                                           0x0000000000000000,
                                           0x03ffffffffffffff,
                                           0x0000000000000000 ]));
        test_unpack_pack(&mut Mod_e221_3([ 0x0000000000000000,
                                           0x03ffffffffffffff,
                                           0x0000000000000000,
                                           0x00007fffffffffff ]));
        test_unpack_pack(&mut Mod_e221_3([ 0x02aaaaaaaaaaaaaa,
                                           0x0155555555555555,
                                           0x02aaaaaaaaaaaaaa,
                                           0x0000555555555555 ]));
        test_unpack_pack(&mut Mod_e221_3([ 0x0155555555555555,
                                           0x02aaaaaaaaaaaaaa,
                                           0x0155555555555555,
                                           0x00002aaaaaaaaaaa ]));
        test_unpack_pack(&mut Mod_e221_3([ 0x02aaaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x02aaaaaaaaaaaaaa,
                                           0x0000000000000000 ]));
        test_unpack_pack(&mut Mod_e221_3([ 0x0000000000000000,
                                           0x02aaaaaaaaaaaaaa,
                                           0x0000000000000000,
                                           0x00002aaaaaaaaaaa ]));
        test_unpack_pack(&mut Mod_e221_3([ 0x03ffffffffffffff,
                                           0x0155555555555555,
                                           0x03ffffffffffffff,
                                           0x0000555555555555 ]));
        test_unpack_pack(&mut Mod_e221_3([ 0x0155555555555555,
                                           0x03ffffffffffffff,
                                           0x0155555555555555,
                                           0x00007fffffffffff ]));
    }

    #[test]
    fn test_add() {
        let l1_zeros: [&mut Mod_e221_3; 5] = [ &mut (&ZERO + &ZERO),
                                                &mut (&M_ONE + &ONE),
                                                &mut (&ONE + &M_ONE),
                                                &mut (&M_TWO + &TWO),
                                                &mut (&TWO + &M_TWO) ];

        let l1_ones: [&mut Mod_e221_3; 4] = [ &mut (&ZERO + &ONE),
                                                &mut (&ONE + &ZERO),
                                                &mut (&M_ONE + &TWO),
                                                &mut (&TWO + &M_ONE) ];

        let l1_twos: [&mut Mod_e221_3; 3] = [ &mut (&ZERO + &TWO),
                                                &mut (&ONE + &ONE),
                                                &mut (&TWO + &ZERO) ];

        let l1_mones: [&mut Mod_e221_3; 4] = [ &mut (&ZERO + &M_ONE),
                                                 &mut (&M_ONE + &ZERO),
                                                 &mut (&M_TWO + &ONE),
                                                 &mut (&ONE + &M_TWO) ];

        let l1_mtwos: [&mut Mod_e221_3; 3] = [ &mut (&ZERO + &M_TWO),
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
                let mut val = *l1_zeros[i] + *l1_zeros[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = *l1_mones[i] + *l1_ones[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = *l1_ones[i] + *l1_mones[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = *l1_mtwos[i] + *l1_twos[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let mut val = *l1_twos[i] + *l1_mtwos[j];

                assert!(ZERO.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..4 {
                let mut val = *l1_zeros[i] + *l1_ones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..5 {
                let mut val = *l1_ones[i] + *l1_zeros[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..3 {
                let mut val = *l1_mones[i] + *l1_twos[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = *l1_twos[i] + *l1_mones[j];

                assert!(ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..3 {
                let mut val = *l1_zeros[i] + *l1_twos[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = *l1_ones[i] + *l1_ones[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..5 {
                let mut val = *l1_twos[i] + *l1_zeros[j];

                assert!(TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..4 {
                let mut val = *l1_zeros[i] + *l1_mones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..5 {
                let mut val = *l1_mones[i] + *l1_zeros[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..4 {
                let mut val = *l1_mtwos[i] + *l1_ones[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..3 {
                let mut val = *l1_ones[i] + *l1_mtwos[j];

                assert!(M_ONE.normalize_eq(&mut val));
            }
        }

        for i in 0..5 {
            for j in 0..3 {
                let mut val = *l1_zeros[i] + *l1_mtwos[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                let mut val = *l1_mones[i] + *l1_mones[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }

        for i in 0..3 {
            for j in 0..5 {
                let mut val = *l1_mtwos[i] + *l1_zeros[j];

                assert!(M_TWO.normalize_eq(&mut val));
            }
        }
    }


    #[test]
    fn test_sub() {
        let l1_zeros: [&mut Mod_e221_3; 3] = [ &mut (&ZERO - &ZERO),
                                                 &mut (&ONE - &ONE),
                                                 &mut (&TWO - &TWO) ];

        let l1_ones: [&mut Mod_e221_3; 4] = [ &mut (&ZERO - &M_ONE),
                                                &mut (&ONE - &ZERO),
                                                &mut (&M_ONE - &M_TWO),
                                                &mut (&TWO - &ONE) ];

        let l1_twos: [&mut Mod_e221_3; 3] = [ &mut (&ZERO - &M_TWO),
                                                &mut (&ONE - &M_ONE),
                                                &mut (&TWO - &ZERO) ];

        let l1_mones: [&mut Mod_e221_3; 4] = [ &mut (&ZERO - &ONE),
                                                 &mut (&M_ONE - &ZERO),
                                                 &mut (&M_TWO - &M_ONE),
                                                 &mut (&ONE - &TWO) ];

        let l1_mtwos: [&mut Mod_e221_3; 3] = [ &mut (&ZERO - &TWO),
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
        let l1_zeros: [&mut Mod_e221_3; 9] = [ &mut (&ZERO * &ZERO),
                                                 &mut (&ONE * &ZERO),
                                                 &mut (&TWO * &ZERO),
                                                 &mut (&M_ONE * &ZERO),
                                                 &mut (&M_TWO * &ZERO),
                                                 &mut (&ZERO * &ONE),
                                                 &mut (&ZERO * &TWO),
                                                 &mut (&ZERO * &M_ONE),
                                                 &mut (&ZERO * &M_TWO) ];

        let l1_ones: [&mut Mod_e221_3; 2] = [ &mut (&ONE * &ONE),
                                                &mut (&M_ONE * &M_ONE) ];

        let l1_twos: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &TWO),
                                                &mut (&TWO * &ONE),
                                                &mut (&M_ONE * &M_TWO),
                                                &mut (&M_TWO * &M_ONE) ];

        let l1_fours: [&mut Mod_e221_3; 2] = [ &mut (&TWO * &TWO),
                                                 &mut (&M_TWO * &M_TWO) ];

        let l1_mones: [&mut Mod_e221_3; 2] = [ &mut (&ONE * &M_ONE),
                                                 &mut (&M_ONE * &ONE) ];

        let l1_mtwos: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &M_TWO),
                                                 &mut (&TWO * &M_ONE),
                                                 &mut (&M_ONE * &TWO),
                                                 &mut (&M_TWO * &ONE) ];

        let l1_mfours: [&mut Mod_e221_3; 2] = [ &mut (&TWO * &M_TWO),
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
        let l1_zeros: [&mut Mod_e221_3; 10] = [ &mut (&ZERO * &ZERO),
                                                  &mut (&ONE * &ZERO),
                                                  &mut (&TWO * &ZERO),
                                                  &mut (&M_ONE * &ZERO),
                                                  &mut (&M_TWO * &ZERO),
                                                  &mut (&ZERO * &ONE),
                                                  &mut (&ZERO * &TWO),
                                                  &mut (&ZERO * &M_ONE),
                                                  &mut (&ZERO * &M_TWO),
                                                  &mut ZERO.squared() ];

        let l1_ones: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &ONE),
                                                &mut (&M_ONE * &M_ONE),
                                                &mut ONE.squared(),
                                                &mut M_ONE.squared() ];

        let l1_twos: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &TWO),
                                                &mut (&TWO * &ONE),
                                                &mut (&M_ONE * &M_TWO),
                                                &mut (&M_TWO * &M_ONE) ];

        let l1_threes: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &THREE),
                                                  &mut (&THREE * &ONE),
                                                  &mut (&M_ONE * &M_THREE),
                                                  &mut (&M_THREE * &M_ONE) ];

        let l1_fours: [&mut Mod_e221_3; 4] = [ &mut (&TWO * &TWO),
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
        let l1_ones: [&mut Mod_e221_3; 2] = [ &mut (&ONE * &ONE),
                                              &mut (&M_ONE * &M_ONE) ];

        let l1_twos: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &TWO),
                                              &mut (&TWO * &ONE),
                                              &mut (&M_ONE * &M_TWO),
                                              &mut (&M_TWO * &M_ONE) ];

        let l1_threes: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &THREE),
                                                &mut (&THREE * &ONE),
                                                &mut (&M_ONE * &M_THREE),
                                                &mut (&M_THREE * &M_ONE) ];

        let l1_fours: [&mut Mod_e221_3; 2] = [ &mut (&TWO * &TWO),
                                               &mut (&M_TWO * &M_TWO) ];

        let l1_mones: [&mut Mod_e221_3; 2] = [ &mut (&ONE * &M_ONE),
                                               &mut (&M_ONE * &ONE) ];

        let l1_mtwos: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &M_TWO),
                                               &mut (&TWO * &M_ONE),
                                               &mut (&M_ONE * &TWO),
                                               &mut (&M_TWO * &ONE) ];

        let l1_mthrees: [&mut Mod_e221_3; 4] = [ &mut (&ONE * &M_THREE),
                                                 &mut (&THREE * &M_ONE),
                                                 &mut (&M_ONE * &THREE),
                                                 &mut (&M_THREE * &ONE) ];

        let l1_mfours: [&mut Mod_e221_3; 2] = [ &mut (&TWO * &M_TWO),
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
        let l1_ones: [&mut Mod_e221_3; 12] = [ &mut (&ONE / &ONE),
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

        let l1_twos: [&mut Mod_e221_3; 10] = [ &mut (&TWO / &ONE),
                                                 &mut (&M_TWO / &M_ONE),
                                                 &mut (&FOUR / &TWO),
                                                 &mut (&M_FOUR / &M_TWO),
                                                 &mut (&SIX / &THREE),
                                                 &mut (&M_SIX / &M_THREE),
                                                 &mut (&EIGHT / &FOUR),
                                                 &mut (&M_EIGHT / &M_FOUR),
                                                 &mut (&SIXTEEN / &EIGHT),
                                                 &mut (&M_SIXTEEN / &M_EIGHT) ];

        let l1_threes: [&mut Mod_e221_3; 6] = [ &mut (&THREE / &ONE),
                                                  &mut (&M_THREE / &M_ONE),
                                                  &mut (&SIX / &TWO),
                                                  &mut (&M_SIX / &M_TWO),
                                                  &mut (&NINE / &THREE),
                                                  &mut (&M_NINE / &M_THREE) ];

        let l1_fours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e221_3; 12] = [ &mut (&ONE / &M_ONE),
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

        let l1_mtwos: [&mut Mod_e221_3; 10] = [ &mut (&TWO / &M_ONE),
                                                  &mut (&M_TWO / &ONE),
                                                  &mut (&FOUR / &M_TWO),
                                                  &mut (&M_FOUR / &TWO),
                                                  &mut (&SIX / &M_THREE),
                                                  &mut (&M_SIX / &THREE),
                                                  &mut (&EIGHT / &M_FOUR),
                                                  &mut (&M_EIGHT / &FOUR),
                                                  &mut (&SIXTEEN / &M_EIGHT),
                                                  &mut (&M_SIXTEEN / &EIGHT) ];

        let l1_mthrees: [&mut Mod_e221_3; 6] = [ &mut (&THREE / &M_ONE),
                                                   &mut (&M_THREE / &ONE),
                                                   &mut (&SIX / &M_TWO),
                                                   &mut (&M_SIX / &TWO),
                                                   &mut (&NINE / &M_THREE),
                                                   &mut (&M_NINE / &THREE) ];

        let l1_mfours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &M_ONE),
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
        let l1_zeros: [&mut Mod_e221_3; 10] = [ &mut (&ZERO * &ZERO),
                                                  &mut (&ONE * &ZERO),
                                                  &mut (&TWO * &ZERO),
                                                  &mut (&M_ONE * &ZERO),
                                                  &mut (&M_TWO * &ZERO),
                                                  &mut (&ZERO * &ONE),
                                                  &mut (&ZERO * &TWO),
                                                  &mut (&ZERO * &M_ONE),
                                                  &mut (&ZERO * &M_TWO),
                                                  &mut ZERO.squared() ];

        let l1_ones: [&mut Mod_e221_3; 12] = [ &mut (&ONE / &ONE),
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

        let l1_twos: [&mut Mod_e221_3; 10] = [ &mut (&TWO / &ONE),
                                                 &mut (&M_TWO / &M_ONE),
                                                 &mut (&FOUR / &TWO),
                                                 &mut (&M_FOUR / &M_TWO),
                                                 &mut (&SIX / &THREE),
                                                 &mut (&M_SIX / &M_THREE),
                                                 &mut (&EIGHT / &FOUR),
                                                 &mut (&M_EIGHT / &M_FOUR),
                                                 &mut (&SIXTEEN / &EIGHT),
                                                 &mut (&M_SIXTEEN / &M_EIGHT) ];

        let l1_threes: [&mut Mod_e221_3; 6] = [ &mut (&THREE / &ONE),
                                                  &mut (&M_THREE / &M_ONE),
                                                  &mut (&SIX / &TWO),
                                                  &mut (&M_SIX / &M_TWO),
                                                  &mut (&NINE / &THREE),
                                                  &mut (&M_NINE / &M_THREE) ];

        let l1_fours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e221_3; 12] = [ &mut (&ONE / &M_ONE),
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

        let l1_mtwos: [&mut Mod_e221_3; 10] = [ &mut (&TWO / &M_ONE),
                                                  &mut (&M_TWO / &ONE),
                                                  &mut (&FOUR / &M_TWO),
                                                  &mut (&M_FOUR / &TWO),
                                                  &mut (&SIX / &M_THREE),
                                                  &mut (&M_SIX / &THREE),
                                                  &mut (&EIGHT / &M_FOUR),
                                                  &mut (&M_EIGHT / &FOUR),
                                                  &mut (&SIXTEEN / &M_EIGHT),
                                                  &mut (&M_SIXTEEN / &EIGHT) ];

        let l1_mthrees: [&mut Mod_e221_3; 6] = [ &mut (&THREE / &M_ONE),
                                                   &mut (&M_THREE / &ONE),
                                                   &mut (&SIX / &M_TWO),
                                                   &mut (&M_SIX / &TWO),
                                                   &mut (&NINE / &M_THREE),
                                                   &mut (&M_NINE / &THREE) ];

        let l1_mfours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &M_ONE),
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

            assert!(M_ONE.normalize_eq(&mut val));
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

            assert!(ONE.normalize_eq(&mut val));
        }

        for i in 0..10 {
            let mut val = l1_mtwos[i].legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..6 {
            let mut val = l1_mthrees[i].legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..6 {
            let mut val = l1_mfours[i].legendre();

            assert!(ONE.normalize_eq(&mut val));
        }
    }

    #[test]
    fn test_quartic_legendre() {
        let l1_fours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &ONE),
                                                 &mut (&M_FOUR / &M_ONE),
                                                 &mut (&EIGHT / &TWO),
                                                 &mut (&M_EIGHT / &M_TWO),
                                                 &mut (&SIXTEEN / &FOUR),
                                                 &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e221_3; 12] = [ &mut (&ONE / &M_ONE),
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

        let l1_mfours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &M_ONE),
                                                 &mut (&M_FOUR / &ONE),
                                                 &mut (&EIGHT / &M_TWO),
                                                 &mut (&M_EIGHT / &TWO),
                                                 &mut (&SIXTEEN / &M_FOUR),
                                                 &mut (&M_SIXTEEN / &FOUR) ];

        for i in 0..6 {
            let mut val = l1_fours[i].quartic_legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..12 {
            let mut val = l1_mones[i].quartic_legendre();

            assert!(M_ONE.normalize_eq(&mut val));
        }

        for i in 0..6 {
            let mut val = l1_mfours[i].quartic_legendre();

            assert!(ONE.normalize_eq(&mut val));
        }
    }

    #[test]
    fn test_sqrt() {
        let l1_fours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &ONE),
                                                &mut (&M_FOUR / &M_ONE),
                                                &mut (&EIGHT / &TWO),
                                                &mut (&M_EIGHT / &M_TWO),
                                                &mut (&SIXTEEN / &FOUR),
                                                &mut (&M_SIXTEEN / &M_FOUR) ];

        let l1_mones: [&mut Mod_e221_3; 12] = [ &mut (&ONE / &M_ONE),
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

        let l1_mfours: [&mut Mod_e221_3; 6] = [ &mut (&FOUR / &M_ONE),
                                                 &mut (&M_FOUR / &ONE),
                                                 &mut (&EIGHT / &M_TWO),
                                                 &mut (&M_EIGHT / &TWO),
                                                 &mut (&SIXTEEN / &M_FOUR),
                                                 &mut (&M_SIXTEEN / &FOUR) ];

        for i in 0..6 {
            let val = l1_fours[i].sqrt();

            assert!(val.squared().normalize_eq(l1_fours[i]));
        }

        for i in 0..12 {
            let val = l1_mones[i].sqrt();

            assert!(val.squared().normalize_eq(l1_mones[i]));
        }

        for i in 0..6 {
            let val = l1_mfours[i].sqrt();

            assert!(val.squared().normalize_eq(l1_mfours[i]));
        }
    }

    #[test]
    fn test_small_add() {
        let l1_zeros: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_add(0),
                                                 &mut M_ONE.small_add(1),
                                                 &mut ONE.small_add(-1),
                                                 &mut M_TWO.small_add(2),
                                                 &mut TWO.small_add(-2) ];

        let l1_ones: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_add(1),
                                                &mut M_ONE.small_add(2),
                                                &mut ONE.small_add(0),
                                                &mut M_TWO.small_add(3),
                                                &mut TWO.small_add(-1) ];

        let l1_twos: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_add(2),
                                                &mut ONE.small_add(1),
                                                &mut M_ONE.small_add(3),
                                                &mut TWO.small_add(0),
                                                &mut M_TWO.small_add(4) ];

        let l1_mones: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_add(-1),
                                                 &mut M_ONE.small_add(0),
                                                 &mut ONE.small_add(-2),
                                                 &mut M_TWO.small_add(1),
                                                 &mut TWO.small_add(-3) ];

        let l1_mtwos: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_add(-2),
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
        let l1_zeros: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_sub(0),
                                                 &mut M_ONE.small_sub(-1),
                                                 &mut ONE.small_sub(1),
                                                 &mut M_TWO.small_sub(-2),
                                                 &mut TWO.small_sub(2) ];

        let l1_ones: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_sub(-1),
                                                &mut M_ONE.small_sub(-2),
                                                &mut ONE.small_sub(0),
                                                &mut M_TWO.small_sub(-3),
                                                &mut TWO.small_sub(1) ];

        let l1_twos: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_sub(-2),
                                                &mut ONE.small_sub(-1),
                                                &mut M_ONE.small_sub(-3),
                                                &mut TWO.small_sub(0),
                                                &mut M_TWO.small_sub(-4) ];

        let l1_mones: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_sub(1),
                                                 &mut M_ONE.small_sub(0),
                                                 &mut ONE.small_sub(2),
                                                 &mut M_TWO.small_sub(-1),
                                                 &mut TWO.small_sub(3) ];

        let l1_mtwos: [&mut Mod_e221_3; 5] = [ &mut ZERO.small_sub(2),
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

        let l1_zeros: [&mut Mod_e221_3; 9] = [ &mut ZERO.small_mul(0),
                                                 &mut ONE.small_mul(0),
                                                 &mut TWO.small_mul(0),
                                                 &mut M_ONE.small_mul(0),
                                                 &mut M_TWO.small_mul(0),
                                                 &mut ZERO.small_mul(1),
                                                 &mut ZERO.small_mul(2),
                                                 &mut ZERO.small_mul(-1),
                                                 &mut ZERO.small_mul(-2) ];

        let l1_ones: [&mut Mod_e221_3; 2] = [ &mut ONE.small_mul(1),
                                                &mut M_ONE.small_mul(-1) ];

        let l1_twos: [&mut Mod_e221_3; 4] = [ &mut ONE.small_mul(2),
                                                &mut TWO.small_mul(1),
                                                &mut M_ONE.small_mul(-2),
                                                &mut M_TWO.small_mul(-1) ];

        let l1_fours: [&mut Mod_e221_3; 2] = [ &mut TWO.small_mul(2),
                                                 &mut M_TWO.small_mul(-2) ];

        let l1_mones: [&mut Mod_e221_3; 2] = [ &mut ONE.small_mul(-1),
                                                 &mut M_ONE.small_mul(1) ];

        let l1_mtwos: [&mut Mod_e221_3; 4] = [ &mut ONE.small_mul(-2),
                                                 &mut TWO.small_mul(-1),
                                                 &mut M_ONE.small_mul(2),
                                                 &mut M_TWO.small_mul(1) ];

        let l1_mfours: [&mut Mod_e221_3; 2] = [ &mut TWO.small_mul(-2),
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
