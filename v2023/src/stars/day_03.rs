use crate::gifts::bitmap::BitMap;
use regex::Regex;

pub fn engine_schematic_sum(input: String) -> u32 {
    let lines = input.trim().split('\n');
    let symbols = create_bitmap(lines.clone());
    let mut parts = 0;
    for (row, line) in lines.enumerate() {
        parts += line_parts_sum(&symbols, line, row);
    }
    parts
}

fn create_bitmap(lines: core::str::Split<char>) -> Vec<BitMap> {
    let mut symbols: Vec<BitMap> = Vec::new();
    for line in lines.clone() {
        let mut markings = BitMap::default();
        for symbol in line.chars() {
            if symbol != '.' && !symbol.is_numeric() {
                markings = markings << 1 | 1;
            } else {
                markings = markings << 1;
            }
        }
        symbols.push(markings);
    }
    symbols
}

fn line_parts_sum(symbols: &[BitMap], line: &str, row: usize) -> u32 {
    let regex = Regex::new(r"\d+").unwrap();
    let mut sum: u32 = 0;
    let length = line.len();
    let top = row.saturating_sub(1);
    let bottom = std::cmp::min(row + 2, 3);
    for num in regex.find_iter(line) {
        let mut ands = BitMap::default();
        let left = num.start();
        let right = std::cmp::min(num.end(), length);
        for parts in symbols.iter().skip(top).take(bottom) {
            let padded_left = length - left;
            let padded_right = length.saturating_sub(right).saturating_sub(1);
            ands = parts.range(padded_left, padded_right) | ands;
        }
        if ands != BitMap::default() {
            sum += num.as_str().parse::<u32>().unwrap();
        }
    }
    sum
}

#[test]
fn test_engine_schematic_sum() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let actual = engine_schematic_sum(input.to_string());
    assert_eq!(actual, 4361);
}

#[test]
fn test_gear_ratios_part_one() {
    let input = std::fs::read_to_string("./inputs/day03").unwrap();
    let actual = engine_schematic_sum(input);
    assert_eq!(actual, 553825);
}
