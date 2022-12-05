use aoc_2022::input_from_day;
use arrayvec::ArrayVec;

fn main() {
    const TOP_N: usize = if cfg!(feature = "b") { 3 } else { 1 };
    let mut lines = input_from_day(1);

    let mut top = ArrayVec::<u64, TOP_N>::new();
    let mut current: u64 = 0;
    while top.len() < TOP_N {
        let line = lines.next().expect("not enough input");
        if !line.is_empty() {
            current += line.parse::<u64>().expect("not a number");
            continue;
        }
        top.push(current);
        current = 0;
    }
    for line in lines {
        if !line.is_empty() {
            current += line.parse::<u64>().unwrap();
            continue;
        }
        let idx = top
            .iter()
            .enumerate()
            .min_by_key(|(_idx, &val)| val)
            .expect("there should be more than 1 element lmao")
            .0;
        top[idx] = current.max(top[idx]);
        current = 0;
    }

    let combined = top.into_iter().sum::<u64>();
    println!("{combined}");
}
