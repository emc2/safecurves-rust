use std::clone::Clone;
use std::ops::Index;
use std::ops::IndexMut;

/// Elements of the finite field mod 2^511 - 187.  Used by the Curve41417
/// curve.  Uses 27-bit digits.

#[derive(Copy, Clone)]
pub struct Mod_e511_187(pub [u32; 19]);

impl PrimeField for Mod_e511_187 {
    fn zero() -> Mod_e511_187 {
        Mod_e511_187([ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                       0, 0, 0, 0, 0, 0, 0, 0, 0 ])
    }

    fn one() -> Mod_e511_187 {
        Mod_e511_187([ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                       0, 0, 0, 0, 0, 0, 0, 0, 0 ])
    }

    fn modulus() -> Mod_e511_187 {
        Mod_e511_187([ 0x07ffff45, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                       0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                       0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                       0x07ffffff, 0x07ffffff, 0x07ffffff, 0x07ffffff,
                       0x07ffffff, 0x07ffffff, 0x01ffffff ])
    }
}

impl Mod_e511_187 {
    fn pack(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];

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

        bytes;
    }

    fn unpack(bytes : &[u8; 64]) -> Mod_e511_187 {
        let mut out = Mod_e511_187([0u32; 19]);

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
                  (((bytes[63] as u32) << 18) & 0x03fc0000);

        out;
    }
}

impl IndexMut<usize> for Mod_e511_187 {
    fn index_mut<'a>(&'a mut self, idx : usize) -> &'a mut u32 {
        let ret : &'a mut u32 = &mut(self.0[idx]);
        ret
    }
}

impl Index<usize> for Mod_e511_187 {
    type Output = u32;

    fn index<'a>(&'a self, idx : usize) -> &'a u32 {
        let ret : &'a u32 = &(self.0[idx]);
        ret
    }
}
