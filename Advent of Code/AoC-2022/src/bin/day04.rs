// Day 04: Camp Cleanup

fn ranges_include(ranges: Vec<(u32, u32)>) -> u32 {
    let a = ranges[0];
    let b = ranges[1];

    let b_in_a = a.0 >= b.0 && a.1 <= b.1;
    let a_in_b = a.0 <= b.0 && a.1 >= b.1;

    (b_in_a || a_in_b) as u32
}

fn ranges_overlap(ranges: Vec<(u32, u32)>) -> u32 {
    let a = ranges[0];
    let b = ranges[1];

    ((a.0 <= b.0 && b.0 <= a.1)
        || (a.0 <= b.1 && b.1 <= a.1)
        || (b.0 <= a.0 && a.0 <= b.1)
        || (b.0 <= a.1 && a.1 <= b.1)) as u32
}

fn main() {
    let pairs = include_str!("../inputs/day04.txt").lines();
    let range_group = pairs.map(|pair| -> Vec<(u32, u32)> {
        pair.split(',')
            .map(|elf| -> (u32, u32) {
                let ids: Vec<u32> = elf
                    .split('-')
                    .map(|id| -> u32 { id.parse().unwrap() })
                    .collect();

                (ids[0], ids[1])
            })
            .collect()
    });

    println!(
        "Part 1: {}",
        range_group.clone().map(ranges_include).sum::<u32>()
    );
    println!("Part 2: {}", range_group.map(ranges_overlap).sum::<u32>());
}
