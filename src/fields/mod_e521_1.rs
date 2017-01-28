use fields::prime_field::PrimeField;
use std::clone::Clone;
//use std::cmp::Eq;
//use std::cmp::PartialEq;
use std::fmt::Debug;
use std::fmt::LowerHex;
use std::fmt::UpperHex;
use std::ops::AddAssign;
use std::ops::Add;
use std::ops::MulAssign;
use std::ops::Mul;
use std::ops::SubAssign;
use std::ops::Sub;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Neg;

/// Elements of the finite field mod 2^521 - 1.  Used by the E-521
/// curve.  Uses a 20-length array of 27-bit digits, with the final
/// digit having 8 bits.

#[derive(Copy, Clone)]
pub struct Mod_e521_1(pub [u32; 20]);

/// The normalized representation of the value 0.
pub const ZERO: Mod_e521_1 = Mod_e521_1([ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                          0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

/// The normalized representation of the value 1.
pub const ONE: Mod_e521_1 = Mod_e521_1([ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

/// The normalized representation of the value -1.
pub const M_ONE: Mod_e521_1 =
    Mod_e521_1([ 0x07fffffe, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x000000ff ]);

/// The normalized representation of the modulus 2^521 - 1.
pub const MODULUS: Mod_e521_1 =
    Mod_e521_1([ 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                 0x07ffffff, 0x07ffffff, 0x07ffffff, 0x000000ff ]);

impl Debug for Mod_e521_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "Mod_e521_1: [ {:x}", &self[0]));

        for i in 1..20 {
            try!(write!(f, ", {:x}", &self[i]));
        }

        write!(f, " ]")
    }
}

impl LowerHex for Mod_e521_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.pack();

        for i in 66..0 {
            try!(write!(f, "{:x}", bytes[i]));
        }

        Ok(())
    }
}

impl UpperHex for Mod_e521_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut cpy = self.clone();
        let bytes = cpy.pack();

        for i in 66..0 {
            try!(write!(f, "{:X}", bytes[i]));
        }

        Ok(())
    }
}

impl PrimeField for Mod_e521_1 {
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
}

impl Mod_e521_1 {
    fn normalize_eq(&mut self, other: &mut Mod_e521_1) -> bool {
        let self_bytes =  self.pack();
        let other_bytes = other.pack();
        let mut are_equal: bool = true;

        for i in 0..66 {
            are_equal &= self_bytes[i] == other_bytes[i];
        }

        are_equal
    }

    /// Get the carry-in value.  We use the highest carry slot to
    /// stash the carry-out value of each operation, and feed that
    /// back into the next one.
    fn carry_out(&self) -> i64 {
        i64::from((self[19] as i32) >> 8)
    }

    /// Multiply by a small number (an i64 that really contains an
    /// i32-representable value).
    pub fn small_mult(self, b: i64) -> Mod_e521_1 {
        let mut out = self.clone();

        let cin: i64 = self.carry_out();
        let a0: i64 = self[0] as i64;
        let a1: i64 = self[1] as i64;
        let a2: i64 = self[2] as i64;
        let a3: i64 = self[3] as i64;
        let a4: i64 = self[4] as i64;
        let a5: i64 = self[5] as i64;
        let a6: i64 = self[6] as i64;
        let a7: i64 = self[7] as i64;
        let a8: i64 = self[8] as i64;
        let a9: i64 = self[9] as i64;
        let a10: i64 = self[10] as i64;
        let a11: i64 = self[11] as i64;
        let a12: i64 = self[12] as i64;
        let a13: i64 = self[13] as i64;
        let a14: i64 = self[14] as i64;
        let a15: i64 = self[15] as i64;
        let a16: i64 = self[16] as i64;
        let a17: i64 = self[17] as i64;
        let a18: i64 = self[18] as i64;
        let a19: i64 = self[19] as i64;

        let m0: i64 = a0 * b;
        let m1: i64 = a1 * b;
        let m2: i64 = a2 * b;
        let m3: i64 = a3 * b;
        let m4: i64 = a4 * b;
        let m5: i64 = a5 * b;
        let m6: i64 = a6 * b;
        let m7: i64 = a7 * b;
        let m8: i64 = a8 * b;
        let m9: i64 = a9 * b;
        let m10: i64 = a10 * b;
        let m11: i64 = a11 * b;
        let m12: i64 = a12 * b;
        let m13: i64 = a13 * b;
        let m14: i64 = a14 * b;
        let m15: i64 = a15 * b;
        let m16: i64 = a16 * b;
        let m17: i64 = a17 * b;
        let m18: i64 = a18 * b;
        let m19: i64 = a19 * b;

        let d0 = (m0 & 0x3fffffffffffff) + ((m1 & 0x07ffffff) << 27) + cin;
        let c0 = d0 >> 54;
        let d1 = (m1 >> 27) + (m2 & 0x3fffffffffffff) +
                 ((m3 & 0x07ffffff) << 27) + c0;
        let c1 = d1 >> 54;
        let d2 = (m3 >> 27) + (m4 & 0x3fffffffffffff) +
                 ((m5 & 0x07ffffff) << 27) + c1;
        let c2 = d2 >> 54;
        let d3 = (m5 >> 27) + (m6 & 0x3fffffffffffff) +
                 ((m7 & 0x07ffffff) << 27) + c2;
        let c3 = d3 >> 54;
        let d4 = (m7 >> 27) + (m8 & 0x3fffffffffffff) +
                 ((m9 & 0x07ffffff) << 27) + c3;
        let c4 = d4 >> 54;
        let d5 = (m9 >> 27) + (m10 & 0x3fffffffffffff) +
                 ((m11 & 0x07ffffff) << 27) + c4;
        let c5 = d5 >> 54;
        let d6 = (m11 >> 27) + (m12 & 0x3fffffffffffff) +
                 ((m13 & 0x07ffffff) << 27) + c5;
        let c6 = d5 >> 54;
        let d7 = (m13 >> 27) + (m14 & 0x3fffffffffffff) +
                 ((m15 & 0x07ffffff) << 27) + c6;
        let c7 = d6 >> 54;
        let d8 = (m15 >> 27) + (m16 & 0x3fffffffffffff) +
                 ((m17 & 0x07ffffff) << 27) + c7;
        let c8 = d7 >> 54;
        let d9 = (m17 >> 27) + (m18 & 0x3fffffffffffff) +
                 ((m19 & 0x07ffffff) << 27) + c8;

        out[0] = (d0 & 0x07ffffff) as u32;
        out[1] = ((d0 >> 27) & 0x07ffffff) as u32;
        out[2] = (d1 & 0x07ffffff) as u32;
        out[3] = ((d1 >> 27) & 0x07ffffff) as u32;
        out[4] = (d2 & 0x07ffffff) as u32;
        out[5] = ((d2 >> 27) & 0x07ffffff) as u32;
        out[6] = (d3 & 0x07ffffff) as u32;
        out[7] = ((d3 >> 27) & 0x07ffffff) as u32;
        out[8] = (d4 & 0x07ffffff) as u32;
        out[9] = ((d4 >> 27) & 0x07ffffff) as u32;
        out[10] = (d5 & 0x07ffffff) as u32;
        out[11] = ((d5 >> 27) & 0x07ffffff) as u32;
        out[12] = (d6 & 0x07ffffff) as u32;
        out[13] = ((d6 >> 27) & 0x07ffffff) as u32;
        out[14] = (d7 & 0x07ffffff) as u32;
        out[15] = ((d7 >> 27) & 0x07ffffff) as u32;
        out[16] = (d8 & 0x07ffffff) as u32;
        out[17] = ((d8 >> 27) & 0x07ffffff) as u32;
        out[18] = (d9 & 0x07ffffff) as u32;
        out[19] = ((d9 >> 27) & 0x07ffffff) as u32;

        out
    }

    /// Normalize the representation, resulting in the internal digits
    /// holding a value that is truly less than 2^521 - 1.
    ///
    /// This can be done n mod (2^m - c) using a single add and small
    /// multiply as follows: we can detect overflow by doing
    /// carry_out(n + c), thus, we can normalize the number by doing
    /// n - (carry_out(n + c) * (2^m - c))
    pub fn normalize(&mut self) {
        let plusone = &(self.clone()) + &ONE;
        let offset = MODULUS.small_mult(plusone.carry_out());
        *self -= &offset;
    }

    /// Serialize a value as a little-endian byte array.  This has the
    /// effect of normalizing the representation.
    pub fn pack(&mut self) -> [u8; 66] {
        self.normalize();
        self.pack_normalized()
    }

    /// Serialize an already normalized number as a little-endian byte
    /// array.  This must only be used on a normalized value.
    pub fn pack_normalized(&mut self) -> [u8; 66] {
        let mut bytes = [0u8; 66];

        bytes[0] = (self[0] & 0b11111111) as u8;
        bytes[1] = ((self[0] >> 8) & 0b11111111) as u8;
        bytes[2] = ((self[0] >> 16) & 0b11111111) as u8;
        bytes[3] = (((self[0] >> 24) & 0b00000111) |
                    ((self[1] << 3) & 0b11111000)) as u8;
        bytes[4] = ((self[1] >> 5) & 0b11111111) as u8;
        bytes[5] = ((self[1] >> 13) & 0b11111111) as u8;
        bytes[6] = (((self[1] >> 21) & 0b00111111) |
                    ((self[2] << 6) & 0b11000000)) as u8;
        bytes[7] = ((self[2] >> 2) & 0b11111111) as u8;
        bytes[8] = ((self[2] >> 10) & 0b11111111) as u8;
        bytes[9] = ((self[2] >> 18) & 0b11111111) as u8;
        bytes[10] = (((self[2] >> 26) & 0b00000001) |
                     ((self[3] << 1) & 0b11111110)) as u8;
        bytes[11] = ((self[3] >> 7) & 0b11111111) as u8;
        bytes[12] = ((self[3] >> 15) & 0b11111111) as u8;
        bytes[13] = (((self[3] >> 23) & 0b00001111) |
                     ((self[4] << 4) & 0b11110000)) as u8;
        bytes[14] = ((self[4] >> 4) & 0b11111111) as u8;
        bytes[15] = ((self[4] >> 12) & 0b11111111) as u8;
        bytes[16] = (((self[4] >> 20) & 0b01111111) |
                     ((self[5] << 7) & 0b10000000)) as u8;
        bytes[17] = ((self[5] >> 1) & 0b11111111) as u8;
        bytes[18] = ((self[5] >> 9) & 0b11111111) as u8;
        bytes[19] = ((self[5] >> 17) & 0b11111111) as u8;
        bytes[20] = (((self[5] >> 25) & 0b00000011) |
                     ((self[6] << 2) & 0b11111100)) as u8;
        bytes[21] = ((self[6] >> 6) & 0b11111111) as u8;
        bytes[22] = ((self[6] >> 14) & 0b11111111) as u8;
        bytes[23] = (((self[6] >> 22) & 0b00011111) |
                     ((self[7] << 5) & 0b11100000)) as u8;
        bytes[24] = ((self[7] >> 3) & 0b11111111) as u8;
        bytes[25] = ((self[7] >> 11) & 0b11111111) as u8;
        bytes[26] = ((self[7] >> 19) & 0b11111111) as u8;
        bytes[27] = (self[8] & 0b11111111) as u8;
        bytes[28] = ((self[8] >> 8) & 0b11111111) as u8;
        bytes[29] = ((self[8] >> 16) & 0b11111111) as u8;
        bytes[30] = (((self[8] >> 24) & 0b00000111) |
                     ((self[9] << 3) & 0b11111000)) as u8;
        bytes[31] = ((self[9] >> 5) & 0b11111111) as u8;
        bytes[32] = ((self[9] >> 13) & 0b11111111) as u8;
        bytes[33] = (((self[9] >> 21) & 0b00111111) |
                     ((self[10] << 6) & 0b11000000)) as u8;
        bytes[34] = ((self[10] >> 2) & 0b11111111) as u8;
        bytes[35] = ((self[10] >> 10) & 0b11111111) as u8;
        bytes[36] = ((self[10] >> 18) & 0b11111111) as u8;
        bytes[37] = (((self[10] >> 26) & 0b00000001) |
                     ((self[11] << 1) & 0b11111110)) as u8;
        bytes[38] = ((self[11] >> 7) & 0b11111111) as u8;
        bytes[39] = ((self[11] >> 15) & 0b11111111) as u8;
        bytes[40] = (((self[11] >> 23) & 0b00001111) |
                     ((self[12] << 4) & 0b11110000)) as u8;
        bytes[41] = ((self[12] >> 4) & 0b11111111) as u8;
        bytes[42] = ((self[12] >> 12) & 0b11111111) as u8;
        bytes[43] = (((self[12] >> 20) & 0b01111111) |
                     ((self[13] << 7) & 0b10000000)) as u8;
        bytes[44] = ((self[13] >> 1) & 0b11111111) as u8;
        bytes[45] = ((self[13] >> 9) & 0b11111111) as u8;
        bytes[46] = ((self[13] >> 17) & 0b11111111) as u8;
        bytes[47] = (((self[13] >> 25) & 0b00000011) |
                     ((self[14] << 2) & 0b11111100)) as u8;
        bytes[48] = ((self[14] >> 6) & 0b11111111) as u8;
        bytes[49] = ((self[14] >> 14) & 0b11111111) as u8;
        bytes[50] = (((self[14] >> 22) & 0b00011111) |
                     ((self[15] << 5) & 0b11100000)) as u8;
        bytes[51] = ((self[15] >> 3) & 0b11111111) as u8;
        bytes[52] = ((self[15] >> 11) & 0b11111111) as u8;
        bytes[53] = ((self[15] >> 19) & 0b11111111) as u8;
        bytes[54] = (self[16] & 0b11111111) as u8;
        bytes[55] = ((self[16] >> 8) & 0b11111111) as u8;
        bytes[56] = ((self[16] >> 16) & 0b11111111) as u8;
        bytes[57] = (((self[16] >> 24) & 0b00000111) |
                     ((self[17] << 3) & 0b11111000)) as u8;
        bytes[58] = ((self[17] >> 5) & 0b11111111) as u8;
        bytes[59] = ((self[17] >> 13) & 0b11111111) as u8;
        bytes[60] = (((self[17] >> 21) & 0b00111111) |
                     ((self[18] << 6) & 0b11000000)) as u8;
        bytes[61] = ((self[18] >> 2) & 0b11111111) as u8;
        bytes[62] = ((self[18] >> 10) & 0b11111111) as u8;
        bytes[63] = ((self[18] >> 18) & 0b11111111) as u8;
        bytes[64] = (((self[18] >> 26) & 0b00000001) |
                     ((self[19] << 1) & 0b11111110)) as u8;
        bytes[65] = ((self[19] >> 7) & 0b11111111) as u8;

        bytes
    }

