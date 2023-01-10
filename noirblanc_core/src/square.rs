use core::num::NonZeroU8;

/// A square.
///
/// [`Square`] and <code>[Option]<[Square]></code> are both 1-byte data types.
/// Because they are cheap to copy, they implement [`Copy`].
///
/// Valid values are 1..=64.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Square(NonZeroU8);

impl Square {
    /// Creates a new [`Square`] with given `file` and `rank`.
    ///
    /// `file` and `rank` must be between 1 and 8 (both inclusive).
    /// If this condition is not met, this function returns None.
    #[inline(always)]
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file.wrapping_sub(1) >= 8 || rank.wrapping_sub(1) >= 8 {
            return None;
        }
        // Safety: file >= 1 && rank >= 1 implies file + rank * 8 - 8 >= 1
        Some(Square(unsafe {
            NonZeroU8::new_unchecked(file + rank * 8 - 8)
        }))
    }

    /// C interface to [`Square::new`].
    #[no_mangle]
    pub extern "C" fn Square_new(file: u8, rank: u8) -> OptionSquare {
        OptionSquare::from(Square::new(file, rank))
    }

    /// Converts [`u8`] to [`Square`] without checking.
    ///
    /// # Safety
    /// `value` must be in range 1..=64
    #[inline(always)]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        if !matches!(value, 1..=64) {
            core::hint::unreachable_unchecked();
        }
        Self(NonZeroU8::new_unchecked(value))
    }

    /// C interface to [`Square::from_u8_unchecked`].
    ///
    /// # Safety
    /// `value` must be in range 1..=64
    #[inline(always)]
    #[no_mangle]
    pub unsafe extern "C" fn Square_from_u8_unchecked(value: u8) -> Self {
        if !matches!(value, 1..=64) {
            core::hint::unreachable_unchecked();
        }
        Self(NonZeroU8::new_unchecked(value))
    }

    /// Finds the index of `self` in range `1..=64`.
    /// It is guaranteed that the result is equal to the internal representation, `file + rank * 8 - 8`.
    ///
    /// Examples:
    /// ```
    /// use noirblanc_core::Square;
    /// assert_eq!(Square::new(3, 6).unwrap().index(), 43);
    /// ```
    #[inline(always)]
    #[export_name = "Square_index"]
    pub extern "C" fn index(self) -> u8 {
        self.sanity_check();
        self.0.get()
    }

    /// Finds the file in range `1..=8`.
    ///
    /// Examples:
    /// ```
    /// use noirblanc_core::Square;
    /// assert_eq!(Square::new(3, 6).unwrap().file(), 3);
    /// ```
    #[inline(always)]
    #[export_name = "Square_file"]
    pub extern "C" fn file(self) -> u8 {
        self.sanity_check();
        ((self.0.get() - 1) & 7) + 1
    }
    /// Finds the rank in range `1..=8`.
    ///
    /// Examples:
    /// ```
    /// use noirblanc_core::Square;
    /// assert_eq!(Square::new(3, 6).unwrap().rank(), 6);
    /// ```
    #[inline(always)]
    #[export_name = "Square_rank"]
    pub extern "C" fn rank(self) -> u8 {
        self.sanity_check();
        ((self.0.get() - 1) >> 3) + 1
    }

    #[inline(always)]
    pub const fn array_index(self) -> usize {
        self.sanity_check();
        let result = (self.0.get() - 1) as usize;
        // Safety: result < Square::NUM always holds
        if result >= Self::NUM {
            unsafe { core::hint::unreachable_unchecked() };
        }
        result
    }

    pub const NUM: usize = 64;

    // Check if self.0 is in 1..=64
    #[inline(always)]
    const fn sanity_check(self) {
        debug_assert!(matches!(self.0.get(), 1..=64));
        // Safety: for any valid Square, its representation must be in 1..=64.
        if !matches!(self.0.get(), 1..=64) {
            unsafe { core::hint::unreachable_unchecked() }
        }
    }
}

/// C interface of <code>[Option]<[Square]></code>.
///
/// This type is provided for C interoperability.
/// cbindgen cannot deduce that <code>[Option]<[Square]></code> can be represented by `uint8_t` in C, so we need to define the bridge type.
/// Users of this type should convert to/from <code>[Option]<[Square]></code>.
///
/// See: <https://github.com/eqrion/cbindgen/issues/326>.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct OptionSquare(u8);

impl From<Option<Square>> for OptionSquare {
    #[inline(always)]
    fn from(arg: Option<Square>) -> Self {
        Self(match arg {
            Some(result) => result.0.get(),
            None => 0,
        })
    }
}

impl From<OptionSquare> for Option<Square> {
    #[inline(always)]
    fn from(arg: OptionSquare) -> Self {
        Some(Square(NonZeroU8::new(arg.0)?))
    }
}

impl core::fmt::Display for Square {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let file = self.file();
        let rank = self.rank();
        write!(f, "{}{}", b"abcdefgh"[file as usize - 1] as char, rank)
    }
}
