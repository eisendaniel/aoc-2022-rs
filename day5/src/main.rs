const _INPUT: &str = include_str!("input");
const _EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

fn part1(input: &str) -> String {
    let (stack_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stack_str.lines().rev();
    let n = (stack_iter.next().unwrap().len() + 1) / 4; //process number line and get width
    let mut stack: Vec<Vec<char>> = vec![vec![]; n];

    stack_iter.for_each(|line| {
        line.chars().skip(1).enumerate().for_each(|(i, c)| {
            if !" []".contains(c) {
                stack[i / 4].push(c);
            }
        });
    });

    moves_str.lines().for_each(|line| {
        let mut cmds = line
            .split_whitespace()
            .filter_map(|token| token.parse::<usize>().ok());

        let amount = cmds.next().unwrap();
        let from = cmds.next().unwrap() - 1;
        let to = cmds.next().unwrap() - 1;

        for _ in 0..amount {
            if let Some(item) = stack[from].pop() {
                stack[to].push(item);
            }
        }
    });

    stack
        .into_iter()
        .filter_map(|mut pile| pile.pop())
        .collect()
}

fn part2(input: &str) -> String {
    let (stack_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stack_str.lines().rev();
    let n = (stack_iter.next().unwrap().len() + 1) / 4; //process number line and get width
    let mut stack: Vec<Vec<char>> = vec![vec![]; n];

    stack_iter.for_each(|line| {
        line.chars().skip(1).enumerate().for_each(|(i, c)| {
            if !" []".contains(c) {
                stack[i / 4].push(c);
            }
        });
    });

    moves_str.lines().for_each(|line| {
        let mut cmds = line
            .split_whitespace()
            .filter_map(|token| token.parse::<usize>().ok());

        let amount = cmds.next().unwrap();
        let from = cmds.next().unwrap() - 1;
        let to = cmds.next().unwrap() - 1;

        let crates2mov: Vec<char> = (0..amount).filter_map(|_| stack[from].pop()).collect();
        for c in crates2mov.into_iter().rev() {
            stack[to].push(c);
        }
    });

    stack
        .into_iter()
        .filter_map(|mut pile| pile.pop())
        .collect()
}

fn main() {
    println!("\nPart 1: {}", part1(_INPUT));
    println!("\nPart 2: {}", part2(_INPUT));
}
