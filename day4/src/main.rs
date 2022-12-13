use std::ops::RangeInclusive;

const _INPUT: &str = include_str!("input");
const _EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn redundant_range(a: &RangeInclusive<u8>, b: &RangeInclusive<u8>) -> bool {
    a.start() <= b.start() && a.end() >= b.end() || b.start() <= a.start() && b.end() >= a.end()
}

fn overlaps(a: &RangeInclusive<u8>, b: &RangeInclusive<u8>) -> bool {
    a.start().max(b.start()) <= a.end().min(b.end())
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|pair| {
            //"2-4,6-8"
            let (a, b) = pair.split_once(',').unwrap(); //a:"2-4" b:"6-8"
            let (a_s, a_e) = a.split_once('-').unwrap();
            let (b_s, b_e) = b.split_once('-').unwrap();
            (
                a_s.parse().unwrap()..=a_e.parse().unwrap(),
                b_s.parse().unwrap()..=b_e.parse().unwrap(),
            )
        })
        .filter(|(a, b)| redundant_range(a, b))
        .count()
}
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|pair| {
            //"2-4,6-8"
            let (a, b) = pair.split_once(',').unwrap(); //a:"2-4" b:"6-8"
            let (a_s, a_e) = a.split_once('-').unwrap();
            let (b_s, b_e) = b.split_once('-').unwrap();
            (
                a_s.parse().unwrap()..=a_e.parse().unwrap(),
                b_s.parse().unwrap()..=b_e.parse().unwrap(),
            )
        })
        .filter(|(a, b)| overlaps(a, b))
        .count()
}

fn main() {
    println!("Part 1: {}", part1(_INPUT));
    println!("Part 1: {}", part2(_INPUT));
}
