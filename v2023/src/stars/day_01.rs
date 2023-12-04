use regex::{Captures, Regex};

const BYTE_OFFSET_ZERO: u8 = b'\x30';

pub fn calibration_letters(input: String) -> u32 {
    let numerics = replace_spellings(&input).unwrap();
    calibration_numbers(numerics)
}

pub fn calibration_numbers(input: String) -> u32 {
    super::gifts::parser::sum_iteration(input, calibrate_value)
}

fn replace_spellings(input: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)")?;
    let replacement = |caps: &Captures| -> String {
        return match caps[0].trim() {
            "one" => "o1ne".to_string(),
            "two" => "t2wo".to_string(),
            "three" => "t3hree".to_string(),
            "four" => "f4our".to_string(),
            "five" => "f5ive".to_string(),
            "six" => "s6ix".to_string(),
            "seven" => "s7even".to_string(),
            "eight" => "e8ight".to_string(),
            "nine" => "n9ine".to_string(),
            x => x.to_string(),
        };
    };
    let overlapped = regex.replace_all(input, &replacement);
    return Ok(regex.replace_all(&overlapped, &replacement).to_string());
}

fn calibrate_value(value: &str) -> u32 {
    let mut first = 0;
    let mut last = 0;
    let mut found = false;
    for byte in value.bytes() {
        let digit = byte - BYTE_OFFSET_ZERO;
        if digit <= 9 {
            if !found {
                first = digit;
                found = true;
            }
            last = digit;
        }
    }
    (first * 10 + last).into()
}

#[test]
fn test_calibration_sum_numbers() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let value = calibration_numbers(input.to_string());
    assert_eq!(value, 142);
}

#[test]
fn test_calibration_sum_letters() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let value = calibration_letters(input.to_string());
    assert_eq!(value, 281);
}

#[test]
fn test_calibration_part_one() {
    let input = std::fs::read_to_string("./inputs/day01").unwrap();
    let actual = calibration_numbers(input);
    assert_eq!(actual, 54877);
}

#[test]
fn test_calibration_part_two() {
    let input = std::fs::read_to_string("./inputs/day01").unwrap();
    let actual = calibration_letters(input);
    assert_eq!(actual, 54100);
}
