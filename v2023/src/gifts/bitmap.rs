use std::ops::{BitAnd, BitOr, Shl, Shr};
use std::{fmt, u128};

/// Compact boolean values for super fast arithmetic.
///
/// ```
/// use aoc::bitmap::BitMap;
///
/// let mut bits = BitMap::default() | 12; // 01100
/// let mut bots = BitMap::default() | 1;  // 00001
/// bits = bits >> 2;
/// bots = bots << 1 | 1;
/// assert_eq!(bits, bots);
/// ```
#[derive(Debug, Default, PartialEq)]
pub struct BitMap {
    n: [u128; 2],
}

impl BitMap {
    /// Retreive bits between range counting from the right.
    ///
    /// ```
    /// use aoc::bitmap::BitMap;
    ///
    /// let bits = BitMap::default() | 15; // 01111
    /// let bots = BitMap::default() | 6;  // 00110
    /// assert_eq!(bits.range(2, 1), bots);
    /// ```
    pub fn range(&self, left: usize, right: usize) -> BitMap {
        let padded_left = (255 - left).try_into().unwrap();
        let padded_right = (255 - right).try_into().unwrap();
        BitMap::mask(padded_left, padded_right) & self
    }

    /// Creates a bitmap with the provided values.
    ///
    /// ```
    /// use aoc::bitmap::BitMap;
    ///
    /// let bits = BitMap::new(0, 0);                 // 00000
    /// let bots = BitMap::new(u128::MAX, u128::MAX); // 11111
    /// assert_eq!(bits, BitMap::default());
    /// assert_ne!(bots, BitMap::default());
    /// ```
    fn new(left: u128, right: u128) -> BitMap {
        BitMap { n: [left, right] }
    }

    /// Fill a range with truthful values from the left.
    ///
    /// ```
    /// use aoc::bitmap::BitMap;
    ///
    /// let bits = BitMap::mask(2, 4); // 001110...00
    /// assert_eq!(bits, BitMap::new(7 << 123, 0));
    /// ```
    fn mask(start: u32, end: u32) -> BitMap {
        let left_mask = u128::MAX.checked_shr(start).unwrap_or(0)
            & u128::MAX
                .checked_shl(128_u32.saturating_sub(end + 1))
                .unwrap_or(0);
        let right_mask = u128::MAX
            .checked_shr(start.saturating_sub(128))
            .unwrap_or(0)
            & u128::MAX.checked_shl(255 - end).unwrap_or(0);
        BitMap::new(left_mask, right_mask)
    }
}

impl BitAnd<&BitMap> for BitMap {
    type Output = BitMap;
    fn bitand(self, rhs: &BitMap) -> Self::Output {
        BitMap::new(self.n[0] & rhs.n[0], self.n[1] & rhs.n[1])
    }
}

impl BitOr for BitMap {
    type Output = BitMap;
    fn bitor(self, rhs: BitMap) -> Self::Output {
        BitMap::new(self.n[0] | rhs.n[0], self.n[1] | rhs.n[1])
    }
}

impl BitOr<u128> for BitMap {
    type Output = BitMap;
    fn bitor(self, rhs: u128) -> Self::Output {
        BitMap::new(self.n[0], self.n[1] | rhs)
    }
}

impl Shl<u32> for BitMap {
    type Output = BitMap;
    fn shl(self, rhs: u32) -> Self::Output {
        let rotation = self.n[1].rotate_left(rhs);
        let wrap = 2_u128.pow(rhs) - 1;
        let left = self.n[0].wrapping_shl(rhs) | (rotation & wrap);
        let right = rotation & !wrap;
        BitMap::new(left, right)
    }
}

impl Shr<u32> for BitMap {
    type Output = BitMap;
    fn shr(self, rhs: u32) -> Self::Output {
        let rotation = self.n[0].rotate_right(rhs);
        let wrap = (2_u128.saturating_pow(rhs) - 1).rotate_right(rhs);
        let left = rotation & !wrap;
        let right = self.n[1].wrapping_shr(rhs) | (rotation & wrap);
        BitMap::new(left, right)
    }
}

