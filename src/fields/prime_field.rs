/* Constant-time, carry-agnostic implementation of
 *
 * We use partially-filled 32-bit unsigned integers ordered from
 * low-ordered to high-ordered, regardless of the word-level bit
 * ordering.  We leave space to be able to do enough add operations
 * without overflows to implement multiply.
 *
 * The multiply algorithm on an n-digit number with m-bit digits will,
 * in the worst case, end up adding n * (2^m - 2) + n, which is
 * (n - 1)2^m + (2^m - 1).  Thus, the largest value n can have is log2(m).
 *
 *
 */

pub const NBITS : [u64; 32] = [0x1 * 32,
                               0x2 * 31,
                               0x4 * 30,
                               0x8 * 29,
                               0x10 * 28,
                               0x20 * 27,
                               0x40 * 26,
                               0x80 * 25,
                               0x100 * 24,
                               0x200 * 23,
                               0x400 * 22,
                               0x800 * 21,
                               0x1000 * 20,
                               0x2000 * 19,
                               0x4000 * 18,
                               0x8000 * 17,
                               0x10000 * 16,
                               0x20000 * 15,
                               0x40000 * 14,
                               0x80000 * 13,
                               0x100000 * 12,
                               0x200000 * 11,
                               0x400000 * 10,
                               0x800000 * 9,
                               0x1000000 * 8,
                               0x2000000 * 7,
                               0x4000000 * 6,
                               0x8000000 * 5,
                               0x10000000 * 4,
                               0x20000000 * 3,
                               0x40000000 * 2,
                               0x80000000 * 1];

pub trait PrimeField {
    fn zero() -> Self;
    fn one() -> Self;
    fn m_one() -> Self;
    fn modulus() -> Self;
}
