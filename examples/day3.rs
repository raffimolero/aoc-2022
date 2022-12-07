use aoc_2022::input_from_day;

fn priority(c: u8) -> u64 {
    1 + if (b'a'..=b'z').contains(&c) {
        c - b'a'
    } else {
        c - b'A' + 26
    } as u64
}

fn bitflag(c: u8) -> u64 {
    1 << priority(c)
}

#[cfg(not(feature = "b"))]
fn common_item_priority(line: String) -> u64 {
    let (a, b) = line.as_bytes().split_at(line.len() / 2);
    let mut flags = 0_u64;
    for c in a {
        flags |= bitflag(*c);
    }
    for c in b {
        if flags & bitflag(*c) != 0 {
            return priority(*c);
        }
    }
    panic!("There were no items in common!");
}

#[cfg(feature = "b")]
fn item_set(line: String) -> u64 {
    line.bytes().fold(0, |flags, c| flags | bitflag(c))
}

fn main() {
    #[cfg(not(feature = "b"))]
    let total = input_from_day(3).map(common_item_priority).sum::<u64>();

    #[cfg(feature = "b")]
    let total = {
        let mut total = 0;
        let mut input = input_from_day(3);
        while let [Some(a), Some(b), Some(c)] = [input.next(), input.next(), input.next()] {
            let flags = item_set(a) & item_set(b);
            for c in c.bytes() {
                if flags & bitflag(c) != 0 {
                    total += priority(c);
                    break;
                }
            }
        }
        total
    };
    println!("{total}");
}
