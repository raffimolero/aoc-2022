use std::ops::RangeInclusive as RI;

use aoc_2022::input_from_day;

fn parse_range(range: &str) -> RI<usize> {
    let (s, e) = range.split_once('-').unwrap();
    let s = s.parse().unwrap();
    let e = e.parse().unwrap();
    s..=e
}

fn parse_line(line: String) -> [RI<usize>; 2] {
    let (a, b) = line.split_once(',').unwrap();
    [a, b].map(parse_range)
}

#[cfg(not(feature = "b"))]
fn overlaps([a, b]: &[RI<usize>; 2]) -> bool {
    [RI::start, RI::end]
        .map(|f| f(a).cmp(f(b)) as i8)
        .into_iter()
        .sum::<i8>()
        .abs()
        < 2
}

#[cfg(feature = "b")]
fn overlaps([a, b]: &[RI<usize>; 2]) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

fn main() {
    let redundant_pair_count = input_from_day(4).map(parse_line).filter(overlaps).count();
    println!("{redundant_pair_count}");
}
