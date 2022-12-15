const _INPUT: &str = include_str!("input");
const _EXAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

fn has_dups(slice: &[char]) -> bool {
    for c in slice {
        if slice.iter().filter(|x| *x == c).count() > 1 {
            return true;
        }
    }
    false
}

fn find_marker(input: &str, win_size: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    chars
        .windows(win_size)
        .enumerate()
        .find_map(|(pos, win)| {
            if !has_dups(win) {
                Some(pos + win_size)
            } else {
                None
            }
        })
        .unwrap()
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    println!("Part 1: {}", part1(_INPUT));
    println!("Part 2: {}", part2(_INPUT));
}
