const BYTE_OFFSET_ZERO: u8 = b'\x30';

pub fn calibration_sum(input: String) -> u32 {
    let lines = input.trim().split("\n");
    let mut sum: u32 = 0;
    for line in lines {
        sum += calibrate_value(line) as u32;
    }
    return sum;
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
fn test_calibration_sum_example() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    let value = calibration_sum(input.to_string());
    assert_eq!(value, 142);
}
