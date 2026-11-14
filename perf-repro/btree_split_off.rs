// BTreeMap::split_off gets slower with the size of the map, although the
// split itself only touches one path from the root to a leaf.
//
//     rustc -O btree_split_off.rs && ./btree_split_off
//
// After splitting, calc_split_length (library/alloc/src/collections/btree/
// split.rs) walks one of the two trees to count its entries. It picks the
// lower one, so a split near the end is cheap and a split in the middle,
// where both halves are equally high, visits every node of one half.

use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

fn split_micros(n: u64, at: u64) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..5 {
        let mut map: BTreeMap<u64, u64> = (0..n).map(|i| (i, i)).collect();
        let t = Instant::now();
        let tail = black_box(map.split_off(black_box(&at)));
        best = best.min(t.elapsed().as_secs_f64());
        assert_eq!(tail.len() as u64, n - at);
    }
    best * 1e6
}

fn main() {
    println!("{:>10} {:>16} {:>18}", "entries", "at n/2 (us)", "at n - 10 (us)");
    for n in [10_000, 100_000, 1_000_000, 10_000_000] {
        println!("{n:>10} {:>16.2} {:>18.2}", split_micros(n, n / 2), split_micros(n, n - 10));
    }
}
