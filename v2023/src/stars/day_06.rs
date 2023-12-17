use crate::gifts::parser;

pub fn boat_distance_errors(input: String) -> u32 {
    let mut lines = input.lines();
    let times = parser::gather_numbers(lines.next().unwrap());
    let distances = parser::gather_numbers(lines.next().unwrap());
    let mut errors = 1;
    for (race, time) in times.into_iter().enumerate() {
        let mut wins = 0;
        for durr in 0..time {
            if (time - durr) * durr > distances[race] {
                wins += 1;
            }
        }
        errors *= wins;
    }
    errors
}

#[test]
fn test_boat_distance_errors_example() {
    let input = "\
Time:      7  15   30
Distance:  9  40  200";
    let value = boat_distance_errors(input.to_string());
    assert_eq!(value, 288);
}
