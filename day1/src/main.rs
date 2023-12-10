use helpers::{print_day, print_solution, read_input};

mod helpers;

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .split('\n')
        .map(|line| {
            let mut front_num: Option<u32> = None;
            let mut back_num: Option<u32> = None;

            for i in 0..line.chars().count() {
                if front_num.is_some() && back_num.is_some() {
                    break;
                }

                if front_num.is_none() {
                    let front_char = line.chars().nth(i).unwrap();
                    front_num = c_to_num(front_char);
                }

                if back_num.is_none() {
                    let back_char = line.chars().nth(line.chars().count() - 1 - i).unwrap();
                    back_num = c_to_num(back_char);
                }
            }

            front_num.unwrap() * 10 + back_num.unwrap()
        })
        .sum::<u32>();

    Some(total)
}

fn c_to_num(c: char) -> Option<u32> {
    c.to_digit(10)
}

fn s_to_number(s: &str) -> Option<u32> {
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
    let total = input
        .split('\n')
        .map(|line| {
            let mut front_num: Option<u32> = None;
            let mut back_num: Option<u32> = None;

            for i in 0..line.chars().count() {
                if front_num.is_some() && back_num.is_some() {
                    break;
                }

                if front_num.is_none() {
                    let front_char = line.chars().nth(i).unwrap();
                    front_num = c_to_num(front_char);
                    if front_num.is_none() {
                        let front_str = &line[0..i];
                        front_num = s_to_number(front_str);
                    }
                }

                if back_num.is_none() {
                    let back_char = line.chars().nth(line.chars().count() - 1 - i).unwrap();
                    back_num = c_to_num(back_char);
                    if back_num.is_none() {
                        let back_str = &line[line.chars().count() - 1 - i..line.chars().count()];
                        back_num = s_to_number(back_str);
                    }
                }
            }

            front_num.unwrap() * 10 + back_num.unwrap()
        })
        .sum::<u32>();

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
    use crate::helpers::{read_example, read_example_2};

    #[test]
    fn test_part_one() {
        let input = read_example();
        assert_eq!(part_one(&input), Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = read_example_2();
        assert_eq!(part_two(&input), Some(281));
    }
}
