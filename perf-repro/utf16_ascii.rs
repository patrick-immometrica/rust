// UTF-16 <-> UTF-8 conversion of ASCII text, std against a hand-written
// "check, then narrow/widen the bytes" version.
//
//     rustc -O utf16_ascii.rs && ./utf16_ascii
//
// String::from_utf16 decodes and pushes one char at a time
// (from_utf16_units in library/alloc/src/string.rs), str::encode_utf16 goes
// through Chars. Neither has a path for input that is all ASCII.

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

fn from_utf16_narrowing(v: &[u16]) -> String {
    if v.iter().all(|&u| u < 0x80) {
        let bytes: Vec<u8> = v.iter().map(|&u| u as u8).collect();
        // ASCII is valid UTF-8
        return unsafe { String::from_utf8_unchecked(bytes) };
    }
    String::from_utf16(v).unwrap()
}

fn encode_utf16_widening(s: &str) -> Vec<u16> {
    if s.is_ascii() {
        return s.bytes().map(u16::from).collect();
    }
    s.encode_utf16().collect()
}

fn main() {
    let text = "lorem ipsum dolor sit amet ".repeat((1 << 20) / 27);
    let units: Vec<u16> = text.encode_utf16().collect();
    assert_eq!(from_utf16_narrowing(&units), text);
    assert_eq!(encode_utf16_widening(&text), units);

    println!("{} ASCII code units", units.len());

    let std_dec = micros(|| String::from_utf16(black_box(&units)).unwrap().len());
    let own_dec = micros(|| from_utf16_narrowing(black_box(&units)).len());
    println!("String::from_utf16        {std_dec:>8.1} us");
    println!("  check + narrow          {own_dec:>8.1} us   ({:.1}x)", std_dec / own_dec);

    let std_enc = micros(|| black_box(text.as_str()).encode_utf16().collect::<Vec<u16>>().len());
    let own_enc = micros(|| encode_utf16_widening(black_box(&text)).len());
    println!("encode_utf16().collect()  {std_enc:>8.1} us");
    println!("  check + widen           {own_enc:>8.1} us   ({:.1}x)", std_enc / own_enc);
}
