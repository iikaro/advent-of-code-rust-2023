extern crate regex;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(1);

lazy_static! {
    static ref VALUE_MAP: HashMap<&'static str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    static ref FIRST_MATCH_PATTERN: Regex =
        Regex::new(r"^.*?(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)")
            .unwrap();
    static ref LAST_MATCH_PATTERN: Regex =
        Regex::new(r".*(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9).*?$")
            .unwrap();
}
fn find_first_and_last_numbers(s: &str) -> u32 {
    let digits: Vec<u32> = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    10_u32 * digits.first().unwrap() + digits.last().unwrap()
}

fn find_first_and_last_number_names(s: &str) -> u32 {
    let first_match = FIRST_MATCH_PATTERN
        .captures(s)
        .and_then(|m| m.get(1).and_then(|s| VALUE_MAP.get(s.as_str())));

    let last_match = LAST_MATCH_PATTERN
        .captures(s)
        .and_then(|m| m.get(1).and_then(|s| VALUE_MAP.get(s.as_str())));
    10_u32 * first_match.unwrap() + last_match.unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();

    for line in &lines {
        answer += find_first_and_last_numbers(line);
    }

    Option::from(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();

    for line in &lines {
        answer += find_first_and_last_number_names(line);
    }

    Option::from(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result.unwrap(), 55607 as u32);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result.unwrap(), 55291 as u32);
    }
}