    /// Deserialize a little-endian byte array into a value.  The byte
    /// array must contain a number less than the modulus 2^521 - 1.
    pub fn unpack(bytes: &[u8; 66]) -> Mod_e521_1 {
        let mut out = Mod_e521_1([0u32; 20]);

        out[0] = ((bytes[0] as u32) & 0x000000ff) |
                 (((bytes[1] as u32) << 8) & 0x0000ff00) |
                 (((bytes[2] as u32) << 16) & 0x00ff0000) |
                 (((bytes[3] as u32) << 24) & 0x07000000);
        out[1] = (((bytes[3] as u32) >> 3) & 0x0000001f) |
                 (((bytes[4] as u32) << 5) & 0x00001fe0) |
                 (((bytes[5] as u32) << 13) & 0x001fe000) |
                 (((bytes[6] as u32) << 21) & 0x07e00000);
        out[2] = (((bytes[6] as u32) >> 6) & 0x00000003) |
                 (((bytes[7] as u32) << 2) & 0x000003fc) |
                 (((bytes[8] as u32) << 10) & 0x0003fc00) |
                 (((bytes[9] as u32) << 18) & 0x03fc0000) |
                 (((bytes[10] as u32) << 26) & 0x04000000);
        out[3] = (((bytes[10] as u32) >> 1) & 0x0000007f) |
                 (((bytes[11] as u32) << 7) & 0x00007f80) |
                 (((bytes[12] as u32) << 15) & 0x007f8000) |
                 (((bytes[13] as u32) << 23) & 0x07800000);
        out[4] = (((bytes[13] as u32) >> 4) & 0x0000000f) |
                 (((bytes[14] as u32) << 4) & 0x00000ff0) |
                 (((bytes[15] as u32) << 12) & 0x000ff000) |
                 (((bytes[16] as u32) << 20) & 0x07f00000);
        out[5] = (((bytes[16] as u32) >> 7) & 0x00000001) |
                 (((bytes[17] as u32) << 1) & 0x000001fe) |
                 (((bytes[18] as u32) << 9) & 0x0001fe00) |
                 (((bytes[19] as u32) << 17) & 0x01fe0000) |
                 (((bytes[20] as u32) << 25) & 0x06000000);
        out[6] = (((bytes[20] as u32) >> 2) & 0x0000003f) |
                 (((bytes[21] as u32) << 6) & 0x00003fc0) |
                 (((bytes[22] as u32) << 14) & 0x003fc000) |
                 (((bytes[23] as u32) << 22) & 0x07c00000);
        out[7] = (((bytes[23] as u32) >> 5) & 0x00000007) |
                 (((bytes[24] as u32) << 3) & 0x000007f8) |
                 (((bytes[25] as u32) << 11) & 0x0007f800) |
                 (((bytes[26] as u32) << 19) & 0x07f80000);
        out[8] = ((bytes[27] as u32) & 0x000000ff) |
                 (((bytes[28] as u32) << 8) & 0x0000ff00) |
                 (((bytes[29] as u32) << 16) & 0x00ff0000) |
                 (((bytes[30] as u32) << 24) & 0x07000000);
        out[9] = (((bytes[30] as u32) >> 3) & 0x0000001f) |
                 (((bytes[31] as u32) << 5) & 0x00001fe0) |
                 (((bytes[32] as u32) << 13) & 0x001fe000) |
                 (((bytes[33] as u32) << 21) & 0x07e00000);
        out[10] = (((bytes[33] as u32) >> 6) & 0x00000003) |
                  (((bytes[34] as u32) << 2) & 0x000003fc) |
                  (((bytes[35] as u32) << 10) & 0x0003fc00) |
                  (((bytes[36] as u32) << 18) & 0x03fc0000) |
                  (((bytes[37] as u32) << 26) & 0x04000000);
        out[11] = (((bytes[37] as u32) >> 1) & 0x0000007f) |
                  (((bytes[38] as u32) << 7) & 0x00007f80) |
                  (((bytes[39] as u32) << 15) & 0x007f8000) |
                  (((bytes[40] as u32) << 23) & 0x07800000);
        out[12] = (((bytes[40] as u32) >> 4) & 0x0000000f) |
                  (((bytes[41] as u32) << 4) & 0x00000ff0) |
                  (((bytes[42] as u32) << 12) & 0x000ff000) |
                  (((bytes[43] as u32) << 20) & 0x07f00000);
        out[13] = (((bytes[43] as u32) >> 7) & 0x00000001) |
                  (((bytes[44] as u32) << 1) & 0x000001fe) |
                  (((bytes[45] as u32) << 9) & 0x0001fe00) |
                  (((bytes[46] as u32) << 17) & 0x01fe0000) |
                  (((bytes[47] as u32) << 25) & 0x06000000);
        out[14] = (((bytes[47] as u32) >> 2) & 0x0000003f) |
                  (((bytes[48] as u32) << 6) & 0x00003fc0) |
                  (((bytes[49] as u32) << 14) & 0x003fc000) |
                  (((bytes[50] as u32) << 22) & 0x07c00000);
        out[15] = (((bytes[50] as u32) >> 5) & 0x00000007) |
                  (((bytes[51] as u32) << 3) & 0x000007f8) |
                  (((bytes[52] as u32) << 11) & 0x0007f800) |
                  (((bytes[53] as u32) << 19) & 0x07f80000);
        out[16] = ((bytes[54] as u32) & 0x000000ff) |
                  (((bytes[55] as u32) << 8) & 0x0000ff00) |
                  (((bytes[56] as u32) << 16) & 0x00ff0000) |
                  (((bytes[57] as u32) << 24) & 0x07000000);
        out[17] = (((bytes[57] as u32) >> 3) & 0x0000001f) |
                  (((bytes[58] as u32) << 5) & 0x00001fe0) |
                  (((bytes[59] as u32) << 13) & 0x001fe000) |
                  (((bytes[60] as u32) << 21) & 0x07e00000);
        out[18] = (((bytes[60] as u32) >> 6) & 0x00000003) |
                  (((bytes[61] as u32) << 2) & 0x000003fc) |
                  (((bytes[62] as u32) << 10) & 0x0003fc00) |
                  (((bytes[63] as u32) << 18) & 0x03fc0000) |
                  (((bytes[64] as u32) << 26) & 0x04000000);
        out[19] = (((bytes[64] as u32) >> 1) & 0x0000007f) |
                  (((bytes[65] as u32) << 7) & 0x00007f80);
        out
    }
}

impl IndexMut<usize> for Mod_e521_1 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut u32 {
        let ret: &'a mut u32 = &mut(self.0[idx]);
        ret
    }
}

impl Index<usize> for Mod_e521_1 {
    type Output = u32;

    fn index<'a>(&'a self, idx: usize) -> &'a u32 {
        let ret: &'a u32 = &(self.0[idx]);
        ret
    }
}

impl<'a> Neg for &'a Mod_e521_1 {
    type Output = Mod_e521_1;

    fn neg(self) -> Mod_e521_1 {
        let mut out = self.clone();

        out += &MODULUS;
        out
    }
}

