/// Trait of things that can be packed into byte arrays.
pub trait Pack {
    /// Deserialize a little-endian byte array into a value.  The byte
    /// array must contain a number less than the modulus.
    fn unpack(&mut self, bytes: &[u8]);

    /// Deserialize a little-endian byte array into a value.  The byte
    /// array must contain a number less than the modulus.
    fn unpacked(bytes: &[u8]) -> Self;

    /// Serialize as a little-endian byte array.  This has the effect
    /// of normalizing the representation.
    fn pack(&mut self, &mut [u8]);

    /// Serialize an already normalized number as a little-endian byte
    /// array.  This must only be used on a normalized value.
    fn pack_normalized(&self, &mut [u8]);

    /// Get the number of bytes in the packed representation.
    fn nbytes() -> i32;
}
