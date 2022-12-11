const _INPUT: &str = include_str!("input");
const _EXAMPLE: &str = include_str!("example");

fn _part1() -> u64 {
    _INPUT
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|food| food.parse::<u64>().ok())
                .sum()
        })
        .max()
        .unwrap_or(0)
}

fn _part2() -> u64 {
    let mut sums = _INPUT
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|food| food.parse::<u64>().ok())
                .sum()
        })
        .collect::<Vec<u64>>();
    sums.sort_unstable();
    sums.iter().rev().take(3).sum()
}

fn main() {
    println!("{}", _part1());
    println!("{}", _part2());
}
