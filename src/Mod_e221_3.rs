mod prime_field;

use prime_field::PrimeField;
use std::clone::Clone;
use std::ops::AddAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Neg;

/// Elements of the finite field mod 2^221 - 3.  Used by the M-221
/// curve.  Uses 29-bit digits.

#[derive(Copy, Clone)]
pub struct Mod_e221_3(pub [u32; 8]);

const modulus : Mod_e221_3 =
    Mod_e221_3([ 0x1ffffffd, 0x1fffffff, 0x1fffffff, 0x1fffffff,
                 0x1fffffff, 0x1fffffff, 0x1fffffff, 0x0003ffff ]);

impl PrimeField for Mod_e221_3 {
    fn zero() -> Mod_e221_3 {
        Mod_e221_3([ 0, 0, 0, 0, 0, 0, 0, 0 ])
    }

    fn one() -> Mod_e221_3 {
        Mod_e221_3([ 1, 0, 0, 0, 0, 0, 0, 0 ])
    }

    fn modulus() -> Mod_e221_3 {
        modulus
    }
}

impl Index<usize> for Mod_e221_3 {
    type Output = u32;

    fn index<'a>(&'a self, idx : usize) -> &'a u32 {
        let ret : &'a u32 = &(self.0[idx]);
        ret
    }
}

impl IndexMut<usize> for Mod_e221_3 {
    fn index_mut<'a>(&'a mut self, idx : usize) -> &'a mut u32 {
        let ret : &'a mut u32 = &mut(self.0[idx]);
        ret
    }
}

impl Mod_e221_3 {
    fn pack(&self) -> [u8; 28] {
        let mut bytes = [0u8; 28];

        bytes[0] = (self[0] & 0b11111111) as u8;
        bytes[1] = ((self[0] >> 8) & 0b11111111) as u8;
        bytes[2] = ((self[0] >> 16) & 0b11111111) as u8;
        bytes[3] = (((self[0] >> 24) & 0b00011111) |
                    ((self[1] << 5) & 0b11100000)) as u8;
        bytes[4] = ((self[1] >> 3) & 0b11111111) as u8;
        bytes[5] = ((self[1] >> 11) & 0b11111111) as u8;
        bytes[6] = ((self[1] >> 19) & 0b11111111) as u8;
        bytes[7] = (((self[1] >> 27) & 0b00000011) |
                    ((self[2] << 2) & 0b11111100)) as u8;
        bytes[8] = ((self[2] >> 6) & 0b11111111) as u8;
        bytes[9] = ((self[2] >> 14) & 0b11111111) as u8;
        bytes[10] = (((self[2] >> 22) & 0b01111111) |
                     ((self[3] << 7) & 0b10000000)) as u8;
        bytes[11] = ((self[3] >> 1) & 0b11111111) as u8;
        bytes[12] = ((self[3] >> 9) & 0b11111111) as u8;
        bytes[13] = ((self[3] >> 17) & 0b11111111) as u8;
        bytes[14] = (((self[3] >> 25) & 0b00001111) |
                     ((self[4] << 4) & 0b11110000)) as u8;
        bytes[15] = ((self[4] >> 4) & 0b11111111) as u8;
        bytes[16] = ((self[4] >> 12) & 0b11111111) as u8;
        bytes[17] = ((self[4] >> 20) & 0b11111111) as u8;
        bytes[18] = (((self[4] >> 28) & 0b00000001) |
                     ((self[5] << 1) & 0b11111110)) as u8;
        bytes[19] = ((self[5] >> 7) & 0b11111111) as u8;
        bytes[20] = ((self[5] >> 15) & 0b11111111) as u8;
        bytes[21] = (((self[5] >> 23) & 0b00111111) |
                     ((self[6] << 6) & 0b11000000)) as u8;
        bytes[22] = ((self[6] >> 2) & 0b11111111) as u8;
        bytes[23] = ((self[6] >> 10) & 0b11111111) as u8;
        bytes[24] = ((self[6] >> 18) & 0b11111111) as u8;
        bytes[25] = (((self[6] >> 26) & 0b00000111) |
                     ((self[7] << 3) & 0b11111000)) as u8;
        bytes[26] = ((self[7] >> 5) & 0b11111111) as u8;
        bytes[27] = ((self[7] >> 13) & 0b11111111) as u8;

        bytes;
    }

