// Day 02: Rock Paper Scissors

#[allow(clippy::identity_op)]
fn main() {
    let strategy_guide = include_str!("../inputs/day02.txt");
    let strategies = strategy_guide
        .lines()
        .map(|line| line.split_once(' ').unwrap());

    println!(
        "Part 1: {}",
        strategies
            .clone()
            .map(|game| match game {
                ("A", "X") => 1 + 3,
                ("A", "Y") => 2 + 6,
                ("A", "Z") => 3 + 0,
                ("B", "X") => 1 + 0,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 1 + 6,
                ("C", "Y") => 2 + 0,
                ("C", "Z") => 3 + 3,
                _ => panic!("Invalid strategy"),
            })
            .sum::<i32>()
    );

    println!(
        "Part 2: {}",
        strategies
            .map(|game| match game {
                ("A", "X") => 3 + 0,
                ("A", "Y") => 1 + 3,
                ("A", "Z") => 2 + 6,
                ("B", "X") => 1 + 0,
                ("B", "Y") => 2 + 3,
                ("B", "Z") => 3 + 6,
                ("C", "X") => 2 + 0,
                ("C", "Y") => 3 + 3,
                ("C", "Z") => 1 + 6,
                _ => panic!("Invalid strategy"),
            })
            .sum::<i32>()
    );
}
