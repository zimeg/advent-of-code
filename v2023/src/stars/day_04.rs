use crate::gifts::parser;
use regex::Regex;

pub fn scratchcard_points(input: String) -> u32 {
    let point_counter = |game: &str| -> u32 {
        let matches = matching_numbers(game);
        if matches == 0 {
            return 0;
        }
        2_u32.pow(matches.saturating_sub(1))
    };
    parser::sum_iteration(input, point_counter)
}

fn matching_numbers(game: &str) -> u32 {
    let regex = Regex::new(r"(?<value>[0-9]+)").unwrap();
    let numbers: Vec<&str> = game.split([':', '|']).collect();
    let splitter = |&nums| {
        let mut values: Vec<u32> = Vec::new();
        let splits = regex.captures_iter(nums);
        for split in splits {
            let value: u32 = split["value"].parse().unwrap();
            values.push(value);
        }
        values
    };
    let winners = numbers.get(1).map(splitter).unwrap();
    let ticket = numbers.get(2).map(splitter).unwrap();
    let mut matches: u32 = 0;
    for value in ticket {
        for wins in winners.clone() {
            if value == wins {
                matches += 1;
            }
        }
    }
    matches
}

#[test]
fn test_scratchcard_points_example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    let value = scratchcard_points(input.to_string());
    assert_eq!(value, 13)
}

#[test]
fn test_scratchcards_part_one() {
    let input = std::fs::read_to_string("./inputs/day04").unwrap();
    let actual = scratchcard_points(input);
    assert_eq!(actual, 19855);
}
