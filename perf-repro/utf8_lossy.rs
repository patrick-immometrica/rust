// String::from_utf8_lossy vs str::from_utf8 on input that is already valid.
//
//     rustc -O utf8_lossy.rs && ./utf8_lossy
//
// Both validate the whole input and borrow it. from_utf8 skips ASCII a word
// at a time (library/core/src/str/validations.rs), Utf8Chunks::next looks at
// every byte (library/core/src/str/lossy.rs), so the gap shows on ASCII only.

use std::hint::black_box;
use std::time::Instant;

fn micros<R>(mut f: impl FnMut() -> R) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..5 {
        let t = Instant::now();
        for _ in 0..50 {
            black_box(f());
        }
        best = best.min(t.elapsed().as_secs_f64() / 50.0);
    }
    best * 1e6
}

fn main() {
    const LEN: usize = 1 << 20;
    let inputs = [
        ("ascii", "lorem ipsum dolor sit amet ".repeat(LEN / 27)),
        ("german", "Größe über Straße, schön für Käufer ".repeat(LEN / 42)),
        ("russian", "Привет мир, это проверка строки ".repeat(LEN / 58)),
    ];

    println!("{:<10} {:>14} {:>22} {:>8}", "1 MiB", "from_utf8 (us)", "from_utf8_lossy (us)", "ratio");
    for (name, text) in &inputs {
        let bytes = text.as_bytes();
        let strict = micros(|| std::str::from_utf8(black_box(bytes)).unwrap().len());
        let lossy = micros(|| String::from_utf8_lossy(black_box(bytes)).len());
        println!("{name:<10} {strict:>14.1} {lossy:>22.1} {:>7.1}x", lossy / strict);
    }
}
