use aoc_2022::input_from_day;

fn main() {
    for line in input_from_day(0) {
        // first half; a0
        #[cfg(not(feature = "b"))]
        println!("A {line}");

        // second half; b0
        #[cfg(feature = "b")]
        println!("B {line}");
    }
}
