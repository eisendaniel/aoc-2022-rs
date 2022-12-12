const _INPUT: &str = include_str!("input");
const _EXAMPLE: &str = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

fn priority(item: u8) -> u8 {
    (item % 32) + (item <= 90) as u8 * 26
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|sack| {
            let compartments = sack.split_at(sack.len() / 2);
            compartments
                .0
                .as_bytes()
                .iter()
                .find(|item| compartments.1.as_bytes().contains(item))
                .map(|&com| priority(com) as usize)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .into_iter()
        .filter_map(|chunks| {
            let a = chunks[0].as_bytes();
            let b = chunks[1].as_bytes();
            let c = chunks[2].as_bytes();
            a.iter()
                .find(|item| b.contains(item) && c.contains(item))
                .map(|&item| priority(item) as usize)
        })
        .sum()
}
fn main() {
    println!("Part 1: {}", part1(_INPUT));
    println!("Part 1: {}", part2(_INPUT));
}
