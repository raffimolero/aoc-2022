use std::iter::Peekable;

use aoc_2022::input_from_day;
use arrayvec::ArrayVec;

fn print_stacks(stacks: &Vec<Vec<u8>>) {
    for stack in stacks {
        for item in stack {
            print!("{:?} ", *item as char);
        }
        println!();
    }
}

fn read_stacks<I: Iterator<Item = String>>(mut input: &mut Peekable<I>) -> Vec<Vec<u8>> {
    let mut stacks = vec![vec![]; (input.peek().unwrap().len() + 1) / 4];
    for line in &mut input {
        // challenge: do not modify the input
        if line.as_bytes()[1] == b'1' {
            break;
        }
        for (c, stack) in line
            .bytes() // ascii only
            .skip(1)
            .step_by(4)
            .zip(&mut stacks)
            .filter(|(c, _)| *c != b' ')
        {
            stack.push(c);
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    input.next(); // empty space
    stacks
}

fn solve(input: impl Iterator<Item = String>, stacks: &mut [Vec<u8>]) {
    for line in input {
        let [n, f, t] = *line
            .split_ascii_whitespace()
            .flat_map(|w| w.parse::<usize>())
            .collect::<ArrayVec<_, 3>>()
        else {
            panic!()
        };
        #[cfg(feature = "b")]
        let start = stacks[t - 1].len();
        for _ in 0..n {
            let v = stacks[f - 1].pop().unwrap();
            stacks[t - 1].push(v);
        }
        #[cfg(feature = "b")]
        stacks[t - 1][start..].reverse();
    }
}

fn main() {
    let mut input = input_from_day(5).peekable();
    let mut stacks = read_stacks(&mut input);
    // print_stacks(&stacks);
    // println!("solving");
    solve(input, &mut stacks);
    // println!("solved");
    // print_stacks(&stacks);
    for stack in stacks {
        print!("{}", *stack.last().unwrap() as char);
    }
    println!();
}
