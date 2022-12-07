use aoc_2022::input_from_day;
use arraydeque::{ArrayDeque, Wrapping};

fn main() {
    const LEN: usize = if cfg!(feature = "b") { 14 } else { 4 };
    let line = input_from_day(6).next().unwrap();
    let (start, bytes) = line.as_bytes().split_at(LEN);
    let mut buf = ArrayDeque::<[u8; LEN], Wrapping>::from_iter(start.iter().copied());
    for (i, byte) in bytes.iter().copied().enumerate() {
        if buf
            .iter()
            .enumerate()
            .skip(1)
            .all(|(i, a)| buf.iter().take(i).all(|b| a != b))
        {
            println!("{}", i + LEN);
            return;
        }
        buf.push_back(byte);
    }
}
