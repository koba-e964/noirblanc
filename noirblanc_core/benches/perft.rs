use criterion::{criterion_group, criterion_main, Criterion};
use noirblanc_core::{perft, Position};

// Table is retrieved from https://aartbik.blogspot.com/2009/02/perft-for-reversi.html.
const TABLE_ALL: [u64; 13] = [
    1, 4, 12, 56, 244, 1396, 8200, 55092, 390216, 3005288, 24571284, 212258800, 1939886636,
];

fn bench_perft(c: &mut Criterion) {
    let depth = 8;
    let expected = TABLE_ALL[depth];
    c.bench_function(&format!("perft depth {}", depth), |b| {
        b.iter(|| {
            let pos = Position::startpos();
            let result = perft::perft(pos, depth, false);
            assert_eq!(result.all, expected);
        });
    });
}

criterion_group!(benches, bench_perft);
criterion_main!(benches);
