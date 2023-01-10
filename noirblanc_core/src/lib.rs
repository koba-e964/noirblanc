pub mod bitboard;
mod color;
pub mod position;
mod square;

#[doc(inline)]
pub use bitboard::*;
#[doc(inline)]
pub use color::*;
#[doc(inline)]
pub use position::*;
#[doc(inline)]
pub use square::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
