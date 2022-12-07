use std::collections::HashMap;

use aoc_2022::input_from_day;

#[derive(Default, Debug)]
struct Dir(HashMap<String, Item>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlFlow {
    Continue,
    Root,
    End,
}

impl Dir {
    fn write_properties(&mut self, sizes: &mut Vec<usize>) {
        let mut dir_size = 0;
        for (_name, item) in self.0.iter_mut() {
            match item {
                Item::Dir(dir) => {
                    dir.write_properties(sizes);
                    dir_size += sizes.last().unwrap();
                }
                Item::File(file_size) => dir_size += *file_size,
            }
        }
        sizes.push(dir_size);
    }

    /// returns the sizes of all subdirectories within
    pub fn properties(&mut self) -> Vec<usize> {
        let mut sizes = vec![];
        self.write_properties(&mut sizes);
        sizes
    }

    /// returns whether to return straight to root
    pub fn os_traverse(&mut self, input: &mut impl Iterator<Item = String>) -> ControlFlow {
        while let Some(line) = input.next() {
            match line.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
                ["$", "cd", "/"] => return ControlFlow::Root,
                ["$", "cd", ".."] => return ControlFlow::Continue,
                &["$", "cd", dir] => {
                    let Some(Item::Dir(dir)) = self.0.get_mut(dir) else {
                        panic!("{dir} is either not a directory or does not exist.");
                    };
                    match dir.os_traverse(input) {
                        ControlFlow::Continue => {}
                        residual => return residual,
                    }
                }
                ["$", "ls"] => {}
                &["dir", name] => {
                    self.0.insert(name.to_owned(), Item::Dir(Dir::default()));
                }
                &[size, name] => {
                    self.0.insert(
                        name.to_owned(),
                        Item::File(size.parse().expect("Invalid file size.")),
                    );
                }
                line => panic!("Unrecognized line: {line:?}"),
            }
        }
        ControlFlow::End
    }
}

#[derive(Debug)]
enum Item {
    Dir(Dir),
    File(usize),
}

impl Default for Item {
    fn default() -> Self {
        Self::Dir(Dir::default())
    }
}

#[cfg(not(feature = "b"))]
/// total size of all directories smaller than the cutoff
fn solve(properties: &[usize]) -> usize {
    const CUTOFF: usize = 100000;
    properties
        .iter()
        .filter(|size| **size <= CUTOFF)
        .sum::<usize>()
}

#[cfg(feature = "b")]
/// size of smallest directory bigger than the required amount
fn solve(properties: &[usize]) -> usize {
    const REQUIRED_SPACE: usize = 30000000;
    const DISK_SPACE: usize = 70000000;
    let amount_to_free = properties.last().unwrap() + REQUIRED_SPACE - DISK_SPACE;
    *properties
        .iter()
        .filter(|size| **size >= amount_to_free)
        .min()
        .unwrap()
}

fn main() {
    let mut root = Dir::default();
    let mut input = input_from_day(7);
    while root.os_traverse(&mut input) != ControlFlow::End {}
    println!("{root:#?}");

    let properties = root.properties();
    println!("{}", solve(&properties));
}
