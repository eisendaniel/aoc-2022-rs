const _INPUT: &str = include_str!("input");
const _EXAMPLE: &str = "A Y\nB X\nC Z";

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
use Outcome::{Draw, Loss, Win};
impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Invalid input string >:("),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper,
    Scissors,
}
use Hand::{Paper, Rock, Scissors};

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Invalid input string >:("),
        }
    }
}

impl Hand {
    fn vs(&self, other: &Hand) -> u64 {
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0, //loss
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3, //draw
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6, //win
        }
    }
    fn weakness(&self) -> Self {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
    fn strength(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|round| {
            let mut hands = round.split_whitespace();
            let their_hand: Hand = hands.next().unwrap().into();
            let our_hand: Hand = hands.next().unwrap().into();

            our_hand as u64 + our_hand.vs(&their_hand)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|round| {
            let mut codes = round.split_whitespace();
            let their_hand: Hand = codes.next().unwrap().into();
            let result: Outcome = codes.next().unwrap().into();

            let our_hand = match result {
                Loss => their_hand.strength(),
                Draw => their_hand,
                Win => their_hand.weakness(),
            };
            (result as u64) + (our_hand as u64)
        })
        .sum()
}
fn main() {
    println!("Part1: {}", part1(_INPUT));
    println!("Part2: {}", part2(_INPUT));
}
