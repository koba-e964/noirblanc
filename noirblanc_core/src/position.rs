use crate::{Bitboard, Color};

// The implementation is done in reference to https://github.com/koba-e964/othello-ai/blob/master/CBoard.hs, which uses routines that are originally in edax.

/// A position of reversi.
///
/// It is a triple of (black stones, white stones, side to play).
#[derive(Clone, Copy)]
pub struct Position(pub Bitboard, pub Bitboard, pub Color);

impl Position {
    #[inline(always)]
    pub const fn startpos() -> Self {
        Self(
            Bitboard(0x8_1000_0000),
            Bitboard(0x10_0800_0000),
            Color::Black,
        )
    }

    #[inline(always)]
    pub const fn side_to_play(self) -> Color {
        self.2
    }

    #[inline]
    pub const fn count_disks(self, color: Color) -> i16 {
        let Position(bl, wh, _turn) = self;
        match color {
            Color::Black => bl.count(),
            Color::White => wh.count(),
        }
    }

    pub fn pass(&mut self) {
        self.2 = self.2.flip();
    }

    pub fn make_move(&mut self, disk: Bitboard) {
        let Self(bl, wh, side) = *self;
        *self = match side {
            Color::Black => {
                let (newbl, newwh) = move_bit_board(bl, wh, disk);
                Self(newbl, newwh, Color::White)
            }
            Color::White => {
                let (newwh, newbl) = move_bit_board(wh, bl, disk);
                Self(newbl, newwh, Color::Black)
            }
        };
    }

    pub fn valid_moves(self) -> Bitboard {
        let Self(bl, wh, side) = self;
        match side {
            Color::Black => valid_moves_set(bl, wh),
            Color::White => valid_moves_set(wh, bl),
        }
    }
}

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let &Position(bl, wh, turn) = self;
        let mut ret = (" |A B C D E F G H \n-+----------------\n").to_string();
        for row in 1..9 {
            let mut board_line = format!("{}|", row);
            for col in 1..9 {
                let pos = (row - 1) * 8 + col - 1; // row first
                let mask = 1u64 << pos;
                let c = if (bl.0 & mask) != 0 {
                    'X'
                } else if (wh.0 & mask) != 0 {
                    'O'
                } else {
                    ' '
                };
                board_line = format!("{}{} ", board_line, c);
            }
            ret = format!("{}{}\n", ret, board_line);
        }
        write!(
            f,
            "{}  (X: Black,  O: White)\n{} to move",
            ret,
            match turn {
                Color::Black => "Black",
                Color::White => "White",
            },
        )
    }
}

impl core::default::Default for Position {
    fn default() -> Self {
        Self::startpos()
    }
}

pub const fn get_score_diff(light: u64, dark: u64) -> i16 {
    light.count_ones() as i16 - dark.count_ones() as i16
}

pub const fn get_tempo(light: u64, dark: u64) -> i16 {
    light.count_ones() as i16 + dark.count_ones() as i16
}

/// Port from <https://github.com/koba-e964/othello-ai/blob/master/CBoard.hs>
///
/// `disk` must be a singleton.
pub fn move_bit_board(my: Bitboard, opp: Bitboard, disk: Bitboard) -> (Bitboard, Bitboard) {
    debug_assert_eq!(disk.count(), 1);
    let val = flippable_indices_set(my.0, opp.0, disk.0);
    (Bitboard(my.0 | val | disk.0), Bitboard(opp.0 & !val))
}

/// set of valid moves represented by Places
///
/// reference: <https://github.com/abulmo/edax-reversi/blob/v4.6/src/board.c>
pub fn valid_moves_set(bl: Bitboard, wh: Bitboard) -> Bitboard {
    let bl = bl.0;
    let wh = wh.0;
    let mask = wh & 0x7e7e7e7e7e7e7e7e;
    let r1 = valid_moves_set_sub(bl, mask, 1);
    let r2 = valid_moves_set_sub(bl, wh, 8);
    let r3 = valid_moves_set_sub(bl, mask, 7);
    let r4 = valid_moves_set_sub(bl, mask, 9);
    Bitboard((r1 | r2 | r3 | r4) & !(bl | wh))
}

fn valid_moves_set_sub(my: u64, mask: u64, dir: usize) -> u64 {
    let dir2 = dir + dir;
    let fl1 = mask & (my << dir);
    let fr1 = mask & (my >> dir);
    let fl2 = fl1 | (mask & (fl1 << dir));
    let fr2 = fr1 | (mask & (fr1 >> dir));
    let maskl = mask & (mask << dir);
    let maskr = mask & (mask >> dir);
    let fl3 = fl2 | maskl & (fl2 << dir2);
    let fr3 = fr2 | maskr & (fr2 >> dir2);
    let fl4 = fl3 | maskl & (fl3 << dir2);
    let fr4 = fr3 | maskr & (fr3 >> dir2);
    (fl4 << dir) | (fr4 >> dir)
}

#[derive(Clone, Copy)]
struct Transfer(u64, usize);

const TRANSFERS_RIGHT: [Transfer; 4] = [
    Transfer(0xffffffffffffff00, 8), // up
    Transfer(0xfefefefefefefe00, 9), // up left
    Transfer(0x7f7f7f7f7f7f7f00, 7), // up right
    Transfer(0xfefefefefefefefe, 1), // left
];
const TRANSFERS_LEFT: [Transfer; 4] = [
    Transfer(0x7f7f7f7f7f7f7f7f, 1), // right
    Transfer(0x00fefefefefefefe, 7), // down left
    Transfer(0x007f7f7f7f7f7f7f, 9), // down right
    Transfer(0x00ffffffffffffff, 8), // down
];

/// disk must be a singleton
fn flippable_indices_set(my: u64, opp: u64, disk: u64) -> u64 {
    let mut cur = 0;
    for &trans in TRANSFERS_RIGHT.iter() {
        cur |= flippable_indices_in_dir_fixed(|x| (x & trans.0) >> trans.1, my, opp, disk);
    }
    for &trans in TRANSFERS_LEFT.iter() {
        cur |= flippable_indices_in_dir_fixed(|x| (x & trans.0) << trans.1, my, opp, disk)
    }
    cur
}

/// reference: <http://ja.wikipedia.org/wiki/%E3%82%AA%E3%82%BB%E3%83%AD%E3%81%AB%E3%81%8A%E3%81%91%E3%82%8B%E3%83%93%E3%83%83%E3%83%88%E3%83%9C%E3%83%BC%E3%83%89>
#[inline(always)]
fn flippable_indices_in_dir_fixed(
    trans_op: impl Fn(u64) -> u64,
    my: u64,
    opp: u64,
    disk: u64,
) -> u64 {
    let ma = trans_op(disk);
    let mut rev = 0;
    let mut mask = ma;
    while mask != 0 {
        if (mask & opp) == 0 {
            break;
        }
        rev |= mask;
        mask = trans_op(mask);
    }
    if (mask & my) != 0 {
        rev
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_moves_test() {
        let pos = Position::startpos();
        let mvs = pos.valid_moves();
        assert_eq!(mvs.count(), 4);
    }
}
