use std::collections::HashSet;
use helpers::{print_day, print_solution, read_input};

mod helpers;

pub fn find_matching_numbers(line: &str) -> usize {
     let set = line
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split("|")
        .map(|cards|
            HashSet::from_iter(cards
                .split_whitespace()
                .filter(|&c| c != " ")
                .map(|n|
                    n.parse::<u32>().unwrap()))).collect::<Vec<HashSet<u32>>>();

    return match &set[..2] {
        [left, right] => left.intersection(&right).count(),
        _ => 0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_winnings = input
        .lines()
        .map(|line| {
            return match find_matching_numbers(line) {
                0 => 0,
                x => 2_u32.pow(x as u32 - 1)
            };
        })
        .sum();

    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut winnings = vec![1; lines.clone().count()];
    lines
        .enumerate()
        .for_each(|(index, line)| {
            for i in index + 1..=index + find_matching_numbers(line) {
                winnings[i] += winnings[index]
            }
        });

    let result = winnings
        .iter()
        .sum();

    Some(result)
}

fn main() {
    let input = read_input();
    print_day();
    print_solution(1, part_one, &input);
    print_solution(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example();
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), Some(30));
    }
}
