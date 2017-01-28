use std::clone::Clone;
use std::ops::Index;
use std::ops::IndexMut;

/// Elements of the finite field mod 2^414 - 17.  Used by the Curve41417
/// curve.  Uses 28-bit digits.

#[derive(Copy, Clone)]
pub struct Mod_e414_17(pub [u32; 15]);

impl PrimeField for Mod_e414_17 {
    fn zero() -> Mod_e414_17 {
        Mod_e414_17([ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ])
    }

    fn one() -> Mod_e414_17 {
        Mod_e414_17([ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ])
    }

    fn modulus() -> Mod_e414_17 {
        Mod_e414_17([ 0x0fffffef, 0x0fffffff, 0x0fffffff, 0x0fffffff,
                      0x0fffffff, 0x0fffffff, 0x0fffffff, 0x0fffffff,
                      0x0fffffff, 0x0fffffff, 0x0fffffff, 0x0fffffff,
                      0x0fffffff, 0x0fffffff, 0x003fffff ])
    }
}

impl Mod_e414_17 {
    fn pack(&self) -> [u8; 48] {
        let mut bytes = [0u8; 48];

        bytes[0] = (self[0] & 0b11111111) as u8;
        bytes[1] = ((self[0] >> 8) & 0b11111111) as u8;
        bytes[2] = ((self[0] >> 16) & 0b11111111) as u8;
        bytes[3] = (((self[0] >> 24) & 0b00001111) |
                    ((self[1] << 4) & 0b11110000)) as u8;
        bytes[4] = ((self[1] >> 4) & 0b11111111) as u8;
        bytes[5] = ((self[1] >> 12) & 0b11111111) as u8;
        bytes[6] = ((self[1] >> 20) & 0b11111111) as u8;
        bytes[7] = (self[2] & 0b11111111) as u8;
        bytes[8] = ((self[2] >> 8) & 0b11111111) as u8;
        bytes[9] = ((self[2] >> 16) & 0b11111111) as u8;
        bytes[10] = (((self[2] >> 24) & 0b00001111) |
                     ((self[3] << 4) & 0b11110000)) as u8;
        bytes[11] = ((self[3] >> 4) & 0b11111111) as u8;
        bytes[12] = ((self[3] >> 12) & 0b11111111) as u8;
        bytes[13] = ((self[3] >> 20) & 0b11111111) as u8;
        bytes[14] = (self[4] & 0b11111111) as u8;
        bytes[15] = ((self[4] >> 8) & 0b11111111) as u8;
        bytes[16] = ((self[4] >> 16) & 0b11111111) as u8;
        bytes[17] = (((self[4] >> 24) & 0b00001111) |
                     ((self[5] << 4) & 0b11110000)) as u8;
        bytes[18] = ((self[5] >> 4) & 0b11111111) as u8;
        bytes[19] = ((self[5] >> 12) & 0b11111111) as u8;
        bytes[20] = ((self[5] >> 20) & 0b11111111) as u8;
        bytes[21] = (self[6] & 0b11111111) as u8;
        bytes[22] = ((self[6] >> 8) & 0b11111111) as u8;
        bytes[23] = ((self[6] >> 16) & 0b11111111) as u8;
        bytes[24] = (((self[6] >> 24) & 0b00001111) |
                     ((self[7] << 4) & 0b11110000)) as u8;
        bytes[25] = ((self[7] >> 4) & 0b11111111) as u8;
        bytes[26] = ((self[7] >> 12) & 0b11111111) as u8;
        bytes[27] = ((self[7] >> 20) & 0b11111111) as u8;
        bytes[28] = (self[8] & 0b11111111) as u8;
        bytes[29] = ((self[8] >> 8) & 0b11111111) as u8;
        bytes[30] = ((self[8] >> 16) & 0b11111111) as u8;
        bytes[31] = (((self[8] >> 24) & 0b00001111) |
                     ((self[9] << 4) & 0b11110000)) as u8;
        bytes[32] = ((self[9] >> 4) & 0b11111111) as u8;
        bytes[33] = ((self[9] >> 12) & 0b11111111) as u8;
        bytes[34] = ((self[9] >> 20) & 0b11111111) as u8;
        bytes[35] = (self[10] & 0b11111111) as u8;
        bytes[36] = ((self[10] >> 8) & 0b11111111) as u8;
        bytes[37] = ((self[10] >> 16) & 0b11111111) as u8;
        bytes[38] = (((self[10] >> 24) & 0b00001111) |
                     ((self[11] << 4) & 0b11110000)) as u8;
        bytes[39] = ((self[11] >> 4) & 0b11111111) as u8;
        bytes[40] = ((self[11] >> 12) & 0b11111111) as u8;
        bytes[41] = ((self[11] >> 20) & 0b11111111) as u8;
        bytes[42] = (self[12] & 0b11111111) as u8;
        bytes[43] = ((self[12] >> 8) & 0b11111111) as u8;
        bytes[44] = ((self[12] >> 16) & 0b11111111) as u8;
        bytes[45] = (((self[12] >> 24) & 0b00001111) |
                     ((self[13] << 4) & 0b11110000)) as u8;
        bytes[46] = ((self[13] >> 4) & 0b11111111) as u8;
        bytes[47] = ((self[13] >> 12) & 0b11111111) as u8;
        bytes[48] = ((self[13] >> 20) & 0b11111111) as u8;
        bytes[49] = (self[14] & 0b11111111) as u8;
        bytes[50] = ((self[14] >> 8) & 0b11111111) as u8;
        bytes[51] = ((self[14] >> 16) & 0b11111111) as u8;

        bytes;
    }

