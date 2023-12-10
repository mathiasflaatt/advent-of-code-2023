use std::time::Instant;

use helpers::{print_day, print_solution, read_input};

mod helpers;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let total = lines
        .map(|line| {
            let mut sum = 0;
            for c in line.chars() {
                let res = c.to_digit(10);
                if res.is_some() {
                    let num = res.unwrap();
                    sum = num * 10;
                    break;
                }
            }

            for c in line.chars().rev() {
                let res = c.to_digit(10);
                if res.is_some() {
                    let num = res.unwrap();
                    sum = sum + num;
                    break;
                }
            }
            sum
        })
        .fold(0, |a, b| a + b);

    Some(total)
}

fn to_number(s: &str) -> Option<u32> {
    if s.contains("one") {
        return Some(1);
    }
    if s.contains("two") {
        return Some(2);
    }
    if s.contains("three") {
        return Some(3);
    }
    if s.contains("four") {
        return Some(4);
    }
    if s.contains("five") {
        return Some(5);
    }
    if s.contains("six") {
        return Some(6);
    }
    if s.contains("seven") {
        return Some(7);
    }
    if s.contains("eight") {
        return Some(8);
    }
    if s.contains("nine") {
        return Some(9);
    }
    if s.contains("zero") {
        return Some(0);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let total = lines
        .map(|line| {
            let mut sum = 0;
            let mut acc = "".to_string();

            for c in line.chars() {
                let res = c.to_digit(10);
                if res.is_some() {
                    let num = res.unwrap();
                    sum = num * 10;
                    break;
                }

                acc.push(c);
                let potential_number = to_number(&acc);
                if potential_number.is_some() {
                    let num = potential_number.unwrap();
                    sum = num * 10;
                    break;
                }
            }

            acc = "".to_string();

            for c in line.chars().rev() {
                let res = c.to_digit(10);
                if res.is_some() {
                    let num = res.unwrap();
                    sum = sum + num;
                    // println!("{}/{}", num, acc);
                    break;
                }

                acc.insert(0, c);
                let potential_number = to_number(&acc);
                if potential_number.is_some() {
                    let num = potential_number.unwrap();
                    sum = sum + num;
                    // println!("{}/{}", num, acc);
                    break;
                }
            }
            sum
        })
        .fold(0, |a, b| a + b);

    Some(total)
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
    #[ignore]
    fn test_part_one() {
        let input = read_example();
        assert_eq!(part_one(&input), Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), Some(281));
    }
}