impl fmt::Display for BitMap {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{:0128b}{:0128b}", self.n[0], self.n[1])
    }
}

#[test]
fn test_bitmap_range() {
    // [011001101 ... 110, 1010 ... 01010]
    // [  ^^^^^^       ^^^^^^^        ^^^]
    //  253    248   129     125      2 0
    let n = BitMap::new((205 << 119) + 6, (5 << 125) + 10);
    assert_eq!(n.range(2, 0), BitMap::new(0, 2));
    assert_eq!(n.range(2, 1), BitMap::new(0, 2));
    assert_eq!(n.range(129, 125), BitMap::new(2, 5 << 125));
    assert_eq!(n.range(253, 248), BitMap::new(38 << 120, 0));
}

#[test]
fn test_bitmap_mask() {
    // [001111110 ... 001, 1110 ... 00111]
    // [  ^^^^^^        ^^^^^^        ^^^]
    //    2    7      127    130    253 255
    assert_eq!(BitMap::mask(2, 7), BitMap::new(63 << 120, 0));
    assert_eq!(BitMap::mask(127, 130), BitMap::new(1, 7 << 125));
    assert_eq!(BitMap::mask(253, 255), BitMap::new(0, 7));
}

#[test]
fn test_bitmap_and() {
    let mut n = BitMap::new(u128::MAX, u128::MAX);
    n = BitMap::new(0, u128::MAX) & &n;
    assert_eq!(n, BitMap::new(0, u128::MAX));
    n = BitMap::new(u128::MAX, 0) & &n;
    assert_eq!(n, BitMap::new(0, 0));
}

#[test]
fn test_bitmap_or() {
    let mut n = BitMap::new(1, 0);
    n = n | 1;
    assert_eq!(n, BitMap::new(1, 1));
    n = n | 2;
    assert_eq!(n, BitMap::new(1, 3));
    n = BitMap::new(3, 3) | BitMap::new(8, 4);
    assert_eq!(n, BitMap::new(11, 7));
}

#[test]
fn test_bitmap_shift_left() {
    let mut n = BitMap::new(0, 1);
    assert_eq!(n << 3, BitMap::new(0, 8));
    n = BitMap::new(1, 0);
    assert_eq!(n << 2, BitMap::new(4, 0));
    n = BitMap::new(0, 1 << 127);
    assert_eq!(n << 1, BitMap::new(1, 0));
    n = BitMap::new(1 << 127, 0);
    assert_eq!(n << 1, BitMap::new(0, 0));
    n = BitMap::new(1, (1 << 126) + 2);
    assert_eq!(n << 2, BitMap::new(5, 8));
}

#[test]
fn test_bitmap_shift_right() {
    let mut n = BitMap::new(0, 8);
    assert_eq!(n >> 3, BitMap::new(0, 1));
    n = BitMap::new(4, 0);
    assert_eq!(n >> 2, BitMap::new(1, 0));
    n = BitMap::new(1, 0);
    assert_eq!(n >> 1, BitMap::new(0, 1 << 127));
    n = BitMap::new(0, 1);
    assert_eq!(n >> 1, BitMap::new(0, 0));
    n = BitMap::new((1 << 127) + 6, (1 << 127) + 3);
    assert_eq!(n >> 2, BitMap::new((1 << 125) + 1, 5 << 125));
}

#[test]
fn test_bitmap_display() {
    let zero: BitMap = BitMap::default();
    let n = BitMap::new(1, 0);
    let max = BitMap::new(u128::MAX, u128::MAX);
    assert_eq!(format!("{}", zero), format!("{:0128b}{:0128b}", 0, 0));
    assert_eq!(format!("{}", n), format!("{:0128b}{:0128b}", 1, 0));
    assert_eq!(
        format!("{}", max),
        format!("{:0128b}{:0128b}", u128::MAX, u128::MAX)
    );
}
