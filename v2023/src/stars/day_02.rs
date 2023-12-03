use regex::{CaptureMatches, Regex};

struct GameCubes {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

pub fn count_possible_games(input: String) -> u32 {
    super::gifts::parser::sum_iteration(input, check_game_possibility)
}

pub fn combine_fewest_cubes(input: String) -> u32 {
    super::gifts::parser::sum_iteration(input, power_of_min_cubeset)
}

fn inspect_game(game: &str, comp: impl Fn(GameCubes) -> u32) -> u32 {
    let regex = Regex::new(r"(?<count>[0-9]+) (?<color>red|green|blue)").unwrap();
    let rounds: Vec<&str> = game.split(&[':', ';']).collect();
    let id: u32 = rounds[0].strip_prefix("Game ").unwrap().parse().unwrap();
    let mut counts: GameCubes = GameCubes {
        id,
        red: 0,
        green: 0,
        blue: 0,
    };
    for round in &rounds[1..] {
        let cubes = regex.captures_iter(round);
        counts = max_cubes_per_game(cubes, counts);
    }
    comp(counts)
}

fn max_cubes_per_game(cubes: CaptureMatches, mut counts: GameCubes) -> GameCubes {
    for cube in cubes {
        let count: u32 = cube["count"].parse().unwrap();
        match &cube["color"] {
            "red" => counts.red = std::cmp::max(counts.red, count),
            "green" => counts.green = std::cmp::max(counts.green, count),
            "blue" => counts.blue = std::cmp::max(counts.blue, count),
            _ => (),
        };
    }
    counts
}

fn check_game_possibility(game: &str) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let score_computation = |counts: GameCubes| -> u32 {
        if counts.red > MAX_RED || counts.blue > MAX_BLUE || counts.green > MAX_GREEN {
            0
        } else {
            counts.id
        }
    };
    inspect_game(game, score_computation)
}

fn power_of_min_cubeset(game: &str) -> u32 {
    let score_computation = |counts: GameCubes| -> u32 { counts.red * counts.blue * counts.green };
    inspect_game(game, score_computation)
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
fn test_combine_fewest_cubes() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let value = combine_fewest_cubes(input.to_string());
    assert_eq!(value, 2286)
}

#[test]
fn test_cube_conundrum_part_one() {
    let input = std::fs::read_to_string("./inputs/day02").unwrap();
    let actual = count_possible_games(input);
    assert_eq!(actual, 1734);
}

#[test]
fn test_cube_conundrum_part_two() {
    let input = std::fs::read_to_string("./inputs/day02").unwrap();
    let actual = combine_fewest_cubes(input);
    assert_eq!(actual, 70387);
}
