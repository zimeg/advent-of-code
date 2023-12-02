use regex::Regex;

const BYTE_OFFSET_ZERO: u8 = b'\x30';

pub fn calibration_sum(input: String) -> u32 {
    let numerics = replace_spellings(input).unwrap();
    let lines = numerics.trim().split("\n");
    let mut sum: u32 = 0;
    for line in lines {
        sum += calibrate_value(line) as u32;
    }
    return sum;
}

fn replace_spellings(mut input: String) -> Result<String, regex::Error> {
    input = replace(&input, r"one", "1")?;
    input = replace(&input, r"two", "2")?;
    input = replace(&input, r"three", "3")?;
    input = replace(&input, r"four", "4")?;
    input = replace(&input, r"five", "5")?;
    input = replace(&input, r"six", "6")?;
    input = replace(&input, r"seven", "7")?;
    input = replace(&input, r"eight", "8")?;
    input = replace(&input, r"nine", "9")?;
    Ok(input.to_string())
}

fn replace(input: &str, target: &str, value: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(input, value).to_string())
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
