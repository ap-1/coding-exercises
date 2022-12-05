// Day 01: Calorie Counting

fn main() {
    let calorie_list = include_str!("../inputs/day01.txt");
    let mut calorie_counts: Vec<u32> = calorie_list
        .split("\n\n")
        .map(|elves: &str| -> u32 {
            elves
                .lines()
                .map(|calories: &str| -> u32 { calories.parse().unwrap() })
                .sum()
        })
        .collect();

    calorie_counts.sort();

    println!("Part 1: {}", calorie_counts.last().unwrap());
    println!(
        "Part 2: {}",
        calorie_counts.iter().rev().take(3).sum::<u32>()
    );
}
