use aoc_2022::input_from_day;

fn score(line: &str) -> (usize, usize) {
    let opponent = match &line[0..1] {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!(),
    };
    let player = match &line[2..3] {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!(),
    };
    #[cfg(feature = "b")]
    let player = (player + opponent + 2) % 3;

    let throw_score = player + 1;
    let match_score = (4 + player - opponent) % 3 * 3;
    (throw_score, match_score)
}

// it's funny because i could've just written an explicit match for these
// instead of writing a bunch of tests that do the exact same thing for me like a big doofus
#[test]
fn test_scoring() {
    // comments are match scores for first half
    assert_eq!(score("A X"), (3, 0)); // (1, 3)
    assert_eq!(score("A Y"), (1, 3)); // (2, 6)
    assert_eq!(score("A Z"), (2, 6)); // (3, 0)
    assert_eq!(score("B X"), (1, 0)); // (1, 0)
    assert_eq!(score("B Y"), (2, 3)); // (2, 3)
    assert_eq!(score("B Z"), (3, 6)); // (3, 6)
    assert_eq!(score("C X"), (2, 0)); // (1, 6)
    assert_eq!(score("C Y"), (3, 3)); // (2, 0)
    assert_eq!(score("C Z"), (1, 6)); // (3, 3)
}

fn main() {
    let mut total_score = 0;
    for line in input_from_day(2) {
        let (throw_score, match_score) = score(&line);
        total_score += throw_score + match_score;
    }
    println!("{total_score}");
}