    fn unpack(bytes : &[u8; 48]) -> Mod_e414_17 {
        let mut out = Mod_e414_17([0u32; 15]);

        out[0] = ((bytes[0] as u32) & 0x000000ff) |
                 (((bytes[1] as u32) << 8) & 0x0000ff00) |
                 (((bytes[2] as u32) << 16) & 0x00ff0000) |
                 (((bytes[3] as u32) << 24) & 0x0f000000);
        out[1] = (((bytes[3] as u32) >> 4) & 0x0000000f) |
                 (((bytes[4] as u32) << 4) & 0x00000ff0) |
                 (((bytes[5] as u32) << 12) & 0x000ff000) |
                 (((bytes[6] as u32) << 20) & 0x0ff00000);
        out[2] = ((bytes[7] as u32) & 0x000000ff) |
                 (((bytes[8] as u32) << 8) & 0x0000ff00) |
                 (((bytes[9] as u32) << 16) & 0x00ff0000) |
                 (((bytes[10] as u32) << 24) & 0x0f000000);
        out[3] = (((bytes[10] as u32) >> 4) & 0x0000000f) |
                 (((bytes[11] as u32) << 4) & 0x00000ff0) |
                 (((bytes[12] as u32) << 12) & 0x000ff000) |
                 (((bytes[13] as u32) << 20) & 0x0ff00000);
        out[4] = ((bytes[14] as u32) & 0x000000ff) |
                 (((bytes[15] as u32) << 8) & 0x0000ff00) |
                 (((bytes[16] as u32) << 16) & 0x00ff0000) |
                 (((bytes[17] as u32) << 24) & 0x0f000000);
        out[5] = (((bytes[17] as u32) >> 4) & 0x0000000f) |
                 (((bytes[18] as u32) << 4) & 0x00000ff0) |
                 (((bytes[19] as u32) << 12) & 0x000ff000) |
                 (((bytes[20] as u32) << 20) & 0x0ff00000);
        out[6] = ((bytes[21] as u32) & 0x000000ff) |
                 (((bytes[22] as u32) << 8) & 0x0000ff00) |
                 (((bytes[23] as u32) << 16) & 0x00ff0000) |
                 (((bytes[24] as u32) << 24) & 0x0f000000);
        out[7] = (((bytes[24] as u32) >> 4) & 0x0000000f) |
                 (((bytes[25] as u32) << 4) & 0x00000ff0) |
                 (((bytes[26] as u32) << 12) & 0x000ff000) |
                 (((bytes[27] as u32) << 20) & 0x0ff00000);
        out[8] = ((bytes[28] as u32) & 0x000000ff) |
                 (((bytes[29] as u32) << 8) & 0x0000ff00) |
                 (((bytes[30] as u32) << 16) & 0x00ff0000) |
                 (((bytes[31] as u32) << 24) & 0x0f000000);
        out[9] = (((bytes[31] as u32) >> 4) & 0x0000000f) |
                 (((bytes[32] as u32) << 4) & 0x00000ff0) |
                 (((bytes[33] as u32) << 12) & 0x000ff000) |
                 (((bytes[34] as u32) << 20) & 0x0ff00000);
        out[10] = ((bytes[35] as u32) & 0x000000ff) |
                  (((bytes[36] as u32) << 8) & 0x0000ff00) |
                  (((bytes[37] as u32) << 16) & 0x00ff0000) |
                  (((bytes[38] as u32) << 24) & 0x0f000000);
        out[11] = (((bytes[38] as u32) >> 4) & 0x0000000f) |
                  (((bytes[39] as u32) << 4) & 0x00000ff0) |
                  (((bytes[40] as u32) << 12) & 0x000ff000) |
                  (((bytes[41] as u32) << 20) & 0x0ff00000);
        out[12] = ((bytes[42] as u32) & 0x000000ff) |
                  (((bytes[43] as u32) << 8) & 0x0000ff00) |
                  (((bytes[44] as u32) << 16) & 0x00ff0000) |
                  (((bytes[45] as u32) << 24) & 0x0f000000);
        out[13] = (((bytes[45] as u32) >> 4) & 0x0000000f) |
                  (((bytes[46] as u32) << 4) & 0x00000ff0) |
                  (((bytes[47] as u32) << 12) & 0x000ff000) |
                  (((bytes[48] as u32) << 20) & 0x0ff00000);
        out[14] = ((bytes[49] as u32) & 0x000000ff) |
                  (((bytes[50] as u32) << 8) & 0x0000ff00) |
                  (((bytes[51] as u32) << 16) & 0x00ff0000);

        out;
    }
}

impl IndexMut<usize> for Mod_e414_17 {
    fn index_mut<'a>(&'a mut self, idx : usize) -> &'a mut u32 {
        let ret : &'a mut u32 = &mut(self.0[idx]);
        ret
    }
}

impl Index<usize> for Mod_e414_17 {
    type Output = u32;

    fn index<'a>(&'a self, idx : usize) -> &'a u32 {
        let ret : &'a u32 = &(self.0[idx]);
        ret
    }
}
