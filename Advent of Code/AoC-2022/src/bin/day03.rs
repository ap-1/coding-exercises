// Day 03: Rucksack Reorganization

use std::collections::HashSet;

fn letter_to_i32(c: &char) -> i32 {
    if c.is_lowercase() {
        (*c as u8 - b'a' + 1) as i32
    } else {
        (*c as u8 - b'A' + 27) as i32
    }
}

fn main() {
    let rucksacks = include_str!("../inputs/day03.txt").lines();

    println!(
        "Part 1: {}",
        rucksacks
            .clone()
            .map(|rucksack| {
                let (left, right) = rucksack.split_at(rucksack.len() / 2);

                let compartment_a: HashSet<char> = HashSet::from_iter(left.chars());
                let compartment_b: HashSet<char> = HashSet::from_iter(right.chars());

                let shared = compartment_a.intersection(&compartment_b).last().unwrap();
                letter_to_i32(shared)
            })
            .sum::<i32>()
    );

    println!(
        "Part 2: {}",
        rucksacks
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|group| {
                let elf_a: HashSet<char> = HashSet::from_iter(group[0].chars());
                let elf_b: HashSet<char> = HashSet::from_iter(group[1].chars());
                let elf_c: HashSet<char> = HashSet::from_iter(group[2].chars());

                let mut shared = elf_a
                    .intersection(&elf_b)
                    .cloned()
                    .collect::<HashSet<char>>();
                shared.retain(|c| elf_c.contains(c));

                letter_to_i32(shared.iter().last().unwrap())
            })
            .sum::<i32>()
    );
}
