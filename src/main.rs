use std::{
    env::{args, current_dir},
    process::Command,
};

fn main() {
    let mut input = args();
    input.next(); // executable path
    let side_day = input.next().expect("cargo run <a|b><day>");
    assert_eq!(input.next(), None, "ay bro too many args");
    let (side, day) = side_day.split_at(1);
    let [feature_tag, half_tag] = match side {
        "a" => ["", ""],
        "b" => ["--features", "b"],
        _ => panic!("a or b, then a number, without spaces"),
    };
    let day = day.parse::<usize>().expect("oi, give me a proper day.");
    let example = format!("day{day}");

    Command::new("cargo")
        .current_dir(current_dir().unwrap())
        .args(["run", "--example", &example, feature_tag, half_tag])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
