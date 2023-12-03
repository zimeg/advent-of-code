use regex::{CaptureMatches, Regex};

const MAX_CUBES_RED: u32 = 12;
const MAX_CUBES_GREEN: u32 = 13;
const MAX_CUBES_BLUE: u32 = 14;

pub fn count_possible_games(input: String) -> u32 {
    super::gifts::parser::sum_iteration(input, inspect_game)
}

fn inspect_game(game: &str) -> u32 {
    let cubes = Regex::new(r"(?<count>[0-9]+) (?<color>red|green|blue)").unwrap();
    let rounds: Vec<&str> = game.split(&[':', ';']).collect();
    let id: u32 = rounds[0].strip_prefix("Game ").unwrap().parse().unwrap();
    for round in &rounds[1..] {
        let cubes = cubes.captures_iter(round);
        if !valid_cubes(cubes) {
            return 0;
        }
    }
    id
}

fn valid_cubes(cubes: CaptureMatches) -> bool {
    for cube in cubes {
        let count: u32 = cube["count"].parse().unwrap();
        let valid = match &cube["color"] {
            "red" => count <= MAX_CUBES_RED,
            "green" => count <= MAX_CUBES_GREEN,
            "blue" => count <= MAX_CUBES_BLUE,
            _ => false,
        };
        if !valid {
            return false;
        }
    }
    true
}

#[test]
fn test_count_possible_games() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let value = count_possible_games(input.to_string());
    assert_eq!(value, 8)
}

#[test]
fn test_cube_conundrum_part_one() {
    let input = std::fs::read_to_string("./inputs/day02").unwrap();
    let actual = count_possible_games(input);
    assert_eq!(actual, 1734);
}