    fn unpack(bytes : &[u8; 28]) -> Mod_e221_3 {
        let mut out = Mod_e221_3([0u32; 8]);

        out[0] = ((bytes[0] as u32) & 0x000000ff) |
                 (((bytes[1] as u32) << 8) & 0x0000ff00) |
                 (((bytes[2] as u32) << 16) & 0x00ff0000) |
                 (((bytes[3] as u32) << 24) & 0x1f000000);
        out[1] = (((bytes[3] as u32) >> 5) & 0x00000007) |
                 (((bytes[4] as u32) << 3) & 0x000007f8) |
                 (((bytes[5] as u32) << 11) & 0x0007f800) |
                 (((bytes[6] as u32) << 19) & 0x07f80000) |
                 (((bytes[7] as u32) << 27) & 0x18000000);
        out[2] = (((bytes[7] as u32) >> 2) & 0x0000003f) |
                 (((bytes[8] as u32) << 6) & 0x00003fc0) |
                 (((bytes[9] as u32) << 14) & 0x003fc000) |
                 (((bytes[10] as u32) << 22) & 0x1fc00000);
        out[3] = (((bytes[10] as u32) >> 7) & 0x00000001) |
                 (((bytes[11] as u32) << 1) & 0x000001fe) |
                 (((bytes[12] as u32) << 9) & 0x0001fe00) |
                 (((bytes[13] as u32) << 17) & 0x01fec000) |
                 (((bytes[14] as u32) << 25) & 0x1e000000);
        out[4] = (((bytes[14] as u32) >> 4) & 0x0000000f) |
                 (((bytes[15] as u32) << 4) & 0x00000ff0) |
                 (((bytes[16] as u32) << 12) & 0x000ff000) |
                 (((bytes[17] as u32) << 20) & 0x0ff00000) |
                 (((bytes[18] as u32) << 28) & 0x10000000);
        out[5] = (((bytes[18] as u32) >> 1) & 0x0000007f) |
                 (((bytes[19] as u32) << 7) & 0x00007f10) |
                 (((bytes[20] as u32) << 15) & 0x007f1000) |
                 (((bytes[21] as u32) << 23) & 0x1f100000);
        out[6] = (((bytes[21] as u32) >> 6) & 0x00000003) |
                 (((bytes[22] as u32) << 2) & 0x000003fc) |
                 (((bytes[23] as u32) << 10)& 0x0003fc00) |
                 (((bytes[24] as u32) << 18) & 0x03fc0000) |
                 (((bytes[25] as u32) << 26) & 0x1c000000);
        out[7] = (((bytes[25] as u32) >> 3) & 0x0000001f) |
                 (((bytes[26] as u32) << 5) & 0x00001fe0) |
                 (((bytes[27] as u32) << 13) & 0x001fe000);

        out;
    }
}

impl<'a> Neg for &'a Mod_e221_3 {
    type Output = Mod_e221_3;

    fn neg(self) -> Mod_e221_3 {
        // We don't need to worry about borrowing, because only the
        // last word of the modulus has a value smaller than
        // 0x1fffffff.
        self[7] = modulus[7] - self[7];
        self[6] = modulus[6] - self[6];
        self[5] = modulus[5] - self[5];
        self[4] = modulus[4] - self[4];
        self[3] = modulus[3] - self[3];
        self[2] = modulus[2] - self[2];
        self[1] = modulus[1] - self[1];
        self[0] = modulus[0] - self[0];
    }
}

impl<'b> AddAssign<&'b Mod_e221_3> for Mod_e221_3 {
    fn add_assign(&mut self, a : &'b Mod_e221_3) {
        let cin = self[7] >> 0x00040000;
        let s0 = self[0] + a[0] + (cin * 3);
        let c0 = s0 >> 29;
        let s1 = self[1] + a[1] + c0;
        let c1 = s1 >> 29;
        let s2 = self[2] + a[2] + c1;
        let c2 = s2 >> 29;
        let s3 = self[3] + a[3] + c2;
        let c3 = s3 >> 29;
        let s4 = self[4] + a[4] + c3;
        let c4 = s4 >> 29;
        let s5 = self[5] + a[5] + c4;
        let c5 = s5 >> 29;
        let s6 = self[6] + a[6] + c5;
        let c6 = s6 >> 29;
        let s7 = self[7] + a[7] + c6;
        let c7 = s7 >> 29;

        self[0] = s0 & 0x1fffffff;
        self[1] = s1 & 0x1fffffff;
        self[2] = s2 & 0x1fffffff;
        self[3] = s3 & 0x1fffffff;
        self[4] = s4 & 0x1fffffff;
        self[5] = s5 & 0x1fffffff;
        self[6] = s6 & 0x1fffffff;
        self[7] = s7 & 0x1fffffff;

        // Do modular reduction
    }
}
