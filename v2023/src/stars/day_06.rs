use crate::gifts::parser;

pub fn boat_distance_errors(input: String) -> u32 {
    let mut lines = input.lines();
    let times = parser::gather_numbers(lines.next().unwrap());
    let distances = parser::gather_numbers(lines.next().unwrap());
    boat_distance_race(times, distances)
}

pub fn boat_distance_single(input: String) -> u32 {
    let mut lines = input.lines();
    let times = parser::gather_numbers(lines.next().unwrap().replace(' ', "").as_str());
    let distances = parser::gather_numbers(lines.next().unwrap().replace(' ', "").as_str());
    boat_distance_race(times, distances)
}

fn boat_distance_race(times: Vec<u64>, distances: Vec<u64>) -> u32 {
    let mut margin = 1;
    for (race, time) in times.into_iter().enumerate() {
        let mut wins = 0;
        for durr in 0..time {
            if (time - durr) * durr > distances[race] {
                wins += 1;
            }
        }
        margin *= wins;
    }
    margin
}

#[test]
fn test_boat_distance_errors_example() {
    let input = "\
Time:      7  15   30
Distance:  9  40  200";
    let value = boat_distance_errors(input.to_string());
    assert_eq!(value, 288);
}

#[test]
fn test_boat_distance_single_example() {
    let input = "\
Time:      7  15   30
Distance:  9  40  200";
    let value = boat_distance_single(input.to_string());
    assert_eq!(value, 71503);
}

#[test]
fn test_boat_distance_errors_part_one() {
    let input = std::fs::read_to_string("./inputs/day06").unwrap();
    let actual = boat_distance_errors(input);
    assert_eq!(actual, 393120);
}

#[test]
fn test_boat_distance_errors_part_two() {
    let input = std::fs::read_to_string("./inputs/day06").unwrap();
    let actual = boat_distance_single(input);
    assert_eq!(actual, 36872656);
}
