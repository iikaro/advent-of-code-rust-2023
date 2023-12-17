use std::collections::HashMap;

use lazy_static::lazy_static;

advent_of_code::solution!(2);

lazy_static! {
    static ref VALUE_MAP: HashMap<&'static str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
}

pub fn split_input(input: &str) -> (Option<u32>, Vec<&str>, Vec<&str>) {
    // Extract game number
    let game: Vec<&str> = input.split(':').collect();
    let game_number: Option<u32> = game.get(0).unwrap()
        .trim()
        .strip_prefix("Game ")
        .and_then(|s| s.parse().ok());

    // Extract number of sets
    let set: Vec<&str> = game[1].split(';').collect();

    // Analyze each draw during a given set
    let cubes: Vec<&str> = set.iter().flat_map(|s| s.split(',')).collect();

    (game_number, set, cubes)
}

fn check_if_game_is_possible(game_number: Option<u32>, cubes: Vec<&str>) -> u32 {
    for string in cubes.iter() {
        // Split the string into number and color
        let parts: Vec<&str> = string.split_whitespace().collect();

        // Extract the number
        let number: u32 = parts.first().and_then(|s| s.parse().ok()).unwrap();

        // Extract the color
        let color = parts.last().copied().unwrap().to_lowercase();

        match VALUE_MAP.get(color.as_str()) {
            Some(&map_number) => {
                if map_number < number {
                    println!("In game {:?} the number of dices does not check! There are {map_number} {color} and {number} were draw", game_number.unwrap());
                    return 0_u32;
                }
            }
            None => {
                println!("Color not found in the map.");
            }
        }
    }

    game_number.unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();

    for line in &lines {
        let split_inputs = split_input(line);
        answer += check_if_game_is_possible(split_inputs.0, split_inputs.2);
    }
    split_input(input);
    Option::from(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();

    for line in &lines {
        let split_inputs = split_input(line);
        answer += check_minimum_number_of_dice(split_inputs.2);
    }
    split_input(input);
    Option::from(answer)
}

fn check_minimum_number_of_dice(cubes: Vec<&str>) -> u32 {
    let mut red_cubes_min: u32 = 0;
    let mut green_cubes_min: u32 = 0;
    let mut blue_cubes_min: u32 = 0;

    for string in cubes.iter() {
        // Split the string into number and color
        let parts: Vec<&str> = string.split_whitespace().collect();

        // Extract the number
        let number: u32 = parts.first().and_then(|s| s.parse().ok()).unwrap();

        // Extract the color
        let color = parts.last().copied().unwrap().to_lowercase();

        match color.to_lowercase().as_str() {
            "red" => {
                if number > red_cubes_min {
                    red_cubes_min = number
                }
            }
            "green" => {
                if number > green_cubes_min {
                    green_cubes_min = number
                }
            }
            "blue" => {
                if number > blue_cubes_min {
                    blue_cubes_min = number
                }
            }
            _ => {
                println!("The color is not red, green, or blue.");
            }
        }
    }
    println!("Minimum number of dice is: {red_cubes_min} red, {green_cubes_min} green, {blue_cubes_min} blue.");

    red_cubes_min * green_cubes_min * blue_cubes_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result.unwrap(), 2406);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result.unwrap(), 78375);
    }
}
