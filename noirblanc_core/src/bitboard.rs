use crate::Square;

/**
 * The implementation is done in reference to https://github.com/koba-e964/othello-ai/blob/master/CBoard.hs, which uses routines that are originally in edax.
 */

/// A set of squares.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// Creates an empty [`Bitboard`].
    #[inline(always)]
    pub const fn new() -> Self {
        Self(0)
    }
    /// Creates a singleton.
    #[inline(always)]
    pub const fn singleton(sq: Square) -> Self {
        Self(1 << sq.array_index())
    }

    /// Counts how many squares are in `self`.
    #[inline(always)]
    pub const fn count(self) -> i16 {
        self.0.count_ones() as i16
    }

    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn pop(&mut self) -> Option<Square> {
        let repr = self.0;
        if repr == 0 {
            return None;
        }
        let pos = repr.trailing_zeros();
        // Safety: 1 <= pos+1 <= 64
        let square = unsafe { Square::from_u8_unchecked(pos as u8 + 1) };
        let newrepr = repr & repr.wrapping_sub(1);
        *self = Self(newrepr);
        Some(square)
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(64))
    }
}
