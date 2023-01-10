use crate::{Bitboard, Position};

pub struct Stat {
    pub all: u64,
}

pub fn perft(mut pos: Position, depth: usize, skipped: bool) -> Stat {
    if depth == 0 {
        return Stat { all: 1 };
    }
    let all = pos.valid_moves();
    if all.is_empty() {
        if skipped {
            return Stat { all: 1 };
        }
        pos.pass();
        return perft(pos, depth - 1, true);
    }
    if depth == 1 {
        return Stat {
            all: all.count() as u64,
        };
    }
    let mut answer_all = 0;
    for mv in all {
        let mut next = pos.clone();
        next.make_move(Bitboard::singleton(mv));
        let sub = perft(next, depth - 1, false);
        answer_all += sub.all;
    }
    Stat { all: answer_all }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Table is retrieved from https://aartbik.blogspot.com/2009/02/perft-for-reversi.html.
    const TABLE_ALL: [u64; 13] = [
        1, 4, 12, 56, 244, 1396, 8200, 55092, 390216, 3005288, 24571284, 212258800, 1939886636,
    ];

    #[test]
    fn perft_result_matches() {
        for (depth, &expected) in TABLE_ALL[..8].iter().enumerate() {
            let pos = Position::startpos();
            let result = perft(pos, depth, false);
            assert_eq!(result.all, expected);
        }
    }

    #[cfg(bench)]
    #[bench]
    fn bench_perft_8(b: &mut test::Bencher) {
        let depth = 8;
        bench_perft(b, depth);
    }

    #[cfg(bench)]
    #[inline(always)]
    fn bench_perft(b: &mut test::Bencher, depth: usize) {
        let expected = TABLE_ALL[depth];
        b.iter(|| {
            let pos = Position::startpos();
            let result = perft(pos, depth, false);
            assert_eq!(result.all, expected);
        });
    }
}
