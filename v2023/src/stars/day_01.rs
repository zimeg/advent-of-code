use regex::{Captures, Regex};

const BYTE_OFFSET_ZERO: u8 = b'\x30';

pub fn calibration_sum(input: String) -> u32 {
    let numerics = replace_spellings(&input).unwrap();
    let lines = numerics.trim().split("\n");
    let mut sum: u32 = 0;
    for line in lines {
        sum += calibrate_value(line) as u32;
    }
    return sum;
}

fn replace_spellings(input: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)")?;
    let replacement = |caps: &Captures| -> String {
        return match caps[0].trim() {
            "one" => "1".to_string(),
            "two" => "2".to_string(),
            "three" => "3".to_string(),
            "four" => "4".to_string(),
            "five" => "5".to_string(),
            "six" => "6".to_string(),
            "seven" => "7".to_string(),
            "eight" => "8".to_string(),
            "nine" => "9".to_string(),
            x => x.to_string(),
        };
    };
    Ok(regex.replace_all(input, &replacement).to_string())
}

fn calibrate_value(value: &str) -> u8 {
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
    return first * 10 + last;
}

#[test]
fn test_calibration_sum_numbers() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let value = calibration_sum(input.to_string());
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
    let value = calibration_sum(input.to_string());
    assert_eq!(value, 281);
}