impl<'b> AddAssign<&'b Mod_e521_1> for Mod_e521_1 {
    fn add_assign(&mut self, rhs: &'b Mod_e521_1) {
        let a0: i64 = (self[0] & 0x07ffffff) as i64 |
                       ((self[1] & 0x07ffffff) as i64) << 27;
        let a1: i64 = (self[2] & 0x07ffffff) as i64 |
                       ((self[3] & 0x07ffffff) as i64) << 27;
        let a2: i64 = (self[4] & 0x07ffffff) as i64 |
                       ((self[5] & 0x07ffffff) as i64) << 27;
        let a3: i64 = (self[6] & 0x07ffffff) as i64 |
                       ((self[7] & 0x07ffffff) as i64) << 27;
        let a4: i64 = (self[8] & 0x07ffffff) as i64 |
                       ((self[9] & 0x07ffffff) as i64) << 27;
        let a5: i64 = (self[10] & 0x07ffffff) as i64 |
                       ((self[11] & 0x07ffffff) as i64) << 27;
        let a6: i64 = (self[12] & 0x07ffffff) as i64 |
                       ((self[13] & 0x07ffffff) as i64) << 27;
        let a7: i64 = (self[14] & 0x07ffffff) as i64 |
                       ((self[15] & 0x07ffffff) as i64) << 27;
        let a8: i64 = (self[16] & 0x07ffffff) as i64 |
                       ((self[17] & 0x07ffffff) as i64) << 27;
        let a9: i64 = (self[18] & 0x07ffffff) as i64 |
                       ((self[19] & 0x000000ff) as i64) << 27;

        let b0: i64 = (rhs[0] & 0x07ffffff) as i64 |
                       ((rhs[1] & 0x07ffffff) as i64) << 27;
        let b1: i64 = (rhs[2] & 0x07ffffff) as i64 |
                       ((rhs[3] & 0x07ffffff) as i64) << 27;
        let b2: i64 = (rhs[4] & 0x07ffffff) as i64 |
                       ((rhs[5] & 0x07ffffff) as i64) << 27;
        let b3: i64 = (rhs[6] & 0x07ffffff) as i64 |
                       ((rhs[7] & 0x07ffffff) as i64) << 27;
        let b4: i64 = (rhs[8] & 0x07ffffff) as i64 |
                       ((rhs[9] & 0x07ffffff) as i64) << 27;
        let b5: i64 = (rhs[10] & 0x07ffffff) as i64 |
                       ((rhs[11] & 0x07ffffff) as i64) << 27;
        let b6: i64 = (rhs[12] & 0x07ffffff) as i64 |
                       ((rhs[13] & 0x07ffffff) as i64) << 27;
        let b7: i64 = (rhs[14] & 0x07ffffff) as i64 |
                       ((rhs[15] & 0x07ffffff) as i64) << 27;
        let b8: i64 = (rhs[16] & 0x07ffffff) as i64 |
                       ((rhs[17] & 0x07ffffff) as i64) << 27;
        let b9: i64 = (rhs[18] & 0x07ffffff) as i64 |
                       ((rhs[19] & 0x000000ff) as i64) << 27;

        let cin: i64 = self.carry_out() + rhs.carry_out();
        let s0: i64 = a0 + b0 + cin;
        let c0: i64 = s0 >> 54;
        let s1: i64 = a1 + b1 + c0;
        let c1: i64 = s1 >> 54;
        let s2: i64 = a2 + b2 + c1;
        let c2: i64 = s2 >> 54;
        let s3: i64 = a3 + b3 + c2;
        let c3: i64 = s3 >> 54;
        let s4: i64 = a4 + b4 + c3;
        let c4: i64 = s4 >> 54;
        let s5: i64 = a5 + b5 + c4;
        let c5: i64 = s5 >> 54;
        let s6: i64 = a6 + b6 + c5;
        let c6: i64 = s6 >> 54;
        let s7: i64 = a7 + b7 + c6;
        let c7: i64 = s7 >> 54;
        let s8: i64 = a8 + b8 + c7;
        let c8: i64 = s8 >> 54;
        let s9: i64 = a9 + b9 + c8;

        self[0] = (s0 & 0x07ffffff) as u32;
        self[1] = ((s0 >> 27) & 0x07ffffff) as u32;
        self[2] = (s1 & 0x07ffffff) as u32;
        self[3] = ((s1 >> 27) & 0x07ffffff) as u32;
        self[4] = (s2 & 0x07ffffff) as u32;
        self[5] = ((s2 >> 27) & 0x07ffffff) as u32;
        self[6] = (s3 & 0x07ffffff) as u32;
        self[7] = ((s3 >> 27) & 0x07ffffff) as u32;
        self[8] = (s4 & 0x07ffffff) as u32;
        self[9] = ((s4 >> 27) & 0x07ffffff) as u32;
        self[10] = (s5 & 0x07ffffff) as u32;
        self[11] = ((s5 >> 27) & 0x07ffffff) as u32;
        self[12] = (s6 & 0x07ffffff) as u32;
        self[13] = ((s6 >> 27) & 0x07ffffff) as u32;
        self[14] = (s7 & 0x07ffffff) as u32;
        self[15] = ((s7 >> 27) & 0x07ffffff) as u32;
        self[16] = (s8 & 0x07ffffff) as u32;
        self[17] = ((s8 >> 27) & 0x07ffffff) as u32;
        self[18] = (s9 & 0x07ffffff) as u32;
        self[19] = (s9 >> 27) as u32;
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

impl<'b> SubAssign<&'b Mod_e521_1> for Mod_e521_1 {
    fn sub_assign(&mut self, rhs: &'b Mod_e521_1) {
        let a0: i64 = (self[0] & 0x07ffffff) as i64 |
                      ((self[1] & 0x07ffffff) as i64) << 27;
        let a1: i64 = (self[2] & 0x07ffffff) as i64 |
                      ((self[3] & 0x07ffffff) as i64) << 27;
        let a2: i64 = (self[4] & 0x07ffffff) as i64 |
                      ((self[5] & 0x07ffffff) as i64) << 27;
        let a3: i64 = (self[6] & 0x07ffffff) as i64 |
                      ((self[7] & 0x07ffffff) as i64) << 27;
        let a4: i64 = (self[8] & 0x07ffffff) as i64 |
                      ((self[9] & 0x07ffffff) as i64) << 27;
        let a5: i64 = (self[10] & 0x07ffffff) as i64 |
                      ((self[11] & 0x07ffffff) as i64) << 27;
        let a6: i64 = (self[12] & 0x07ffffff) as i64 |
                      ((self[13] & 0x07ffffff) as i64) << 27;
        let a7: i64 = (self[14] & 0x07ffffff) as i64 |
                      ((self[15] & 0x07ffffff) as i64) << 27;
        let a8: i64 = (self[16] & 0x07ffffff) as i64 |
                      ((self[17] & 0x07ffffff) as i64) << 27;
        let a9: i64 = (self[18] & 0x07ffffff) as i64 |
                       ((self[19] & 0x000000ff) as i64) << 27;

        let b0: i64 = (rhs[0] & 0x07ffffff) as i64 |
                      ((rhs[1] & 0x07ffffff) as i64) << 27;
        let b1: i64 = (rhs[2] & 0x07ffffff) as i64 |
                      ((rhs[3] & 0x07ffffff) as i64) << 27;
        let b2: i64 = (rhs[4] & 0x07ffffff) as i64 |
                      ((rhs[5] & 0x07ffffff) as i64) << 27;
        let b3: i64 = (rhs[6] & 0x07ffffff) as i64 |
                      ((rhs[7] & 0x07ffffff) as i64) << 27;
        let b4: i64 = (rhs[8] & 0x07ffffff) as i64 |
                      ((rhs[9] & 0x07ffffff) as i64) << 27;
        let b5: i64 = (rhs[10] & 0x07ffffff) as i64 |
                      ((rhs[11] & 0x07ffffff) as i64) << 27;
        let b6: i64 = (rhs[12] & 0x07ffffff) as i64 |
                      ((rhs[13] & 0x07ffffff) as i64) << 27;
        let b7: i64 = (rhs[14] & 0x07ffffff) as i64 |
                      ((rhs[15] & 0x07ffffff) as i64) << 27;
        let b8: i64 = (rhs[16] & 0x07ffffff) as i64 |
                      ((rhs[17] & 0x07ffffff) as i64) << 27;
        let b9: i64 = (rhs[18] & 0x07ffffff) as i64 |
                      ((rhs[19] & 0x000000ff) as i64) << 27;

        let cin: i64 = self.carry_out() + rhs.carry_out();
        let s0: i64 = a0 - b0 + cin;
        let c0: i64 = s0 >> 54;
        let s1: i64 = a1 - b1 + c0;
        let c1: i64 = s1 >> 54;
        let s2: i64 = a2 - b2 + c1;
        let c2: i64 = s2 >> 54;
        let s3: i64 = a3 - b3 + c2;
        let c3: i64 = s3 >> 54;
        let s4: i64 = a4 - b4 + c3;
        let c4: i64 = s4 >> 54;
        let s5: i64 = a5 - b5 + c4;
        let c5: i64 = s5 >> 54;
        let s6: i64 = a6 - b6 + c5;
        let c6: i64 = s6 >> 54;
        let s7: i64 = a7 - b7 + c6;
        let c7: i64 = s7 >> 54;
        let s8: i64 = a8 - b8 + c7;
        let c8: i64 = s8 >> 54;
        let s9: i64 = a9 - b9 + c8;

        self[0] = (s0 & 0x07ffffff) as u32;
        self[1] = ((s0 >> 27) & 0x07ffffff) as u32;
        self[2] = (s1 & 0x07ffffff) as u32;
        self[3] = ((s1 >> 27) & 0x07ffffff) as u32;
        self[4] = (s2 & 0x07ffffff) as u32;
        self[5] = ((s2 >> 27) & 0x07ffffff) as u32;
        self[6] = (s3 & 0x07ffffff) as u32;
        self[7] = ((s3 >> 27) & 0x07ffffff) as u32;
        self[8] = (s4 & 0x07ffffff) as u32;
        self[9] = ((s4 >> 27) & 0x07ffffff) as u32;
        self[10] = (s5 & 0x07ffffff) as u32;
        self[11] = ((s5 >> 27) & 0x07ffffff) as u32;
        self[12] = (s6 & 0x07ffffff) as u32;
        self[13] = ((s6 >> 27) & 0x07ffffff) as u32;
        self[14] = (s7 & 0x07ffffff) as u32;
        self[15] = ((s7 >> 27) & 0x07ffffff) as u32;
        self[16] = (s8 & 0x07ffffff) as u32;
        self[17] = ((s8 >> 27) & 0x07ffffff) as u32;
        self[18] = (s9 & 0x07ffffff) as u32;
        self[19] = (s9 >> 27) as u32;
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

impl<'b> MulAssign<&'b Mod_e521_1> for Mod_e521_1 {
    fn mul_assign(&mut self, rhs: &'b Mod_e521_1) {
        let a0: u64 = self[0] as u64;
        let a1: u64 = self[1] as u64;
        let a2: u64 = self[2] as u64;
        let a3: u64 = self[3] as u64;
        let a4: u64 = self[4] as u64;
        let a5: u64 = self[5] as u64;
        let a6: u64 = self[6] as u64;
        let a7: u64 = self[7] as u64;
        let a8: u64 = self[8] as u64;
        let a9: u64 = self[9] as u64;
        let a10: u64 = self[10] as u64;
        let a11: u64 = self[11] as u64;
        let a12: u64 = self[12] as u64;
        let a13: u64 = self[13] as u64;
        let a14: u64 = self[14] as u64;
        let a15: u64 = self[15] as u64;
        let a16: u64 = self[16] as u64;
        let a17: u64 = self[17] as u64;
        let a18: u64 = self[18] as u64;
        let a19: u64 = self[19] as u64;

        let b0: u64 = rhs[0] as u64;
        let b1: u64 = rhs[1] as u64;
        let b2: u64 = rhs[2] as u64;
        let b3: u64 = rhs[3] as u64;
        let b4: u64 = rhs[4] as u64;
        let b5: u64 = rhs[5] as u64;
        let b6: u64 = rhs[6] as u64;
        let b7: u64 = rhs[7] as u64;
        let b8: u64 = rhs[8] as u64;
        let b9: u64 = rhs[9] as u64;
        let b10: u64 = rhs[10] as u64;
        let b11: u64 = rhs[11] as u64;
        let b12: u64 = rhs[12] as u64;
        let b13: u64 = rhs[13] as u64;
        let b14: u64 = rhs[14] as u64;
        let b15: u64 = rhs[15] as u64;
        let b16: u64 = rhs[16] as u64;
        let b17: u64 = rhs[17] as u64;
        let b18: u64 = rhs[18] as u64;
        let b19: u64 = rhs[19] as u64;

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

        // 27-bit products
        let p_0_0: u32 = (m_0_0 & 0x07ffffff) as u32;
        let p_0_1: u32 = (m_0_1 & 0x07ffffff) as u32;
        let p_0_2: u32 = (m_0_2 & 0x07ffffff) as u32;
        let p_0_3: u32 = (m_0_3 & 0x07ffffff) as u32;
        let p_0_4: u32 = (m_0_4 & 0x07ffffff) as u32;
        let p_0_5: u32 = (m_0_5 & 0x07ffffff) as u32;
        let p_0_6: u32 = (m_0_6 & 0x07ffffff) as u32;
        let p_0_7: u32 = (m_0_7 & 0x07ffffff) as u32;
        let p_0_8: u32 = (m_0_8 & 0x07ffffff) as u32;
        let p_0_9: u32 = (m_0_9 & 0x07ffffff) as u32;
        let p_0_10: u32 = (m_0_10 & 0x07ffffff) as u32;
        let p_0_11: u32 = (m_0_11 & 0x07ffffff) as u32;
        let p_0_12: u32 = (m_0_12 & 0x07ffffff) as u32;
        let p_0_13: u32 = (m_0_13 & 0x07ffffff) as u32;
        let p_0_14: u32 = (m_0_14 & 0x07ffffff) as u32;
        let p_0_15: u32 = (m_0_15 & 0x07ffffff) as u32;
        let p_0_16: u32 = (m_0_16 & 0x07ffffff) as u32;
        let p_0_17: u32 = (m_0_17 & 0x07ffffff) as u32;
        let p_0_18: u32 = (m_0_18 & 0x07ffffff) as u32;
        let p_0_19: u32 = (m_0_19 & 0x07ffffff) as u32;
        let p_1_0: u32 = (m_1_0 & 0x07ffffff) as u32;
        let p_1_1: u32 = (m_1_1 & 0x07ffffff) as u32;
        let p_1_2: u32 = (m_1_2 & 0x07ffffff) as u32;
        let p_1_3: u32 = (m_1_3 & 0x07ffffff) as u32;
        let p_1_4: u32 = (m_1_4 & 0x07ffffff) as u32;
        let p_1_5: u32 = (m_1_5 & 0x07ffffff) as u32;
        let p_1_6: u32 = (m_1_6 & 0x07ffffff) as u32;
        let p_1_7: u32 = (m_1_7 & 0x07ffffff) as u32;
        let p_1_8: u32 = (m_1_8 & 0x07ffffff) as u32;
        let p_1_9: u32 = (m_1_9 & 0x07ffffff) as u32;
        let p_1_10: u32 = (m_1_10 & 0x07ffffff) as u32;
        let p_1_11: u32 = (m_1_11 & 0x07ffffff) as u32;
        let p_1_12: u32 = (m_1_12 & 0x07ffffff) as u32;
        let p_1_13: u32 = (m_1_13 & 0x07ffffff) as u32;
        let p_1_14: u32 = (m_1_14 & 0x07ffffff) as u32;
        let p_1_15: u32 = (m_1_15 & 0x07ffffff) as u32;
        let p_1_16: u32 = (m_1_16 & 0x07ffffff) as u32;
        let p_1_17: u32 = (m_1_17 & 0x07ffffff) as u32;
        let p_1_18: u32 = (m_1_18 & 0x07ffffff) as u32;
        let p_1_19: u32 = (m_1_19 & 0x07ffffff) as u32;
        let p_2_0: u32 = (m_2_0 & 0x07ffffff) as u32;
        let p_2_1: u32 = (m_2_1 & 0x07ffffff) as u32;
        let p_2_2: u32 = (m_2_2 & 0x07ffffff) as u32;
        let p_2_3: u32 = (m_2_3 & 0x07ffffff) as u32;
        let p_2_4: u32 = (m_2_4 & 0x07ffffff) as u32;
        let p_2_5: u32 = (m_2_5 & 0x07ffffff) as u32;
        let p_2_6: u32 = (m_2_6 & 0x07ffffff) as u32;
        let p_2_7: u32 = (m_2_7 & 0x07ffffff) as u32;
        let p_2_8: u32 = (m_2_8 & 0x07ffffff) as u32;
        let p_2_9: u32 = (m_2_9 & 0x07ffffff) as u32;
        let p_2_10: u32 = (m_2_10 & 0x07ffffff) as u32;
        let p_2_11: u32 = (m_2_11 & 0x07ffffff) as u32;
        let p_2_12: u32 = (m_2_12 & 0x07ffffff) as u32;
        let p_2_13: u32 = (m_2_13 & 0x07ffffff) as u32;
        let p_2_14: u32 = (m_2_14 & 0x07ffffff) as u32;
        let p_2_15: u32 = (m_2_15 & 0x07ffffff) as u32;
        let p_2_16: u32 = (m_2_16 & 0x07ffffff) as u32;
        let p_2_17: u32 = (m_2_17 & 0x07ffffff) as u32;
        let p_2_18: u32 = (m_2_18 & 0x07ffffff) as u32;
        let p_2_19: u32 = (m_2_19 & 0x07ffffff) as u32;
        let p_3_0: u32 = (m_3_0 & 0x07ffffff) as u32;
        let p_3_1: u32 = (m_3_1 & 0x07ffffff) as u32;
        let p_3_2: u32 = (m_3_2 & 0x07ffffff) as u32;
        let p_3_3: u32 = (m_3_3 & 0x07ffffff) as u32;
        let p_3_4: u32 = (m_3_4 & 0x07ffffff) as u32;
        let p_3_5: u32 = (m_3_5 & 0x07ffffff) as u32;
        let p_3_6: u32 = (m_3_6 & 0x07ffffff) as u32;
        let p_3_7: u32 = (m_3_7 & 0x07ffffff) as u32;
        let p_3_8: u32 = (m_3_8 & 0x07ffffff) as u32;
        let p_3_9: u32 = (m_3_9 & 0x07ffffff) as u32;
        let p_3_10: u32 = (m_3_10 & 0x07ffffff) as u32;
        let p_3_11: u32 = (m_3_11 & 0x07ffffff) as u32;
        let p_3_12: u32 = (m_3_12 & 0x07ffffff) as u32;
        let p_3_13: u32 = (m_3_13 & 0x07ffffff) as u32;
        let p_3_14: u32 = (m_3_14 & 0x07ffffff) as u32;
        let p_3_15: u32 = (m_3_15 & 0x07ffffff) as u32;
        let p_3_16: u32 = (m_3_16 & 0x07ffffff) as u32;
        let p_3_17: u32 = (m_3_17 & 0x07ffffff) as u32;
        let p_3_18: u32 = (m_3_18 & 0x07ffffff) as u32;
        let p_3_19: u32 = (m_3_19 & 0x07ffffff) as u32;
        let p_4_0: u32 = (m_4_0 & 0x07ffffff) as u32;
        let p_4_1: u32 = (m_4_1 & 0x07ffffff) as u32;
        let p_4_2: u32 = (m_4_2 & 0x07ffffff) as u32;
        let p_4_3: u32 = (m_4_3 & 0x07ffffff) as u32;
        let p_4_4: u32 = (m_4_4 & 0x07ffffff) as u32;
        let p_4_5: u32 = (m_4_5 & 0x07ffffff) as u32;
        let p_4_6: u32 = (m_4_6 & 0x07ffffff) as u32;
        let p_4_7: u32 = (m_4_7 & 0x07ffffff) as u32;
        let p_4_8: u32 = (m_4_8 & 0x07ffffff) as u32;
        let p_4_9: u32 = (m_4_9 & 0x07ffffff) as u32;
        let p_4_10: u32 = (m_4_10 & 0x07ffffff) as u32;
        let p_4_11: u32 = (m_4_11 & 0x07ffffff) as u32;
        let p_4_12: u32 = (m_4_12 & 0x07ffffff) as u32;
        let p_4_13: u32 = (m_4_13 & 0x07ffffff) as u32;
        let p_4_14: u32 = (m_4_14 & 0x07ffffff) as u32;
        let p_4_15: u32 = (m_4_15 & 0x07ffffff) as u32;
        let p_4_16: u32 = (m_4_16 & 0x07ffffff) as u32;
        let p_4_17: u32 = (m_4_17 & 0x07ffffff) as u32;
        let p_4_18: u32 = (m_4_18 & 0x07ffffff) as u32;
        let p_4_19: u32 = (m_4_19 & 0x07ffffff) as u32;
        let p_5_0: u32 = (m_5_0 & 0x07ffffff) as u32;
        let p_5_1: u32 = (m_5_1 & 0x07ffffff) as u32;
        let p_5_2: u32 = (m_5_2 & 0x07ffffff) as u32;
        let p_5_3: u32 = (m_5_3 & 0x07ffffff) as u32;
        let p_5_4: u32 = (m_5_4 & 0x07ffffff) as u32;
        let p_5_5: u32 = (m_5_5 & 0x07ffffff) as u32;
        let p_5_6: u32 = (m_5_6 & 0x07ffffff) as u32;
        let p_5_7: u32 = (m_5_7 & 0x07ffffff) as u32;
        let p_5_8: u32 = (m_5_8 & 0x07ffffff) as u32;
        let p_5_9: u32 = (m_5_9 & 0x07ffffff) as u32;
        let p_5_10: u32 = (m_5_10 & 0x07ffffff) as u32;
        let p_5_11: u32 = (m_5_11 & 0x07ffffff) as u32;
        let p_5_12: u32 = (m_5_12 & 0x07ffffff) as u32;
        let p_5_13: u32 = (m_5_13 & 0x07ffffff) as u32;
        let p_5_14: u32 = (m_5_14 & 0x07ffffff) as u32;
        let p_5_15: u32 = (m_5_15 & 0x07ffffff) as u32;
        let p_5_16: u32 = (m_5_16 & 0x07ffffff) as u32;
        let p_5_17: u32 = (m_5_17 & 0x07ffffff) as u32;
        let p_5_18: u32 = (m_5_18 & 0x07ffffff) as u32;
        let p_5_19: u32 = (m_5_19 & 0x07ffffff) as u32;
        let p_6_0: u32 = (m_6_0 & 0x07ffffff) as u32;
        let p_6_1: u32 = (m_6_1 & 0x07ffffff) as u32;
        let p_6_2: u32 = (m_6_2 & 0x07ffffff) as u32;
        let p_6_3: u32 = (m_6_3 & 0x07ffffff) as u32;
        let p_6_4: u32 = (m_6_4 & 0x07ffffff) as u32;
        let p_6_5: u32 = (m_6_5 & 0x07ffffff) as u32;
        let p_6_6: u32 = (m_6_6 & 0x07ffffff) as u32;
        let p_6_7: u32 = (m_6_7 & 0x07ffffff) as u32;
        let p_6_8: u32 = (m_6_8 & 0x07ffffff) as u32;
        let p_6_9: u32 = (m_6_9 & 0x07ffffff) as u32;
        let p_6_10: u32 = (m_6_10 & 0x07ffffff) as u32;
        let p_6_11: u32 = (m_6_11 & 0x07ffffff) as u32;
        let p_6_12: u32 = (m_6_12 & 0x07ffffff) as u32;
        let p_6_13: u32 = (m_6_13 & 0x07ffffff) as u32;
        let p_6_14: u32 = (m_6_14 & 0x07ffffff) as u32;
        let p_6_15: u32 = (m_6_15 & 0x07ffffff) as u32;
        let p_6_16: u32 = (m_6_16 & 0x07ffffff) as u32;
        let p_6_17: u32 = (m_6_17 & 0x07ffffff) as u32;
        let p_6_18: u32 = (m_6_18 & 0x07ffffff) as u32;
        let p_6_19: u32 = (m_6_19 & 0x07ffffff) as u32;
        let p_7_0: u32 = (m_7_0 & 0x07ffffff) as u32;
        let p_7_1: u32 = (m_7_1 & 0x07ffffff) as u32;
        let p_7_2: u32 = (m_7_2 & 0x07ffffff) as u32;
        let p_7_3: u32 = (m_7_3 & 0x07ffffff) as u32;
        let p_7_4: u32 = (m_7_4 & 0x07ffffff) as u32;
        let p_7_5: u32 = (m_7_5 & 0x07ffffff) as u32;
        let p_7_6: u32 = (m_7_6 & 0x07ffffff) as u32;
        let p_7_7: u32 = (m_7_7 & 0x07ffffff) as u32;
        let p_7_8: u32 = (m_7_8 & 0x07ffffff) as u32;
        let p_7_9: u32 = (m_7_9 & 0x07ffffff) as u32;
        let p_7_10: u32 = (m_7_10 & 0x07ffffff) as u32;
        let p_7_11: u32 = (m_7_11 & 0x07ffffff) as u32;
        let p_7_12: u32 = (m_7_12 & 0x07ffffff) as u32;
        let p_7_13: u32 = (m_7_13 & 0x07ffffff) as u32;
        let p_7_14: u32 = (m_7_14 & 0x07ffffff) as u32;
        let p_7_15: u32 = (m_7_15 & 0x07ffffff) as u32;
        let p_7_16: u32 = (m_7_16 & 0x07ffffff) as u32;
        let p_7_17: u32 = (m_7_17 & 0x07ffffff) as u32;
        let p_7_18: u32 = (m_7_18 & 0x07ffffff) as u32;
        let p_7_19: u32 = (m_7_19 & 0x07ffffff) as u32;
        let p_8_0: u32 = (m_8_0 & 0x07ffffff) as u32;
        let p_8_1: u32 = (m_8_1 & 0x07ffffff) as u32;
        let p_8_2: u32 = (m_8_2 & 0x07ffffff) as u32;
        let p_8_3: u32 = (m_8_3 & 0x07ffffff) as u32;
        let p_8_4: u32 = (m_8_4 & 0x07ffffff) as u32;
        let p_8_5: u32 = (m_8_5 & 0x07ffffff) as u32;
        let p_8_6: u32 = (m_8_6 & 0x07ffffff) as u32;
        let p_8_7: u32 = (m_8_7 & 0x07ffffff) as u32;
        let p_8_8: u32 = (m_8_8 & 0x07ffffff) as u32;
        let p_8_9: u32 = (m_8_9 & 0x07ffffff) as u32;
        let p_8_10: u32 = (m_8_10 & 0x07ffffff) as u32;
        let p_8_11: u32 = (m_8_11 & 0x07ffffff) as u32;
        let p_8_12: u32 = (m_8_12 & 0x07ffffff) as u32;
        let p_8_13: u32 = (m_8_13 & 0x07ffffff) as u32;
        let p_8_14: u32 = (m_8_14 & 0x07ffffff) as u32;
        let p_8_15: u32 = (m_8_15 & 0x07ffffff) as u32;
        let p_8_16: u32 = (m_8_16 & 0x07ffffff) as u32;
        let p_8_17: u32 = (m_8_17 & 0x07ffffff) as u32;
        let p_8_18: u32 = (m_8_18 & 0x07ffffff) as u32;
        let p_8_19: u32 = (m_8_19 & 0x07ffffff) as u32;
        let p_9_0: u32 = (m_9_0 & 0x07ffffff) as u32;
        let p_9_1: u32 = (m_9_1 & 0x07ffffff) as u32;
        let p_9_2: u32 = (m_9_2 & 0x07ffffff) as u32;
        let p_9_3: u32 = (m_9_3 & 0x07ffffff) as u32;
        let p_9_4: u32 = (m_9_4 & 0x07ffffff) as u32;
        let p_9_5: u32 = (m_9_5 & 0x07ffffff) as u32;
        let p_9_6: u32 = (m_9_6 & 0x07ffffff) as u32;
        let p_9_7: u32 = (m_9_7 & 0x07ffffff) as u32;
        let p_9_8: u32 = (m_9_8 & 0x07ffffff) as u32;
        let p_9_9: u32 = (m_9_9 & 0x07ffffff) as u32;
        let p_9_10: u32 = (m_9_10 & 0x07ffffff) as u32;
        let p_9_11: u32 = (m_9_11 & 0x07ffffff) as u32;
        let p_9_12: u32 = (m_9_12 & 0x07ffffff) as u32;
        let p_9_13: u32 = (m_9_13 & 0x07ffffff) as u32;
        let p_9_14: u32 = (m_9_14 & 0x07ffffff) as u32;
        let p_9_15: u32 = (m_9_15 & 0x07ffffff) as u32;
        let p_9_16: u32 = (m_9_16 & 0x07ffffff) as u32;
        let p_9_17: u32 = (m_9_17 & 0x07ffffff) as u32;
        let p_9_18: u32 = (m_9_18 & 0x07ffffff) as u32;
        let p_9_19: u32 = (m_9_19 & 0x07ffffff) as u32;
        let p_10_0: u32 = (m_10_0 & 0x07ffffff) as u32;
        let p_10_1: u32 = (m_10_1 & 0x07ffffff) as u32;
        let p_10_2: u32 = (m_10_2 & 0x07ffffff) as u32;
        let p_10_3: u32 = (m_10_3 & 0x07ffffff) as u32;
        let p_10_4: u32 = (m_10_4 & 0x07ffffff) as u32;
        let p_10_5: u32 = (m_10_5 & 0x07ffffff) as u32;
        let p_10_6: u32 = (m_10_6 & 0x07ffffff) as u32;
        let p_10_7: u32 = (m_10_7 & 0x07ffffff) as u32;
        let p_10_8: u32 = (m_10_8 & 0x07ffffff) as u32;
        let p_10_9: u32 = (m_10_9 & 0x07ffffff) as u32;
        let p_10_10: u32 = (m_10_10 & 0x07ffffff) as u32;
        let p_10_11: u32 = (m_10_11 & 0x07ffffff) as u32;
        let p_10_12: u32 = (m_10_12 & 0x07ffffff) as u32;
        let p_10_13: u32 = (m_10_13 & 0x07ffffff) as u32;
        let p_10_14: u32 = (m_10_14 & 0x07ffffff) as u32;
        let p_10_15: u32 = (m_10_15 & 0x07ffffff) as u32;
        let p_10_16: u32 = (m_10_16 & 0x07ffffff) as u32;
        let p_10_17: u32 = (m_10_17 & 0x07ffffff) as u32;
        let p_10_18: u32 = (m_10_18 & 0x07ffffff) as u32;
        let p_10_19: u32 = (m_10_19 & 0x07ffffff) as u32;
        let p_11_0: u32 = (m_11_0 & 0x07ffffff) as u32;
        let p_11_1: u32 = (m_11_1 & 0x07ffffff) as u32;
        let p_11_2: u32 = (m_11_2 & 0x07ffffff) as u32;
        let p_11_3: u32 = (m_11_3 & 0x07ffffff) as u32;
        let p_11_4: u32 = (m_11_4 & 0x07ffffff) as u32;
        let p_11_5: u32 = (m_11_5 & 0x07ffffff) as u32;
        let p_11_6: u32 = (m_11_6 & 0x07ffffff) as u32;
        let p_11_7: u32 = (m_11_7 & 0x07ffffff) as u32;
        let p_11_8: u32 = (m_11_8 & 0x07ffffff) as u32;
        let p_11_9: u32 = (m_11_9 & 0x07ffffff) as u32;
        let p_11_10: u32 = (m_11_10 & 0x07ffffff) as u32;
        let p_11_11: u32 = (m_11_11 & 0x07ffffff) as u32;
        let p_11_12: u32 = (m_11_12 & 0x07ffffff) as u32;
        let p_11_13: u32 = (m_11_13 & 0x07ffffff) as u32;
        let p_11_14: u32 = (m_11_14 & 0x07ffffff) as u32;
        let p_11_15: u32 = (m_11_15 & 0x07ffffff) as u32;
        let p_11_16: u32 = (m_11_16 & 0x07ffffff) as u32;
        let p_11_17: u32 = (m_11_17 & 0x07ffffff) as u32;
        let p_11_18: u32 = (m_11_18 & 0x07ffffff) as u32;
        let p_11_19: u32 = (m_11_19 & 0x07ffffff) as u32;
        let p_12_0: u32 = (m_12_0 & 0x07ffffff) as u32;
        let p_12_1: u32 = (m_12_1 & 0x07ffffff) as u32;
        let p_12_2: u32 = (m_12_2 & 0x07ffffff) as u32;
        let p_12_3: u32 = (m_12_3 & 0x07ffffff) as u32;
        let p_12_4: u32 = (m_12_4 & 0x07ffffff) as u32;
        let p_12_5: u32 = (m_12_5 & 0x07ffffff) as u32;
        let p_12_6: u32 = (m_12_6 & 0x07ffffff) as u32;
        let p_12_7: u32 = (m_12_7 & 0x07ffffff) as u32;
        let p_12_8: u32 = (m_12_8 & 0x07ffffff) as u32;
        let p_12_9: u32 = (m_12_9 & 0x07ffffff) as u32;
        let p_12_10: u32 = (m_12_10 & 0x07ffffff) as u32;
        let p_12_11: u32 = (m_12_11 & 0x07ffffff) as u32;
        let p_12_12: u32 = (m_12_12 & 0x07ffffff) as u32;
        let p_12_13: u32 = (m_12_13 & 0x07ffffff) as u32;
        let p_12_14: u32 = (m_12_14 & 0x07ffffff) as u32;
        let p_12_15: u32 = (m_12_15 & 0x07ffffff) as u32;
        let p_12_16: u32 = (m_12_16 & 0x07ffffff) as u32;
        let p_12_17: u32 = (m_12_17 & 0x07ffffff) as u32;
        let p_12_18: u32 = (m_12_18 & 0x07ffffff) as u32;
        let p_12_19: u32 = (m_12_19 & 0x07ffffff) as u32;
        let p_13_0: u32 = (m_13_0 & 0x07ffffff) as u32;
        let p_13_1: u32 = (m_13_1 & 0x07ffffff) as u32;
        let p_13_2: u32 = (m_13_2 & 0x07ffffff) as u32;
        let p_13_3: u32 = (m_13_3 & 0x07ffffff) as u32;
        let p_13_4: u32 = (m_13_4 & 0x07ffffff) as u32;
        let p_13_5: u32 = (m_13_5 & 0x07ffffff) as u32;
        let p_13_6: u32 = (m_13_6 & 0x07ffffff) as u32;
        let p_13_7: u32 = (m_13_7 & 0x07ffffff) as u32;
        let p_13_8: u32 = (m_13_8 & 0x07ffffff) as u32;
        let p_13_9: u32 = (m_13_9 & 0x07ffffff) as u32;
        let p_13_10: u32 = (m_13_10 & 0x07ffffff) as u32;
        let p_13_11: u32 = (m_13_11 & 0x07ffffff) as u32;
        let p_13_12: u32 = (m_13_12 & 0x07ffffff) as u32;
        let p_13_13: u32 = (m_13_13 & 0x07ffffff) as u32;
        let p_13_14: u32 = (m_13_14 & 0x07ffffff) as u32;
        let p_13_15: u32 = (m_13_15 & 0x07ffffff) as u32;
        let p_13_16: u32 = (m_13_16 & 0x07ffffff) as u32;
        let p_13_17: u32 = (m_13_17 & 0x07ffffff) as u32;
        let p_13_18: u32 = (m_13_18 & 0x07ffffff) as u32;
        let p_13_19: u32 = (m_13_19 & 0x07ffffff) as u32;
        let p_14_0: u32 = (m_14_0 & 0x07ffffff) as u32;
        let p_14_1: u32 = (m_14_1 & 0x07ffffff) as u32;
        let p_14_2: u32 = (m_14_2 & 0x07ffffff) as u32;
        let p_14_3: u32 = (m_14_3 & 0x07ffffff) as u32;
        let p_14_4: u32 = (m_14_4 & 0x07ffffff) as u32;
        let p_14_5: u32 = (m_14_5 & 0x07ffffff) as u32;
        let p_14_6: u32 = (m_14_6 & 0x07ffffff) as u32;
        let p_14_7: u32 = (m_14_7 & 0x07ffffff) as u32;
        let p_14_8: u32 = (m_14_8 & 0x07ffffff) as u32;
        let p_14_9: u32 = (m_14_9 & 0x07ffffff) as u32;
        let p_14_10: u32 = (m_14_10 & 0x07ffffff) as u32;
        let p_14_11: u32 = (m_14_11 & 0x07ffffff) as u32;
        let p_14_12: u32 = (m_14_12 & 0x07ffffff) as u32;
        let p_14_13: u32 = (m_14_13 & 0x07ffffff) as u32;
        let p_14_14: u32 = (m_14_14 & 0x07ffffff) as u32;
        let p_14_15: u32 = (m_14_15 & 0x07ffffff) as u32;
        let p_14_16: u32 = (m_14_16 & 0x07ffffff) as u32;
        let p_14_17: u32 = (m_14_17 & 0x07ffffff) as u32;
        let p_14_18: u32 = (m_14_18 & 0x07ffffff) as u32;
        let p_14_19: u32 = (m_14_19 & 0x07ffffff) as u32;
        let p_15_0: u32 = (m_15_0 & 0x07ffffff) as u32;
        let p_15_1: u32 = (m_15_1 & 0x07ffffff) as u32;
        let p_15_2: u32 = (m_15_2 & 0x07ffffff) as u32;
        let p_15_3: u32 = (m_15_3 & 0x07ffffff) as u32;
        let p_15_4: u32 = (m_15_4 & 0x07ffffff) as u32;
        let p_15_5: u32 = (m_15_5 & 0x07ffffff) as u32;
        let p_15_6: u32 = (m_15_6 & 0x07ffffff) as u32;
        let p_15_7: u32 = (m_15_7 & 0x07ffffff) as u32;
        let p_15_8: u32 = (m_15_8 & 0x07ffffff) as u32;
        let p_15_9: u32 = (m_15_9 & 0x07ffffff) as u32;
        let p_15_10: u32 = (m_15_10 & 0x07ffffff) as u32;
        let p_15_11: u32 = (m_15_11 & 0x07ffffff) as u32;
        let p_15_12: u32 = (m_15_12 & 0x07ffffff) as u32;
        let p_15_13: u32 = (m_15_13 & 0x07ffffff) as u32;
        let p_15_14: u32 = (m_15_14 & 0x07ffffff) as u32;
        let p_15_15: u32 = (m_15_15 & 0x07ffffff) as u32;
        let p_15_16: u32 = (m_15_16 & 0x07ffffff) as u32;
        let p_15_17: u32 = (m_15_17 & 0x07ffffff) as u32;
        let p_15_18: u32 = (m_15_18 & 0x07ffffff) as u32;
        let p_15_19: u32 = (m_15_19 & 0x07ffffff) as u32;
        let p_16_0: u32 = (m_16_0 & 0x07ffffff) as u32;
        let p_16_1: u32 = (m_16_1 & 0x07ffffff) as u32;
        let p_16_2: u32 = (m_16_2 & 0x07ffffff) as u32;
        let p_16_3: u32 = (m_16_3 & 0x07ffffff) as u32;
        let p_16_4: u32 = (m_16_4 & 0x07ffffff) as u32;
        let p_16_5: u32 = (m_16_5 & 0x07ffffff) as u32;
        let p_16_6: u32 = (m_16_6 & 0x07ffffff) as u32;
        let p_16_7: u32 = (m_16_7 & 0x07ffffff) as u32;
        let p_16_8: u32 = (m_16_8 & 0x07ffffff) as u32;
        let p_16_9: u32 = (m_16_9 & 0x07ffffff) as u32;
        let p_16_10: u32 = (m_16_10 & 0x07ffffff) as u32;
        let p_16_11: u32 = (m_16_11 & 0x07ffffff) as u32;
        let p_16_12: u32 = (m_16_12 & 0x07ffffff) as u32;
        let p_16_13: u32 = (m_16_13 & 0x07ffffff) as u32;
        let p_16_14: u32 = (m_16_14 & 0x07ffffff) as u32;
        let p_16_15: u32 = (m_16_15 & 0x07ffffff) as u32;
        let p_16_16: u32 = (m_16_16 & 0x07ffffff) as u32;
        let p_16_17: u32 = (m_16_17 & 0x07ffffff) as u32;
        let p_16_18: u32 = (m_16_18 & 0x07ffffff) as u32;
        let p_16_19: u32 = (m_16_19 & 0x07ffffff) as u32;
        let p_17_0: u32 = (m_17_0 & 0x07ffffff) as u32;
        let p_17_1: u32 = (m_17_1 & 0x07ffffff) as u32;
        let p_17_2: u32 = (m_17_2 & 0x07ffffff) as u32;
        let p_17_3: u32 = (m_17_3 & 0x07ffffff) as u32;
        let p_17_4: u32 = (m_17_4 & 0x07ffffff) as u32;
        let p_17_5: u32 = (m_17_5 & 0x07ffffff) as u32;
        let p_17_6: u32 = (m_17_6 & 0x07ffffff) as u32;
        let p_17_7: u32 = (m_17_7 & 0x07ffffff) as u32;
        let p_17_8: u32 = (m_17_8 & 0x07ffffff) as u32;
        let p_17_9: u32 = (m_17_9 & 0x07ffffff) as u32;
        let p_17_10: u32 = (m_17_10 & 0x07ffffff) as u32;
        let p_17_11: u32 = (m_17_11 & 0x07ffffff) as u32;
        let p_17_12: u32 = (m_17_12 & 0x07ffffff) as u32;
        let p_17_13: u32 = (m_17_13 & 0x07ffffff) as u32;
        let p_17_14: u32 = (m_17_14 & 0x07ffffff) as u32;
        let p_17_15: u32 = (m_17_15 & 0x07ffffff) as u32;
        let p_17_16: u32 = (m_17_16 & 0x07ffffff) as u32;
        let p_17_17: u32 = (m_17_17 & 0x07ffffff) as u32;
        let p_17_18: u32 = (m_17_18 & 0x07ffffff) as u32;
        let p_17_19: u32 = (m_17_19 & 0x07ffffff) as u32;
        let p_18_0: u32 = (m_18_0 & 0x07ffffff) as u32;
        let p_18_1: u32 = (m_18_1 & 0x07ffffff) as u32;
        let p_18_2: u32 = (m_18_2 & 0x07ffffff) as u32;
        let p_18_3: u32 = (m_18_3 & 0x07ffffff) as u32;
        let p_18_4: u32 = (m_18_4 & 0x07ffffff) as u32;
        let p_18_5: u32 = (m_18_5 & 0x07ffffff) as u32;
        let p_18_6: u32 = (m_18_6 & 0x07ffffff) as u32;
        let p_18_7: u32 = (m_18_7 & 0x07ffffff) as u32;
        let p_18_8: u32 = (m_18_8 & 0x07ffffff) as u32;
        let p_18_9: u32 = (m_18_9 & 0x07ffffff) as u32;
        let p_18_10: u32 = (m_18_10 & 0x07ffffff) as u32;
        let p_18_11: u32 = (m_18_11 & 0x07ffffff) as u32;
        let p_18_12: u32 = (m_18_12 & 0x07ffffff) as u32;
        let p_18_13: u32 = (m_18_13 & 0x07ffffff) as u32;
        let p_18_14: u32 = (m_18_14 & 0x07ffffff) as u32;
        let p_18_15: u32 = (m_18_15 & 0x07ffffff) as u32;
        let p_18_16: u32 = (m_18_16 & 0x07ffffff) as u32;
        let p_18_17: u32 = (m_18_17 & 0x07ffffff) as u32;
        let p_18_18: u32 = (m_18_18 & 0x07ffffff) as u32;
        let p_18_19: u32 = (m_18_19 & 0x07ffffff) as u32;
        let p_19_0: u32 = (m_19_0 & 0x07ffffff) as u32;
        let p_19_1: u32 = (m_19_1 & 0x07ffffff) as u32;
        let p_19_2: u32 = (m_19_2 & 0x07ffffff) as u32;
        let p_19_3: u32 = (m_19_3 & 0x07ffffff) as u32;
        let p_19_4: u32 = (m_19_4 & 0x07ffffff) as u32;
        let p_19_5: u32 = (m_19_5 & 0x07ffffff) as u32;
        let p_19_6: u32 = (m_19_6 & 0x07ffffff) as u32;
        let p_19_7: u32 = (m_19_7 & 0x07ffffff) as u32;
        let p_19_8: u32 = (m_19_8 & 0x07ffffff) as u32;
        let p_19_9: u32 = (m_19_9 & 0x07ffffff) as u32;
        let p_19_10: u32 = (m_19_10 & 0x07ffffff) as u32;
        let p_19_11: u32 = (m_19_11 & 0x07ffffff) as u32;
        let p_19_12: u32 = (m_19_12 & 0x07ffffff) as u32;
        let p_19_13: u32 = (m_19_13 & 0x07ffffff) as u32;
        let p_19_14: u32 = (m_19_14 & 0x07ffffff) as u32;
        let p_19_15: u32 = (m_19_15 & 0x07ffffff) as u32;
        let p_19_16: u32 = (m_19_16 & 0x07ffffff) as u32;
        let p_19_17: u32 = (m_19_17 & 0x07ffffff) as u32;
        let p_19_18: u32 = (m_19_18 & 0x07ffffff) as u32;
        let p_19_19: u32 = (m_19_19 & 0x07ffffff) as u32;

        // Overflow values
        let o_0_0: u32 = ((m_0_0 >> 27) & 0x07ffffff) as u32;
        let o_0_1: u32 = ((m_0_1 >> 27) & 0x07ffffff) as u32;
        let o_0_2: u32 = ((m_0_2 >> 27) & 0x07ffffff) as u32;
        let o_0_3: u32 = ((m_0_3 >> 27) & 0x07ffffff) as u32;
        let o_0_4: u32 = ((m_0_4 >> 27) & 0x07ffffff) as u32;
        let o_0_5: u32 = ((m_0_5 >> 27) & 0x07ffffff) as u32;
        let o_0_6: u32 = ((m_0_6 >> 27) & 0x07ffffff) as u32;
        let o_0_7: u32 = ((m_0_7 >> 27) & 0x07ffffff) as u32;
        let o_0_8: u32 = ((m_0_8 >> 27) & 0x07ffffff) as u32;
        let o_0_9: u32 = ((m_0_9 >> 27) & 0x07ffffff) as u32;
        let o_0_10: u32 = ((m_0_10 >> 27) & 0x07ffffff) as u32;
        let o_0_11: u32 = ((m_0_11 >> 27) & 0x07ffffff) as u32;
        let o_0_12: u32 = ((m_0_12 >> 27) & 0x07ffffff) as u32;
        let o_0_13: u32 = ((m_0_13 >> 27) & 0x07ffffff) as u32;
        let o_0_14: u32 = ((m_0_14 >> 27) & 0x07ffffff) as u32;
        let o_0_15: u32 = ((m_0_15 >> 27) & 0x07ffffff) as u32;
        let o_0_16: u32 = ((m_0_16 >> 27) & 0x07ffffff) as u32;
        let o_0_17: u32 = ((m_0_17 >> 27) & 0x07ffffff) as u32;
        let o_0_18: u32 = ((m_0_18 >> 27) & 0x07ffffff) as u32;
        let o_0_19: u32 = ((m_0_19 >> 27) & 0x07ffffff) as u32;
        let o_1_0: u32 = ((m_1_0 >> 27) & 0x07ffffff) as u32;
        let o_1_1: u32 = ((m_1_1 >> 27) & 0x07ffffff) as u32;
        let o_1_2: u32 = ((m_1_2 >> 27) & 0x07ffffff) as u32;
        let o_1_3: u32 = ((m_1_3 >> 27) & 0x07ffffff) as u32;
        let o_1_4: u32 = ((m_1_4 >> 27) & 0x07ffffff) as u32;
        let o_1_5: u32 = ((m_1_5 >> 27) & 0x07ffffff) as u32;
        let o_1_6: u32 = ((m_1_6 >> 27) & 0x07ffffff) as u32;
        let o_1_7: u32 = ((m_1_7 >> 27) & 0x07ffffff) as u32;
        let o_1_8: u32 = ((m_1_8 >> 27) & 0x07ffffff) as u32;
        let o_1_9: u32 = ((m_1_9 >> 27) & 0x07ffffff) as u32;
        let o_1_10: u32 = ((m_1_10 >> 27) & 0x07ffffff) as u32;
        let o_1_11: u32 = ((m_1_11 >> 27) & 0x07ffffff) as u32;
        let o_1_12: u32 = ((m_1_12 >> 27) & 0x07ffffff) as u32;
        let o_1_13: u32 = ((m_1_13 >> 27) & 0x07ffffff) as u32;
        let o_1_14: u32 = ((m_1_14 >> 27) & 0x07ffffff) as u32;
        let o_1_15: u32 = ((m_1_15 >> 27) & 0x07ffffff) as u32;
        let o_1_16: u32 = ((m_1_16 >> 27) & 0x07ffffff) as u32;
        let o_1_17: u32 = ((m_1_17 >> 27) & 0x07ffffff) as u32;
        let o_1_18: u32 = ((m_1_18 >> 27) & 0x07ffffff) as u32;
        let o_1_19: u32 = ((m_1_19 >> 27) & 0x07ffffff) as u32;
        let o_2_0: u32 = ((m_2_0 >> 27) & 0x07ffffff) as u32;
        let o_2_1: u32 = ((m_2_1 >> 27) & 0x07ffffff) as u32;
        let o_2_2: u32 = ((m_2_2 >> 27) & 0x07ffffff) as u32;
        let o_2_3: u32 = ((m_2_3 >> 27) & 0x07ffffff) as u32;
        let o_2_4: u32 = ((m_2_4 >> 27) & 0x07ffffff) as u32;
        let o_2_5: u32 = ((m_2_5 >> 27) & 0x07ffffff) as u32;
        let o_2_6: u32 = ((m_2_6 >> 27) & 0x07ffffff) as u32;
        let o_2_7: u32 = ((m_2_7 >> 27) & 0x07ffffff) as u32;
        let o_2_8: u32 = ((m_2_8 >> 27) & 0x07ffffff) as u32;
        let o_2_9: u32 = ((m_2_9 >> 27) & 0x07ffffff) as u32;
        let o_2_10: u32 = ((m_2_10 >> 27) & 0x07ffffff) as u32;
        let o_2_11: u32 = ((m_2_11 >> 27) & 0x07ffffff) as u32;
        let o_2_12: u32 = ((m_2_12 >> 27) & 0x07ffffff) as u32;
        let o_2_13: u32 = ((m_2_13 >> 27) & 0x07ffffff) as u32;
        let o_2_14: u32 = ((m_2_14 >> 27) & 0x07ffffff) as u32;
        let o_2_15: u32 = ((m_2_15 >> 27) & 0x07ffffff) as u32;
        let o_2_16: u32 = ((m_2_16 >> 27) & 0x07ffffff) as u32;
        let o_2_17: u32 = ((m_2_17 >> 27) & 0x07ffffff) as u32;
        let o_2_18: u32 = ((m_2_18 >> 27) & 0x07ffffff) as u32;
        let o_2_19: u32 = ((m_2_19 >> 27) & 0x07ffffff) as u32;
        let o_3_0: u32 = ((m_3_0 >> 27) & 0x07ffffff) as u32;
        let o_3_1: u32 = ((m_3_1 >> 27) & 0x07ffffff) as u32;
        let o_3_2: u32 = ((m_3_2 >> 27) & 0x07ffffff) as u32;
        let o_3_3: u32 = ((m_3_3 >> 27) & 0x07ffffff) as u32;
        let o_3_4: u32 = ((m_3_4 >> 27) & 0x07ffffff) as u32;
        let o_3_5: u32 = ((m_3_5 >> 27) & 0x07ffffff) as u32;
        let o_3_6: u32 = ((m_3_6 >> 27) & 0x07ffffff) as u32;
        let o_3_7: u32 = ((m_3_7 >> 27) & 0x07ffffff) as u32;
        let o_3_8: u32 = ((m_3_8 >> 27) & 0x07ffffff) as u32;
        let o_3_9: u32 = ((m_3_9 >> 27) & 0x07ffffff) as u32;
        let o_3_10: u32 = ((m_3_10 >> 27) & 0x07ffffff) as u32;
        let o_3_11: u32 = ((m_3_11 >> 27) & 0x07ffffff) as u32;
        let o_3_12: u32 = ((m_3_12 >> 27) & 0x07ffffff) as u32;
        let o_3_13: u32 = ((m_3_13 >> 27) & 0x07ffffff) as u32;
        let o_3_14: u32 = ((m_3_14 >> 27) & 0x07ffffff) as u32;
        let o_3_15: u32 = ((m_3_15 >> 27) & 0x07ffffff) as u32;
        let o_3_16: u32 = ((m_3_16 >> 27) & 0x07ffffff) as u32;
        let o_3_17: u32 = ((m_3_17 >> 27) & 0x07ffffff) as u32;
        let o_3_18: u32 = ((m_3_18 >> 27) & 0x07ffffff) as u32;
        let o_3_19: u32 = ((m_3_19 >> 27) & 0x07ffffff) as u32;
        let o_4_0: u32 = ((m_4_0 >> 27) & 0x07ffffff) as u32;
        let o_4_1: u32 = ((m_4_1 >> 27) & 0x07ffffff) as u32;
        let o_4_2: u32 = ((m_4_2 >> 27) & 0x07ffffff) as u32;
        let o_4_3: u32 = ((m_4_3 >> 27) & 0x07ffffff) as u32;
        let o_4_4: u32 = ((m_4_4 >> 27) & 0x07ffffff) as u32;
        let o_4_5: u32 = ((m_4_5 >> 27) & 0x07ffffff) as u32;
        let o_4_6: u32 = ((m_4_6 >> 27) & 0x07ffffff) as u32;
        let o_4_7: u32 = ((m_4_7 >> 27) & 0x07ffffff) as u32;
        let o_4_8: u32 = ((m_4_8 >> 27) & 0x07ffffff) as u32;
        let o_4_9: u32 = ((m_4_9 >> 27) & 0x07ffffff) as u32;
        let o_4_10: u32 = ((m_4_10 >> 27) & 0x07ffffff) as u32;
        let o_4_11: u32 = ((m_4_11 >> 27) & 0x07ffffff) as u32;
        let o_4_12: u32 = ((m_4_12 >> 27) & 0x07ffffff) as u32;
        let o_4_13: u32 = ((m_4_13 >> 27) & 0x07ffffff) as u32;
        let o_4_14: u32 = ((m_4_14 >> 27) & 0x07ffffff) as u32;
        let o_4_15: u32 = ((m_4_15 >> 27) & 0x07ffffff) as u32;
        let o_4_16: u32 = ((m_4_16 >> 27) & 0x07ffffff) as u32;
        let o_4_17: u32 = ((m_4_17 >> 27) & 0x07ffffff) as u32;
        let o_4_18: u32 = ((m_4_18 >> 27) & 0x07ffffff) as u32;
        let o_4_19: u32 = ((m_4_19 >> 27) & 0x07ffffff) as u32;
        let o_5_0: u32 = ((m_5_0 >> 27) & 0x07ffffff) as u32;
        let o_5_1: u32 = ((m_5_1 >> 27) & 0x07ffffff) as u32;
        let o_5_2: u32 = ((m_5_2 >> 27) & 0x07ffffff) as u32;
        let o_5_3: u32 = ((m_5_3 >> 27) & 0x07ffffff) as u32;
        let o_5_4: u32 = ((m_5_4 >> 27) & 0x07ffffff) as u32;
        let o_5_5: u32 = ((m_5_5 >> 27) & 0x07ffffff) as u32;
        let o_5_6: u32 = ((m_5_6 >> 27) & 0x07ffffff) as u32;
        let o_5_7: u32 = ((m_5_7 >> 27) & 0x07ffffff) as u32;
        let o_5_8: u32 = ((m_5_8 >> 27) & 0x07ffffff) as u32;
        let o_5_9: u32 = ((m_5_9 >> 27) & 0x07ffffff) as u32;
        let o_5_10: u32 = ((m_5_10 >> 27) & 0x07ffffff) as u32;
        let o_5_11: u32 = ((m_5_11 >> 27) & 0x07ffffff) as u32;
        let o_5_12: u32 = ((m_5_12 >> 27) & 0x07ffffff) as u32;
        let o_5_13: u32 = ((m_5_13 >> 27) & 0x07ffffff) as u32;
        let o_5_14: u32 = ((m_5_14 >> 27) & 0x07ffffff) as u32;
        let o_5_15: u32 = ((m_5_15 >> 27) & 0x07ffffff) as u32;
        let o_5_16: u32 = ((m_5_16 >> 27) & 0x07ffffff) as u32;
        let o_5_17: u32 = ((m_5_17 >> 27) & 0x07ffffff) as u32;
        let o_5_18: u32 = ((m_5_18 >> 27) & 0x07ffffff) as u32;
        let o_5_19: u32 = ((m_5_19 >> 27) & 0x07ffffff) as u32;
        let o_6_0: u32 = ((m_6_0 >> 27) & 0x07ffffff) as u32;
        let o_6_1: u32 = ((m_6_1 >> 27) & 0x07ffffff) as u32;
        let o_6_2: u32 = ((m_6_2 >> 27) & 0x07ffffff) as u32;
        let o_6_3: u32 = ((m_6_3 >> 27) & 0x07ffffff) as u32;
        let o_6_4: u32 = ((m_6_4 >> 27) & 0x07ffffff) as u32;
        let o_6_5: u32 = ((m_6_5 >> 27) & 0x07ffffff) as u32;
        let o_6_6: u32 = ((m_6_6 >> 27) & 0x07ffffff) as u32;
        let o_6_7: u32 = ((m_6_7 >> 27) & 0x07ffffff) as u32;
        let o_6_8: u32 = ((m_6_8 >> 27) & 0x07ffffff) as u32;
        let o_6_9: u32 = ((m_6_9 >> 27) & 0x07ffffff) as u32;
        let o_6_10: u32 = ((m_6_10 >> 27) & 0x07ffffff) as u32;
        let o_6_11: u32 = ((m_6_11 >> 27) & 0x07ffffff) as u32;
        let o_6_12: u32 = ((m_6_12 >> 27) & 0x07ffffff) as u32;
        let o_6_13: u32 = ((m_6_13 >> 27) & 0x07ffffff) as u32;
        let o_6_14: u32 = ((m_6_14 >> 27) & 0x07ffffff) as u32;
        let o_6_15: u32 = ((m_6_15 >> 27) & 0x07ffffff) as u32;
        let o_6_16: u32 = ((m_6_16 >> 27) & 0x07ffffff) as u32;
        let o_6_17: u32 = ((m_6_17 >> 27) & 0x07ffffff) as u32;
        let o_6_18: u32 = ((m_6_18 >> 27) & 0x07ffffff) as u32;
        let o_6_19: u32 = ((m_6_19 >> 27) & 0x07ffffff) as u32;
        let o_7_0: u32 = ((m_7_0 >> 27) & 0x07ffffff) as u32;
        let o_7_1: u32 = ((m_7_1 >> 27) & 0x07ffffff) as u32;
        let o_7_2: u32 = ((m_7_2 >> 27) & 0x07ffffff) as u32;
        let o_7_3: u32 = ((m_7_3 >> 27) & 0x07ffffff) as u32;
        let o_7_4: u32 = ((m_7_4 >> 27) & 0x07ffffff) as u32;
        let o_7_5: u32 = ((m_7_5 >> 27) & 0x07ffffff) as u32;
        let o_7_6: u32 = ((m_7_6 >> 27) & 0x07ffffff) as u32;
        let o_7_7: u32 = ((m_7_7 >> 27) & 0x07ffffff) as u32;
        let o_7_8: u32 = ((m_7_8 >> 27) & 0x07ffffff) as u32;
        let o_7_9: u32 = ((m_7_9 >> 27) & 0x07ffffff) as u32;
        let o_7_10: u32 = ((m_7_10 >> 27) & 0x07ffffff) as u32;
        let o_7_11: u32 = ((m_7_11 >> 27) & 0x07ffffff) as u32;
        let o_7_12: u32 = ((m_7_12 >> 27) & 0x07ffffff) as u32;
        let o_7_13: u32 = ((m_7_13 >> 27) & 0x07ffffff) as u32;
        let o_7_14: u32 = ((m_7_14 >> 27) & 0x07ffffff) as u32;
        let o_7_15: u32 = ((m_7_15 >> 27) & 0x07ffffff) as u32;
        let o_7_16: u32 = ((m_7_16 >> 27) & 0x07ffffff) as u32;
        let o_7_17: u32 = ((m_7_17 >> 27) & 0x07ffffff) as u32;
        let o_7_18: u32 = ((m_7_18 >> 27) & 0x07ffffff) as u32;
        let o_7_19: u32 = ((m_7_19 >> 27) & 0x07ffffff) as u32;
        let o_8_0: u32 = ((m_8_0 >> 27) & 0x07ffffff) as u32;
        let o_8_1: u32 = ((m_8_1 >> 27) & 0x07ffffff) as u32;
        let o_8_2: u32 = ((m_8_2 >> 27) & 0x07ffffff) as u32;
        let o_8_3: u32 = ((m_8_3 >> 27) & 0x07ffffff) as u32;
        let o_8_4: u32 = ((m_8_4 >> 27) & 0x07ffffff) as u32;
        let o_8_5: u32 = ((m_8_5 >> 27) & 0x07ffffff) as u32;
        let o_8_6: u32 = ((m_8_6 >> 27) & 0x07ffffff) as u32;
        let o_8_7: u32 = ((m_8_7 >> 27) & 0x07ffffff) as u32;
        let o_8_8: u32 = ((m_8_8 >> 27) & 0x07ffffff) as u32;
        let o_8_9: u32 = ((m_8_9 >> 27) & 0x07ffffff) as u32;
        let o_8_10: u32 = ((m_8_10 >> 27) & 0x07ffffff) as u32;
        let o_8_11: u32 = ((m_8_11 >> 27) & 0x07ffffff) as u32;
        let o_8_12: u32 = ((m_8_12 >> 27) & 0x07ffffff) as u32;
        let o_8_13: u32 = ((m_8_13 >> 27) & 0x07ffffff) as u32;
        let o_8_14: u32 = ((m_8_14 >> 27) & 0x07ffffff) as u32;
        let o_8_15: u32 = ((m_8_15 >> 27) & 0x07ffffff) as u32;
        let o_8_16: u32 = ((m_8_16 >> 27) & 0x07ffffff) as u32;
        let o_8_17: u32 = ((m_8_17 >> 27) & 0x07ffffff) as u32;
        let o_8_18: u32 = ((m_8_18 >> 27) & 0x07ffffff) as u32;
        let o_8_19: u32 = ((m_8_19 >> 27) & 0x07ffffff) as u32;
        let o_9_0: u32 = ((m_9_0 >> 27) & 0x07ffffff) as u32;
        let o_9_1: u32 = ((m_9_1 >> 27) & 0x07ffffff) as u32;
        let o_9_2: u32 = ((m_9_2 >> 27) & 0x07ffffff) as u32;
        let o_9_3: u32 = ((m_9_3 >> 27) & 0x07ffffff) as u32;
        let o_9_4: u32 = ((m_9_4 >> 27) & 0x07ffffff) as u32;
        let o_9_5: u32 = ((m_9_5 >> 27) & 0x07ffffff) as u32;
        let o_9_6: u32 = ((m_9_6 >> 27) & 0x07ffffff) as u32;
        let o_9_7: u32 = ((m_9_7 >> 27) & 0x07ffffff) as u32;
        let o_9_8: u32 = ((m_9_8 >> 27) & 0x07ffffff) as u32;
        let o_9_9: u32 = ((m_9_9 >> 27) & 0x07ffffff) as u32;
        let o_9_10: u32 = ((m_9_10 >> 27) & 0x07ffffff) as u32;
        let o_9_11: u32 = ((m_9_11 >> 27) & 0x07ffffff) as u32;
        let o_9_12: u32 = ((m_9_12 >> 27) & 0x07ffffff) as u32;
        let o_9_13: u32 = ((m_9_13 >> 27) & 0x07ffffff) as u32;
        let o_9_14: u32 = ((m_9_14 >> 27) & 0x07ffffff) as u32;
        let o_9_15: u32 = ((m_9_15 >> 27) & 0x07ffffff) as u32;
        let o_9_16: u32 = ((m_9_16 >> 27) & 0x07ffffff) as u32;
        let o_9_17: u32 = ((m_9_17 >> 27) & 0x07ffffff) as u32;
        let o_9_18: u32 = ((m_9_18 >> 27) & 0x07ffffff) as u32;
        let o_9_19: u32 = ((m_9_19 >> 27) & 0x07ffffff) as u32;
        let o_10_0: u32 = ((m_10_0 >> 27) & 0x07ffffff) as u32;
        let o_10_1: u32 = ((m_10_1 >> 27) & 0x07ffffff) as u32;
        let o_10_2: u32 = ((m_10_2 >> 27) & 0x07ffffff) as u32;
        let o_10_3: u32 = ((m_10_3 >> 27) & 0x07ffffff) as u32;
        let o_10_4: u32 = ((m_10_4 >> 27) & 0x07ffffff) as u32;
        let o_10_5: u32 = ((m_10_5 >> 27) & 0x07ffffff) as u32;
        let o_10_6: u32 = ((m_10_6 >> 27) & 0x07ffffff) as u32;
        let o_10_7: u32 = ((m_10_7 >> 27) & 0x07ffffff) as u32;
        let o_10_8: u32 = ((m_10_8 >> 27) & 0x07ffffff) as u32;
        let o_10_9: u32 = ((m_10_9 >> 27) & 0x07ffffff) as u32;
        let o_10_10: u32 = ((m_10_10 >> 27) & 0x07ffffff) as u32;
        let o_10_11: u32 = ((m_10_11 >> 27) & 0x07ffffff) as u32;
        let o_10_12: u32 = ((m_10_12 >> 27) & 0x07ffffff) as u32;
        let o_10_13: u32 = ((m_10_13 >> 27) & 0x07ffffff) as u32;
        let o_10_14: u32 = ((m_10_14 >> 27) & 0x07ffffff) as u32;
        let o_10_15: u32 = ((m_10_15 >> 27) & 0x07ffffff) as u32;
        let o_10_16: u32 = ((m_10_16 >> 27) & 0x07ffffff) as u32;
        let o_10_17: u32 = ((m_10_17 >> 27) & 0x07ffffff) as u32;
        let o_10_18: u32 = ((m_10_18 >> 27) & 0x07ffffff) as u32;
        let o_10_19: u32 = ((m_10_19 >> 27) & 0x07ffffff) as u32;
        let o_11_0: u32 = ((m_11_0 >> 27) & 0x07ffffff) as u32;
        let o_11_1: u32 = ((m_11_1 >> 27) & 0x07ffffff) as u32;
        let o_11_2: u32 = ((m_11_2 >> 27) & 0x07ffffff) as u32;
        let o_11_3: u32 = ((m_11_3 >> 27) & 0x07ffffff) as u32;
        let o_11_4: u32 = ((m_11_4 >> 27) & 0x07ffffff) as u32;
        let o_11_5: u32 = ((m_11_5 >> 27) & 0x07ffffff) as u32;
        let o_11_6: u32 = ((m_11_6 >> 27) & 0x07ffffff) as u32;
        let o_11_7: u32 = ((m_11_7 >> 27) & 0x07ffffff) as u32;
        let o_11_8: u32 = ((m_11_8 >> 27) & 0x07ffffff) as u32;
        let o_11_9: u32 = ((m_11_9 >> 27) & 0x07ffffff) as u32;
        let o_11_10: u32 = ((m_11_10 >> 27) & 0x07ffffff) as u32;
        let o_11_11: u32 = ((m_11_11 >> 27) & 0x07ffffff) as u32;
        let o_11_12: u32 = ((m_11_12 >> 27) & 0x07ffffff) as u32;
        let o_11_13: u32 = ((m_11_13 >> 27) & 0x07ffffff) as u32;
        let o_11_14: u32 = ((m_11_14 >> 27) & 0x07ffffff) as u32;
        let o_11_15: u32 = ((m_11_15 >> 27) & 0x07ffffff) as u32;
        let o_11_16: u32 = ((m_11_16 >> 27) & 0x07ffffff) as u32;
        let o_11_17: u32 = ((m_11_17 >> 27) & 0x07ffffff) as u32;
        let o_11_18: u32 = ((m_11_18 >> 27) & 0x07ffffff) as u32;
        let o_11_19: u32 = ((m_11_19 >> 27) & 0x07ffffff) as u32;
        let o_12_0: u32 = ((m_12_0 >> 27) & 0x07ffffff) as u32;
        let o_12_1: u32 = ((m_12_1 >> 27) & 0x07ffffff) as u32;
        let o_12_2: u32 = ((m_12_2 >> 27) & 0x07ffffff) as u32;
        let o_12_3: u32 = ((m_12_3 >> 27) & 0x07ffffff) as u32;
        let o_12_4: u32 = ((m_12_4 >> 27) & 0x07ffffff) as u32;
        let o_12_5: u32 = ((m_12_5 >> 27) & 0x07ffffff) as u32;
        let o_12_6: u32 = ((m_12_6 >> 27) & 0x07ffffff) as u32;
        let o_12_7: u32 = ((m_12_7 >> 27) & 0x07ffffff) as u32;
        let o_12_8: u32 = ((m_12_8 >> 27) & 0x07ffffff) as u32;
        let o_12_9: u32 = ((m_12_9 >> 27) & 0x07ffffff) as u32;
        let o_12_10: u32 = ((m_12_10 >> 27) & 0x07ffffff) as u32;
        let o_12_11: u32 = ((m_12_11 >> 27) & 0x07ffffff) as u32;
        let o_12_12: u32 = ((m_12_12 >> 27) & 0x07ffffff) as u32;
        let o_12_13: u32 = ((m_12_13 >> 27) & 0x07ffffff) as u32;
        let o_12_14: u32 = ((m_12_14 >> 27) & 0x07ffffff) as u32;
        let o_12_15: u32 = ((m_12_15 >> 27) & 0x07ffffff) as u32;
        let o_12_16: u32 = ((m_12_16 >> 27) & 0x07ffffff) as u32;
        let o_12_17: u32 = ((m_12_17 >> 27) & 0x07ffffff) as u32;
        let o_12_18: u32 = ((m_12_18 >> 27) & 0x07ffffff) as u32;
        let o_12_19: u32 = ((m_12_19 >> 27) & 0x07ffffff) as u32;
        let o_13_0: u32 = ((m_13_0 >> 27) & 0x07ffffff) as u32;
        let o_13_1: u32 = ((m_13_1 >> 27) & 0x07ffffff) as u32;
        let o_13_2: u32 = ((m_13_2 >> 27) & 0x07ffffff) as u32;
        let o_13_3: u32 = ((m_13_3 >> 27) & 0x07ffffff) as u32;
        let o_13_4: u32 = ((m_13_4 >> 27) & 0x07ffffff) as u32;
        let o_13_5: u32 = ((m_13_5 >> 27) & 0x07ffffff) as u32;
        let o_13_6: u32 = ((m_13_6 >> 27) & 0x07ffffff) as u32;
        let o_13_7: u32 = ((m_13_7 >> 27) & 0x07ffffff) as u32;
        let o_13_8: u32 = ((m_13_8 >> 27) & 0x07ffffff) as u32;
        let o_13_9: u32 = ((m_13_9 >> 27) & 0x07ffffff) as u32;
        let o_13_10: u32 = ((m_13_10 >> 27) & 0x07ffffff) as u32;
        let o_13_11: u32 = ((m_13_11 >> 27) & 0x07ffffff) as u32;
        let o_13_12: u32 = ((m_13_12 >> 27) & 0x07ffffff) as u32;
        let o_13_13: u32 = ((m_13_13 >> 27) & 0x07ffffff) as u32;
        let o_13_14: u32 = ((m_13_14 >> 27) & 0x07ffffff) as u32;
        let o_13_15: u32 = ((m_13_15 >> 27) & 0x07ffffff) as u32;
        let o_13_16: u32 = ((m_13_16 >> 27) & 0x07ffffff) as u32;
        let o_13_17: u32 = ((m_13_17 >> 27) & 0x07ffffff) as u32;
        let o_13_18: u32 = ((m_13_18 >> 27) & 0x07ffffff) as u32;
        let o_13_19: u32 = ((m_13_19 >> 27) & 0x07ffffff) as u32;
        let o_14_0: u32 = ((m_14_0 >> 27) & 0x07ffffff) as u32;
        let o_14_1: u32 = ((m_14_1 >> 27) & 0x07ffffff) as u32;
        let o_14_2: u32 = ((m_14_2 >> 27) & 0x07ffffff) as u32;
        let o_14_3: u32 = ((m_14_3 >> 27) & 0x07ffffff) as u32;
        let o_14_4: u32 = ((m_14_4 >> 27) & 0x07ffffff) as u32;
        let o_14_5: u32 = ((m_14_5 >> 27) & 0x07ffffff) as u32;
        let o_14_6: u32 = ((m_14_6 >> 27) & 0x07ffffff) as u32;
        let o_14_7: u32 = ((m_14_7 >> 27) & 0x07ffffff) as u32;
        let o_14_8: u32 = ((m_14_8 >> 27) & 0x07ffffff) as u32;
        let o_14_9: u32 = ((m_14_9 >> 27) & 0x07ffffff) as u32;
        let o_14_10: u32 = ((m_14_10 >> 27) & 0x07ffffff) as u32;
        let o_14_11: u32 = ((m_14_11 >> 27) & 0x07ffffff) as u32;
        let o_14_12: u32 = ((m_14_12 >> 27) & 0x07ffffff) as u32;
        let o_14_13: u32 = ((m_14_13 >> 27) & 0x07ffffff) as u32;
        let o_14_14: u32 = ((m_14_14 >> 27) & 0x07ffffff) as u32;
        let o_14_15: u32 = ((m_14_15 >> 27) & 0x07ffffff) as u32;
        let o_14_16: u32 = ((m_14_16 >> 27) & 0x07ffffff) as u32;
        let o_14_17: u32 = ((m_14_17 >> 27) & 0x07ffffff) as u32;
        let o_14_18: u32 = ((m_14_18 >> 27) & 0x07ffffff) as u32;
        let o_14_19: u32 = ((m_14_19 >> 27) & 0x07ffffff) as u32;
        let o_15_0: u32 = ((m_15_0 >> 27) & 0x07ffffff) as u32;
        let o_15_1: u32 = ((m_15_1 >> 27) & 0x07ffffff) as u32;
        let o_15_2: u32 = ((m_15_2 >> 27) & 0x07ffffff) as u32;
        let o_15_3: u32 = ((m_15_3 >> 27) & 0x07ffffff) as u32;
        let o_15_4: u32 = ((m_15_4 >> 27) & 0x07ffffff) as u32;
        let o_15_5: u32 = ((m_15_5 >> 27) & 0x07ffffff) as u32;
        let o_15_6: u32 = ((m_15_6 >> 27) & 0x07ffffff) as u32;
        let o_15_7: u32 = ((m_15_7 >> 27) & 0x07ffffff) as u32;
        let o_15_8: u32 = ((m_15_8 >> 27) & 0x07ffffff) as u32;
        let o_15_9: u32 = ((m_15_9 >> 27) & 0x07ffffff) as u32;
        let o_15_10: u32 = ((m_15_10 >> 27) & 0x07ffffff) as u32;
        let o_15_11: u32 = ((m_15_11 >> 27) & 0x07ffffff) as u32;
        let o_15_12: u32 = ((m_15_12 >> 27) & 0x07ffffff) as u32;
        let o_15_13: u32 = ((m_15_13 >> 27) & 0x07ffffff) as u32;
        let o_15_14: u32 = ((m_15_14 >> 27) & 0x07ffffff) as u32;
        let o_15_15: u32 = ((m_15_15 >> 27) & 0x07ffffff) as u32;
        let o_15_16: u32 = ((m_15_16 >> 27) & 0x07ffffff) as u32;
        let o_15_17: u32 = ((m_15_17 >> 27) & 0x07ffffff) as u32;
        let o_15_18: u32 = ((m_15_18 >> 27) & 0x07ffffff) as u32;
        let o_15_19: u32 = ((m_15_19 >> 27) & 0x07ffffff) as u32;
        let o_16_0: u32 = ((m_16_0 >> 27) & 0x07ffffff) as u32;
        let o_16_1: u32 = ((m_16_1 >> 27) & 0x07ffffff) as u32;
        let o_16_2: u32 = ((m_16_2 >> 27) & 0x07ffffff) as u32;
        let o_16_3: u32 = ((m_16_3 >> 27) & 0x07ffffff) as u32;
        let o_16_4: u32 = ((m_16_4 >> 27) & 0x07ffffff) as u32;
        let o_16_5: u32 = ((m_16_5 >> 27) & 0x07ffffff) as u32;
        let o_16_6: u32 = ((m_16_6 >> 27) & 0x07ffffff) as u32;
        let o_16_7: u32 = ((m_16_7 >> 27) & 0x07ffffff) as u32;
        let o_16_8: u32 = ((m_16_8 >> 27) & 0x07ffffff) as u32;
        let o_16_9: u32 = ((m_16_9 >> 27) & 0x07ffffff) as u32;
        let o_16_10: u32 = ((m_16_10 >> 27) & 0x07ffffff) as u32;
        let o_16_11: u32 = ((m_16_11 >> 27) & 0x07ffffff) as u32;
        let o_16_12: u32 = ((m_16_12 >> 27) & 0x07ffffff) as u32;
        let o_16_13: u32 = ((m_16_13 >> 27) & 0x07ffffff) as u32;
        let o_16_14: u32 = ((m_16_14 >> 27) & 0x07ffffff) as u32;
        let o_16_15: u32 = ((m_16_15 >> 27) & 0x07ffffff) as u32;
        let o_16_16: u32 = ((m_16_16 >> 27) & 0x07ffffff) as u32;
        let o_16_17: u32 = ((m_16_17 >> 27) & 0x07ffffff) as u32;
        let o_16_18: u32 = ((m_16_18 >> 27) & 0x07ffffff) as u32;
        let o_16_19: u32 = ((m_16_19 >> 27) & 0x07ffffff) as u32;
        let o_17_0: u32 = ((m_17_0 >> 27) & 0x07ffffff) as u32;
        let o_17_1: u32 = ((m_17_1 >> 27) & 0x07ffffff) as u32;
        let o_17_2: u32 = ((m_17_2 >> 27) & 0x07ffffff) as u32;
        let o_17_3: u32 = ((m_17_3 >> 27) & 0x07ffffff) as u32;
        let o_17_4: u32 = ((m_17_4 >> 27) & 0x07ffffff) as u32;
        let o_17_5: u32 = ((m_17_5 >> 27) & 0x07ffffff) as u32;
        let o_17_6: u32 = ((m_17_6 >> 27) & 0x07ffffff) as u32;
        let o_17_7: u32 = ((m_17_7 >> 27) & 0x07ffffff) as u32;
        let o_17_8: u32 = ((m_17_8 >> 27) & 0x07ffffff) as u32;
        let o_17_9: u32 = ((m_17_9 >> 27) & 0x07ffffff) as u32;
        let o_17_10: u32 = ((m_17_10 >> 27) & 0x07ffffff) as u32;
        let o_17_11: u32 = ((m_17_11 >> 27) & 0x07ffffff) as u32;
        let o_17_12: u32 = ((m_17_12 >> 27) & 0x07ffffff) as u32;
        let o_17_13: u32 = ((m_17_13 >> 27) & 0x07ffffff) as u32;
        let o_17_14: u32 = ((m_17_14 >> 27) & 0x07ffffff) as u32;
        let o_17_15: u32 = ((m_17_15 >> 27) & 0x07ffffff) as u32;
        let o_17_16: u32 = ((m_17_16 >> 27) & 0x07ffffff) as u32;
        let o_17_17: u32 = ((m_17_17 >> 27) & 0x07ffffff) as u32;
        let o_17_18: u32 = ((m_17_18 >> 27) & 0x07ffffff) as u32;
        let o_17_19: u32 = ((m_17_19 >> 27) & 0x07ffffff) as u32;
        let o_18_0: u32 = ((m_18_0 >> 27) & 0x07ffffff) as u32;
        let o_18_1: u32 = ((m_18_1 >> 27) & 0x07ffffff) as u32;
        let o_18_2: u32 = ((m_18_2 >> 27) & 0x07ffffff) as u32;
        let o_18_3: u32 = ((m_18_3 >> 27) & 0x07ffffff) as u32;
        let o_18_4: u32 = ((m_18_4 >> 27) & 0x07ffffff) as u32;
        let o_18_5: u32 = ((m_18_5 >> 27) & 0x07ffffff) as u32;
        let o_18_6: u32 = ((m_18_6 >> 27) & 0x07ffffff) as u32;
        let o_18_7: u32 = ((m_18_7 >> 27) & 0x07ffffff) as u32;
        let o_18_8: u32 = ((m_18_8 >> 27) & 0x07ffffff) as u32;
        let o_18_9: u32 = ((m_18_9 >> 27) & 0x07ffffff) as u32;
        let o_18_10: u32 = ((m_18_10 >> 27) & 0x07ffffff) as u32;
        let o_18_11: u32 = ((m_18_11 >> 27) & 0x07ffffff) as u32;
        let o_18_12: u32 = ((m_18_12 >> 27) & 0x07ffffff) as u32;
        let o_18_13: u32 = ((m_18_13 >> 27) & 0x07ffffff) as u32;
        let o_18_14: u32 = ((m_18_14 >> 27) & 0x07ffffff) as u32;
        let o_18_15: u32 = ((m_18_15 >> 27) & 0x07ffffff) as u32;
        let o_18_16: u32 = ((m_18_16 >> 27) & 0x07ffffff) as u32;
        let o_18_17: u32 = ((m_18_17 >> 27) & 0x07ffffff) as u32;
        let o_18_18: u32 = ((m_18_18 >> 27) & 0x07ffffff) as u32;
        let o_18_19: u32 = ((m_18_19 >> 27) & 0x07ffffff) as u32;
        let o_19_0: u32 = ((m_19_0 >> 27) & 0x07ffffff) as u32;
        let o_19_1: u32 = ((m_19_1 >> 27) & 0x07ffffff) as u32;
        let o_19_2: u32 = ((m_19_2 >> 27) & 0x07ffffff) as u32;
        let o_19_3: u32 = ((m_19_3 >> 27) & 0x07ffffff) as u32;
        let o_19_4: u32 = ((m_19_4 >> 27) & 0x07ffffff) as u32;
        let o_19_5: u32 = ((m_19_5 >> 27) & 0x07ffffff) as u32;
        let o_19_6: u32 = ((m_19_6 >> 27) & 0x07ffffff) as u32;
        let o_19_7: u32 = ((m_19_7 >> 27) & 0x07ffffff) as u32;
        let o_19_8: u32 = ((m_19_8 >> 27) & 0x07ffffff) as u32;
        let o_19_9: u32 = ((m_19_9 >> 27) & 0x07ffffff) as u32;
        let o_19_10: u32 = ((m_19_10 >> 27) & 0x07ffffff) as u32;
        let o_19_11: u32 = ((m_19_11 >> 27) & 0x07ffffff) as u32;
        let o_19_12: u32 = ((m_19_12 >> 27) & 0x07ffffff) as u32;
        let o_19_13: u32 = ((m_19_13 >> 27) & 0x07ffffff) as u32;
        let o_19_14: u32 = ((m_19_14 >> 27) & 0x07ffffff) as u32;
        let o_19_15: u32 = ((m_19_15 >> 27) & 0x07ffffff) as u32;
        let o_19_16: u32 = ((m_19_16 >> 27) & 0x07ffffff) as u32;
        let o_19_17: u32 = ((m_19_17 >> 27) & 0x07ffffff) as u32;
        let o_19_18: u32 = ((m_19_18 >> 27) & 0x07ffffff) as u32;

        // Compute the 40-digit combined product.
        let d0 = p_0_0;
        let d1 = p_0_1 + o_0_0 +
                 p_1_0;
        let c1 = d1 >> 27;
        let d2 = p_0_2 + o_0_1 +
                 p_1_1 + o_1_0 +
                 p_2_0 + c1;
        let c2 = d2 >> 27;
        let d3 = p_0_3 + o_0_2 +
                 p_1_2 + o_1_1 +
                 p_2_1 + o_2_0 +
                 p_3_0 + c2;
        let c3 = d3 >> 27;
        let d4 = p_0_4 + o_0_3 +
                 p_1_3 + o_1_2 +
                 p_2_2 + o_2_1 +
                 p_3_1 + o_3_0 +
                 p_4_0 + c3;
        let c4 = d4 >> 27;
        let d5 = p_0_5 + o_0_4 +
                 p_1_4 + o_1_3 +
                 p_2_3 + o_2_2 +
                 p_3_2 + o_3_1 +
                 p_4_1 + o_4_0 +
                 p_5_0 + c4;
        let c5 = d5 >> 27;
        let d6 = p_0_6 + o_0_5 +
                 p_1_5 + o_1_4 +
                 p_2_4 + o_2_3 +
                 p_3_3 + o_3_2 +
                 p_4_2 + o_4_1 +
                 p_5_1 + o_5_0 +
                 p_6_0 + c5;
        let c6 = d6 >> 27;
        let d7 = p_0_7 + o_0_6 +
                 p_1_6 + o_1_5 +
                 p_2_5 + o_2_4 +
                 p_3_4 + o_3_3 +
                 p_4_3 + o_4_2 +
                 p_5_2 + o_5_1 +
                 p_6_1 + o_6_0 +
                 p_7_0 + c6;
        let c7 = d7 >> 27;
        let d8 = p_0_8 + o_0_7 +
                 p_1_7 + o_1_6 +
                 p_2_6 + o_2_5 +
                 p_3_5 + o_3_4 +
                 p_4_4 + o_4_3 +
                 p_5_3 + o_5_2 +
                 p_6_2 + o_6_1 +
                 p_7_1 + o_7_0 +
                 p_8_0 + c7;
        let c8 = d8 >> 27;
        let d9 = p_0_9 + o_0_8 +
                 p_1_8 + o_1_7 +
                 p_2_7 + o_2_6 +
                 p_3_6 + o_3_5 +
                 p_4_5 + o_4_4 +
                 p_5_4 + o_5_3 +
                 p_6_3 + o_6_2 +
                 p_7_2 + o_7_1 +
                 p_8_1 + o_8_0 +
                 p_9_0 + c8;
        let c9 = d9 >> 27;
        let d10 = p_0_10 + o_0_9 +
                  p_1_9 + o_1_8 +
                  p_2_8 + o_2_7 +
                  p_3_7 + o_3_6 +
                  p_4_6 + o_4_5 +
                  p_5_5 + o_5_4 +
                  p_6_4 + o_6_3 +
                  p_7_3 + o_7_2 +
                  p_8_2 + o_8_1 +
                  p_9_1 + o_9_0 +
                  p_10_0 + c9;
        let c10 = d10 >> 27;
        let d11 = p_0_11 + o_0_10 +
                  p_1_10 + o_1_9 +
                  p_2_9 + o_2_8 +
                  p_3_8 + o_3_7 +
                  p_4_7 + o_4_6 +
                  p_5_6 + o_5_5 +
                  p_6_5 + o_6_4 +
                  p_7_4 + o_7_3 +
                  p_8_3 + o_8_2 +
                  p_9_2 + o_9_1 +
                  p_10_1 + o_10_0 +
                  p_11_0 + c10;
        let c11 = d11 >> 27;
        let d12 = p_0_12 + o_0_11 +
                  p_1_11 + o_1_10 +
                  p_2_10 + o_2_9 +
                  p_3_9 + o_3_8 +
                  p_4_8 + o_4_7 +
                  p_5_7 + o_5_6 +
                  p_6_6 + o_6_5 +
                  p_7_5 + o_7_4 +
                  p_8_4 + o_8_3 +
                  p_9_3 + o_9_2 +
                  p_10_2 + o_10_1 +
                  p_11_1 + o_11_0 +
                  p_12_0 + c11;
        let c12 = d12 >> 27;
        let d13 = p_0_13 + o_0_12 +
                  p_1_12 + o_1_11 +
                  p_2_11 + o_2_10 +
                  p_3_10 + o_3_9 +
                  p_4_9 + o_4_8 +
                  p_5_8 + o_5_7 +
                  p_6_7 + o_6_6 +
                  p_7_6 + o_7_5 +
                  p_8_5 + o_8_4 +
                  p_9_4 + o_9_3 +
                  p_10_3 + o_10_2 +
                  p_11_2 + o_11_1 +
                  p_12_1 + o_12_0 +
                  p_13_0 + c12;
        let c13 = d13 >> 27;
        let d14 = p_0_14 + o_0_13 +
                  p_1_13 + o_1_12 +
                  p_2_12 + o_2_11 +
                  p_3_11 + o_3_10 +
                  p_4_10 + o_4_9 +
                  p_5_9 + o_5_8 +
                  p_6_8 + o_6_7 +
                  p_7_7 + o_7_6 +
                  p_8_6 + o_8_5 +
                  p_9_5 + o_9_4 +
                  p_10_4 + o_10_3 +
                  p_11_3 + o_11_2 +
                  p_12_2 + o_12_1 +
                  p_13_1 + o_13_0 +
                  p_14_0 + c13;
        let c14 = d14 >> 27;
        let d15 = p_0_15 + o_0_14 +
                  p_1_14 + o_1_13 +
                  p_2_13 + o_2_12 +
                  p_3_12 + o_3_11 +
                  p_4_11 + o_4_10 +
                  p_5_10 + o_5_9 +
                  p_6_9 + o_6_8 +
                  p_7_8 + o_7_7 +
                  p_8_7 + o_8_6 +
                  p_9_6 + o_9_5 +
                  p_10_5 + o_10_4 +
                  p_11_4 + o_11_3 +
                  p_12_3 + o_12_2 +
                  p_13_2 + o_13_1 +
                  p_14_1 + o_14_0 +
                  p_15_0 + c14;
        let c15 = d15 >> 27;
        let d16 = p_0_16 + o_0_15 +
                  p_1_15 + o_1_14 +
                  p_2_14 + o_2_13 +
                  p_3_13 + o_3_12 +
                  p_4_12 + o_4_11 +
                  p_5_11 + o_5_10 +
                  p_6_10 + o_6_9 +
                  p_7_9 + o_7_8 +
                  p_8_8 + o_8_7 +
                  p_9_7 + o_9_6 +
                  p_10_6 + o_10_5 +
                  p_11_5 + o_11_4 +
                  p_12_4 + o_12_3 +
                  p_13_3 + o_13_2 +
                  p_14_2 + o_14_1 +
                  p_15_1 + o_15_0 +
                  p_16_0 + c15;
        let c16 = d16 >> 27;
        let d17 = p_0_17 + o_0_16 +
                  p_1_16 + o_1_15 +
                  p_2_15 + o_2_14 +
                  p_3_14 + o_3_13 +
                  p_4_13 + o_4_12 +
                  p_5_12 + o_5_11 +
                  p_6_11 + o_6_10 +
                  p_7_10 + o_7_9 +
                  p_8_9 + o_8_8 +
                  p_9_8 + o_9_7 +
                  p_10_7 + o_10_6 +
                  p_11_6 + o_11_5 +
                  p_12_5 + o_12_4 +
                  p_13_4 + o_13_3 +
                  p_14_3 + o_14_2 +
                  p_15_2 + o_15_1 +
                  p_16_1 + o_16_0 +
                  p_17_0 + c16;
        let c17 = d17 >> 27;
        let d18 = p_0_18 + o_0_17 +
                  p_1_17 + o_1_16 +
                  p_2_16 + o_2_15 +
                  p_3_15 + o_3_14 +
                  p_4_14 + o_4_13 +
                  p_5_13 + o_5_12 +
                  p_6_12 + o_6_11 +
                  p_7_11 + o_7_10 +
                  p_8_10 + o_8_9 +
                  p_9_9 + o_9_8 +
                  p_10_8 + o_10_7 +
                  p_11_7 + o_11_6 +
                  p_12_6 + o_12_5 +
                  p_13_5 + o_13_4 +
                  p_14_4 + o_14_3 +
                  p_15_3 + o_15_2 +
                  p_16_2 + o_16_1 +
                  p_17_1 + o_17_0 +
                  p_18_0 + c17;
        let c18 = d18 >> 27;
        let d19 = p_0_19 + o_0_18 +
                  p_1_18 + o_1_17 +
                  p_2_17 + o_2_16 +
                  p_3_16 + o_3_15 +
                  p_4_15 + o_4_14 +
                  p_5_14 + o_5_13 +
                  p_6_13 + o_6_12 +
                  p_7_12 + o_7_11 +
                  p_8_11 + o_8_10 +
                  p_9_10 + o_9_9 +
                  p_10_9 + o_10_8 +
                  p_11_8 + o_11_7 +
                  p_12_7 + o_12_6 +
                  p_13_6 + o_13_5 +
                  p_14_5 + o_14_4 +
                  p_15_4 + o_15_3 +
                  p_16_3 + o_16_2 +
                  p_17_2 + o_17_1 +
                  p_18_1 + o_18_0 +
                  p_19_0 + c18;
        let c19 = d19 >> 27;
        let d20 = p_1_19 + o_0_19 +
                  p_2_18 + o_1_18 +
                  p_3_17 + o_2_17 +
                  p_4_16 + o_3_16 +
                  p_5_15 + o_4_15 +
                  p_6_14 + o_5_14 +
                  p_7_13 + o_6_13 +
                  p_8_12 + o_7_12 +
                  p_9_11 + o_8_11 +
                  p_10_10 + o_9_10 +
                  p_11_9 + o_10_9 +
                  p_12_8 + o_11_8 +
                  p_13_7 + o_12_7 +
                  p_14_6 + o_13_6 +
                  p_15_5 + o_14_5 +
                  p_16_4 + o_15_4 +
                  p_17_3 + o_16_3 +
                  p_18_2 + o_17_2 +
                  p_19_1 + o_18_1 +
                  c19 + o_19_0;
        let c20 = d20 >> 27;
        let d21 = p_2_19 + o_1_19 +
                  p_3_18 + o_2_18 +
                  p_4_17 + o_3_17 +
                  p_5_16 + o_4_16 +
                  p_6_15 + o_5_15 +
                  p_7_14 + o_6_14 +
                  p_8_13 + o_7_13 +
                  p_9_12 + o_8_12 +
                  p_10_11 + o_9_11 +
                  p_11_10 + o_10_10 +
                  p_12_9 + o_11_9 +
                  p_13_8 + o_12_8 +
                  p_14_7 + o_13_7 +
                  p_15_6 + o_14_6 +
                  p_16_5 + o_15_5 +
                  p_17_4 + o_16_4 +
                  p_18_3 + o_17_3 +
                  p_19_2 + o_18_2 +
                  c20 + o_19_1;
        let c21 = d21 >> 27;
        let d22 = p_3_19 + o_2_19 +
                  p_4_18 + o_3_18 +
                  p_5_17 + o_4_17 +
                  p_6_16 + o_5_16 +
                  p_7_15 + o_6_15 +
                  p_8_14 + o_7_14 +
                  p_9_13 + o_8_13 +
                  p_10_12 + o_9_12 +
                  p_11_11 + o_10_11 +
                  p_12_10 + o_11_10 +
                  p_13_9 + o_12_9 +
                  p_14_8 + o_13_8 +
                  p_15_7 + o_14_7 +
                  p_16_6 + o_15_6 +
                  p_17_5 + o_16_5 +
                  p_18_4 + o_17_4 +
                  p_19_3 + o_18_3 +
                  c21 + o_19_2;
        let c22 = d22 >> 27;
        let d23 = p_4_19 + o_3_19 +
                  p_5_18 + o_4_18 +
                  p_6_17 + o_5_17 +
                  p_7_16 + o_6_16 +
                  p_8_15 + o_7_15 +
                  p_9_14 + o_8_14 +
                  p_10_13 + o_9_13 +
                  p_11_12 + o_10_12 +
                  p_12_11 + o_11_11 +
                  p_13_10 + o_12_10 +
                  p_14_9 + o_13_9 +
                  p_15_8 + o_14_8 +
                  p_16_7 + o_15_7 +
                  p_17_6 + o_16_6 +
                  p_18_5 + o_17_5 +
                  p_19_4 + o_18_4 +
                  c22 + o_19_3;
        let c23 = d23 >> 27;
        let d24 = p_5_19 + o_4_19 +
                  p_6_18 + o_5_18 +
                  p_7_17 + o_6_17 +
                  p_8_16 + o_7_16 +
                  p_9_15 + o_8_15 +
                  p_10_14 + o_9_14 +
                  p_11_13 + o_10_13 +
                  p_12_12 + o_11_12 +
                  p_13_11 + o_12_11 +
                  p_14_10 + o_13_10 +
                  p_15_9 + o_14_9 +
                  p_16_8 + o_15_8 +
                  p_17_7 + o_16_7 +
                  p_18_6 + o_17_6 +
                  p_19_5 + o_18_5 +
                  c23 + o_19_4;
        let c24 = d24 >> 27;
        let d25 = p_6_19 + o_5_19 +
                  p_7_18 + o_6_18 +
                  p_8_17 + o_7_17 +
                  p_9_16 + o_8_16 +
                  p_10_15 + o_9_15 +
                  p_11_14 + o_10_14 +
                  p_12_13 + o_11_13 +
                  p_13_12 + o_12_12 +
                  p_14_11 + o_13_11 +
                  p_15_10 + o_14_10 +
                  p_16_9 + o_15_9 +
                  p_17_8 + o_16_8 +
                  p_18_7 + o_17_7 +
                  p_19_6 + o_18_6 +
                  c24 + o_19_5;
        let c25 = d25 >> 27;
        let d26 = p_7_19 + o_6_19 +
                  p_8_18 + o_7_18 +
                  p_9_17 + o_8_17 +
                  p_10_16 + o_9_16 +
                  p_11_15 + o_10_15 +
                  p_12_14 + o_11_14 +
                  p_13_13 + o_12_13 +
                  p_14_12 + o_13_12 +
                  p_15_11 + o_14_11 +
                  p_16_10 + o_15_10 +
                  p_17_9 + o_16_9 +
                  p_18_8 + o_17_8 +
                  p_19_7 + o_18_7 +
                  c25 + o_19_6;
        let c26 = d26 >> 27;
        let d27 = p_8_19 + o_7_19 +
                  p_9_18 + o_8_18 +
                  p_10_17 + o_9_17 +
                  p_11_16 + o_10_16 +
                  p_12_15 + o_11_15 +
                  p_13_14 + o_12_14 +
                  p_14_13 + o_13_13 +
                  p_15_12 + o_14_12 +
                  p_16_11 + o_15_11 +
                  p_17_10 + o_16_10 +
                  p_18_9 + o_17_9 +
                  p_19_8 + o_18_8 +
                  c26 + o_19_7;
        let c27 = d27 >> 27;
        let d28 = p_9_19 + o_8_19 +
                  p_10_18 + o_9_18 +
                  p_11_17 + o_10_17 +
                  p_12_16 + o_11_16 +
                  p_13_15 + o_12_15 +
                  p_14_14 + o_13_14 +
                  p_15_13 + o_14_13 +
                  p_16_12 + o_15_12 +
                  p_17_11 + o_16_11 +
                  p_18_10 + o_17_10 +
                  p_19_9 + o_18_9 +
                  c27 + o_19_8;
        let c28 = d28 >> 27;
        let d29 = p_10_19 + o_9_19 +
                  p_11_18 + o_10_18 +
                  p_12_17 + o_11_17 +
                  p_13_16 + o_12_16 +
                  p_14_15 + o_13_15 +
                  p_15_14 + o_14_14 +
                  p_16_13 + o_15_13 +
                  p_17_12 + o_16_12 +
                  p_18_11 + o_17_11 +
                  p_19_10 + o_18_10 +
                  c28 + o_19_9;
        let c29 = d29 >> 27;
        let d30 = p_11_19 + o_10_19 +
                  p_12_18 + o_11_18 +
                  p_13_17 + o_12_17 +
                  p_14_16 + o_13_16 +
                  p_15_15 + o_14_15 +
                  p_16_14 + o_15_14 +
                  p_17_13 + o_16_13 +
                  p_18_12 + o_17_12 +
                  p_19_11 + o_18_11 +
                  c29 + o_19_10;
        let c30 = d30 >> 27;
        let d31 = p_12_19 + o_11_19 +
                  p_13_18 + o_12_18 +
                  p_14_17 + o_13_17 +
                  p_15_16 + o_14_16 +
                  p_16_15 + o_15_15 +
                  p_17_14 + o_16_14 +
                  p_18_13 + o_17_13 +
                  p_19_12 + o_18_12 +
                  c30 + o_19_11;
        let c31 = d31 >> 27;
        let d32 = p_13_19 + o_12_19 +
                  p_14_18 + o_13_18 +
                  p_15_17 + o_14_17 +
                  p_16_16 + o_15_16 +
                  p_17_15 + o_16_15 +
                  p_18_14 + o_17_14 +
                  p_19_13 + o_18_13 +
                  c31 + o_19_12;
        let c32 = d32 >> 27;
        let d33 = p_14_19 + o_13_19 +
                  p_15_18 + o_14_18 +
                  p_16_17 + o_15_17 +
                  p_17_16 + o_16_16 +
                  p_18_15 + o_17_15 +
                  p_19_14 + o_18_14 +
                  c32 + o_19_13;
        let c33 = d33 >> 27;
        let d34 = p_15_19 + o_14_19 +
                  p_16_18 + o_15_18 +
                  p_17_17 + o_16_17 +
                  p_18_16 + o_17_16 +
                  p_19_15 + o_18_15 +
                  c33 + o_19_14;
        let c34 = d34 >> 27;
        let d35 = p_16_19 + o_15_19 +
                  p_17_18 + o_16_18 +
                  p_18_17 + o_17_17 +
                  p_19_16 + o_18_16 +
                  c34 + o_19_15;
        let c35 = d35 >> 27;
        let d36 = p_17_19 + o_16_19 +
                  p_18_18 + o_17_18 +
                  p_19_17 + o_18_17 +
                  c35 + o_19_16;
        let c36 = d36 >> 27;
        let d37 = p_18_19 + o_17_19 +
                  p_19_18 + o_18_18 +
                  c36 + o_19_17;
        let c37 = d37 >> 27;
        let d38 = p_19_19 + o_18_19 +
                  c37 + o_19_18;

        // Modular reduction by a pseudo-mersenne prime of the form 2^n - c.

        // These are the n low-order bits.
        let l0_0: u64 = (d0 & 0x07ffffff) as u64 |
                        ((d1 & 0x07ffffff) as u64) << 27;
        let l1_0: u64 = (d2 & 0x07ffffff) as u64 |
                        ((d3 & 0x07ffffff) as u64) << 27;
        let l2_0: u64 = (d4 & 0x07ffffff) as u64 |
                        ((d5 & 0x07ffffff) as u64) << 27;
        let l3_0: u64 = (d6 & 0x07ffffff) as u64 |
                        ((d7 & 0x07ffffff) as u64) << 27;
        let l4_0: u64 = (d8 & 0x07ffffff) as u64 |
                        ((d9 & 0x07ffffff) as u64) << 27;
        let l5_0: u64 = (d10 & 0x07ffffff) as u64 |
                        ((d11 & 0x07ffffff) as u64) << 27;
        let l6_0: u64 = (d12 & 0x07ffffff) as u64 |
                        ((d13 & 0x07ffffff) as u64) << 27;
        let l7_0: u64 = (d14 & 0x07ffffff) as u64 |
                        ((d15 & 0x07ffffff) as u64) << 27;
        let l8_0: u64 = (d16 & 0x07ffffff) as u64 |
                        ((d17 & 0x07ffffff) as u64) << 27;
        let l9_0: u64 = (d18 & 0x07ffffff) as u64 |
                        ((d19 & 0x0000007f) as u64) << 27;

        // Shift the high bits down into another n-bit number.
        let h0_0: u64 = ((d19 & 0x07ffff80) as u64) >> 7 |
                        ((d20 & 0x07ffffff) as u64) << 20 |
                        ((d21 & 0x0000007f) as u64) << 34;
        let h1_0: u64 = ((d21 & 0x07ffff80) as u64) >> 7 |
                        ((d22 & 0x07ffffff) as u64) << 20 |
                        ((d23 & 0x0000007f) as u64) << 34;
        let h2_0: u64 = ((d23 & 0x07ffff80) as u64) >> 7 |
                        ((d24 & 0x07ffffff) as u64) << 20 |
                        ((d25 & 0x0000007f) as u64) << 34;
        let h3_0: u64 = ((d25 & 0x07ffff80) as u64) >> 7 |
                        ((d26 & 0x07ffffff) as u64) << 20 |
                        ((d27 & 0x0000007f) as u64) << 34;
        let h4_0: u64 = ((d27 & 0x07ffff80) as u64) >> 7 |
                        ((d28 & 0x07ffffff) as u64) << 20 |
                        ((d29 & 0x0000007f) as u64) << 34;
        let h5_0: u64 = ((d29 & 0x07ffff80) as u64) >> 7 |
                        ((d30 & 0x07ffffff) as u64) << 20 |
                        ((d31 & 0x0000007f) as u64) << 34;
        let h6_0: u64 = ((d31 & 0x07ffff80) as u64) >> 7 |
                        ((d32 & 0x07ffffff) as u64) << 20 |
                        ((d33 & 0x0000007f) as u64) << 34;
        let h7_0: u64 = ((d33 & 0x07ffff80) as u64) >> 7 |
                        ((d34 & 0x07ffffff) as u64) << 20 |
                        ((d35 & 0x0000007f) as u64) << 34;
        let h8_0: u64 = ((d35 & 0x07ffff80) as u64) >> 7 |
                        ((d36 & 0x07ffffff) as u64) << 20 |
                        ((d37 & 0x0000007f) as u64) << 34;
        let h9_0: u64 = ((d37 & 0x07ffff80) as u64) >> 7 |
                        ((d38 & 0x07ffffff) as u64) << 20;

        // Normally, we multiply h by c, but since c = 1 here, we skip.

        // Add h and l.

        // Need kin_0
        let kin_0: u64 = h9_0 >> 27;
        let s0_0: u64 = l0_0 + h0_0 + kin_0;
        let k0_0: u64 = s0_0 >> 54;
        let s1_0: u64 = l1_0 + h1_0 + k0_0;
        let k1_0: u64 = s1_0 >> 54;
        let s2_0: u64 = l2_0 + h2_0 + k1_0;
        let k2_0: u64 = s2_0 >> 54;
        let s3_0: u64 = l3_0 + h3_0 + k2_0;
        let k3_0: u64 = s3_0 >> 54;
        let s4_0: u64 = l4_0 + h4_0 + k3_0;
        let k4_0: u64 = s4_0 >> 54;
        let s5_0: u64 = l5_0 + h5_0 + k4_0;
        let k5_0: u64 = s5_0 >> 54;
        let s6_0: u64 = l6_0 + h6_0 + k5_0;
        let k6_0: u64 = s6_0 >> 54;
        let s7_0: u64 = l7_0 + h7_0 + k6_0;
        let k7_0: u64 = s7_0 >> 54;
        let s8_0: u64 = l8_0 + h8_0 + k7_0;
        let k8_0: u64 = s8_0 >> 54;
        let s9_0: u64 = l9_0 + h9_0 + k8_0;

        self[0] = (s0_0 & 0x07ffffff) as u32;
        self[1] = ((s0_0 >> 27) & 0x07ffffff) as u32;
        self[2] = (s1_0 & 0x07ffffff) as u32;
        self[3] = ((s1_0 >> 27) & 0x07ffffff) as u32;
        self[4] = (s2_0 & 0x07ffffff) as u32;
        self[5] = ((s2_0 >> 27) & 0x07ffffff) as u32;
        self[6] = (s3_0 & 0x07ffffff) as u32;
        self[7] = ((s3_0 >> 27) & 0x07ffffff) as u32;
        self[8] = (s4_0 & 0x07ffffff) as u32;
        self[9] = ((s4_0 >> 27) & 0x07ffffff) as u32;
        self[10] = (s5_0 & 0x07ffffff) as u32;
        self[11] = ((s5_0 >> 27) & 0x07ffffff) as u32;
        self[12] = (s6_0 & 0x07ffffff) as u32;
        self[13] = ((s6_0 >> 27) & 0x07ffffff) as u32;
        self[14] = (s7_0 & 0x07ffffff) as u32;
        self[15] = ((s7_0 >> 27) & 0x07ffffff) as u32;
        self[16] = (s8_0 & 0x07ffffff) as u32;
        self[17] = ((s8_0 >> 27) & 0x07ffffff) as u32;
        self[18] = (s9_0 & 0x07ffffff) as u32;
        self[19] = (s9_0 >> 27) as u32;
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

#[cfg(test)]
mod tests {
    use fields::mod_e521_1::*;

    pub const TWO: Mod_e521_1 = Mod_e521_1([ 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]);

    pub const M_TWO: Mod_e521_1 =
        Mod_e521_1([ 0x07fffffd, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                     0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                     0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                     0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                     0x07ffffff, 0x07ffffff, 0x07ffffff, 0x000000ff ]);

    fn test_pack_unpack(expected: &[u8; 66]) {
        let mut unpacked = Mod_e521_1::unpack(expected);
        let actual = unpacked.pack();

        for i in 0..65 {
            assert!(expected[i] == actual[i]);
        }
    }

    fn test_unpack_pack(expected: &mut Mod_e521_1) {
        let bytes = expected.pack();
        let actual = Mod_e521_1::unpack(&bytes);

        for i in 0..19 {
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
        test_unpack_pack(&mut Mod_e521_1([ 0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000,
                                           0x07ffffff, 0x00000000 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x07ffffff,
                                           0x00000000, 0x000000ff ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x05555555,
                                           0x02aaaaaa, 0x00000055 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x02aaaaaa,
                                           0x05555555, 0x000000aa ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000,
                                           0x02aaaaaa, 0x00000000 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x02aaaaaa,
                                           0x00000000, 0x000000aa ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x05555555,
                                           0x07ffffff, 0x00000055 ]));
        test_unpack_pack(&mut Mod_e521_1([ 0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x07ffffff,
                                           0x05555555, 0x000000ff ]));
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
}
