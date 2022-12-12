use std::ops::RangeInclusive;

const _INPUT: &str = include_str!("input");
const _EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn overlaps(a: RangeInclusive<u8>, b: RangeInclusive<u8>) -> bool {
    
    true
}

fn part1(input: &str) -> usize {
    input.lines().map(|pair| {
        let (a, b) = pair.split_once(',').unwrap();
    });
    0
}

fn main() {
    println!("Part 1: {}", part1(_EXAMPLE));
    // println!("Part 1: {}", part2(_INPUT));
}
