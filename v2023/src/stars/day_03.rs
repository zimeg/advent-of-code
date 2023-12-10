use crate::gifts::bitmap::BitMap;
use num::Complex;
use regex::{Match, Regex};

pub fn engine_schematic_sum(input: String) -> u32 {
    let lines = input.trim().split('\n');
    let filter = |s: char| s != '.' && !s.is_numeric();
    let matched = |b: BitMap| b << 1 | 1;
    let empty = |b: BitMap| b << 1;
    let symbols = create_bitmap(lines.clone(), filter, matched, empty);
    let mut parts = 0;
    for (row, line) in lines.enumerate() {
        parts += line_parts_sum(&symbols, line, row);
    }
    parts
}

fn line_parts_sum(symbols: &[BitMap], line: &str, row: usize) -> u32 {
    let regex = Regex::new(r"\d+").unwrap();
    let mut sum: u32 = 0;
    for matched in regex.find_iter(line) {
        let mut ands = BitMap::default();
        let bounds = adjacent_bounds(line.len(), row, matched, true);
        for parts in symbols.iter().skip(bounds.top).take(bounds.bottom) {
            ands = parts.range(bounds.left, bounds.right) | ands;
        }
        if ands != BitMap::default() {
            sum += matched.as_str().parse::<u32>().unwrap();
        }
    }
    sum
}

pub fn adjacent_gear_ratios(input: String) -> u32 {
    let lines = input.trim().split('\n');
    let filter = |s: char| s == '*';
    let matched = |existing: Vec<Complex<i32>>| {
        let mut v = existing.to_vec();
        v.push(Complex { re: -1, im: 0 });
        v
    };
    let empty = |existing: Vec<Complex<i32>>| {
        let mut v = existing.to_vec();
        v.push(Complex::default());
        v
    };
    let mut symbols = create_bitmap(lines.clone(), filter, matched, empty);
    for (row, line) in lines.enumerate() {
        symbols = compute_ratios(symbols, line, row);
    }
    let mut parts = 0;
    for ratios in symbols {
        for c in ratios {
            if c.re > 0 && c.im != 0 {
                parts += c.re;
            }
        }
    }
    parts.try_into().unwrap()
}

fn compute_ratios(
    mut gears: Vec<Vec<Complex<i32>>>,
    line: &str,
    row: usize,
) -> Vec<Vec<Complex<i32>>> {
    let regex = Regex::new(r"\d+").unwrap();
    for matched in regex.find_iter(line) {
        let bounds = adjacent_bounds(line.len(), row, matched, false);
        let num = matched.as_str().parse::<i32>().unwrap();
        for shelf in gears.iter_mut().skip(bounds.top).take(bounds.bottom) {
            let mut y: usize = bounds.left;
            for place in shelf.clone().iter().skip(bounds.left).take(bounds.right) {
                if place.re != 0 && place.im != 0 {
                    if place.re / num != place.im - num {
                        shelf[y] = Complex { re: 0, im: 0 };
                    }
                } else if place.re != 0 {
                    shelf[y] = place * Complex { re: 0, im: num };
                } else if place.im != 0 {
                    let lock = Complex {
                        re: 0,
                        im: num - place.im,
                    };
                    shelf[y] = place * Complex { re: 0, im: num } + lock;
                }
                y += 1;
            }
        }
    }
    gears
}

struct Boundaries {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

fn adjacent_bounds(length: usize, row: usize, matched: Match, reversed: bool) -> Boundaries {
    let mut left = matched.start().saturating_sub(1);
    let mut right = std::cmp::min(matched.end() + 1, length) - left;
    if reversed {
        left = length.saturating_sub(matched.start());
        right = length.saturating_sub(matched.end()).saturating_sub(1);
    }
    Boundaries {
        top: row.saturating_sub(1),
        bottom: std::cmp::min(row + 2, 3),
        left,
        right,
    }
}

fn create_bitmap<T: Default>(
    lines: core::str::Split<char>,
    filter: impl Fn(char) -> bool,
    matched: impl Fn(T) -> T,
    empty: impl Fn(T) -> T,
) -> Vec<T> {
    let mut symbols: Vec<T> = Vec::new();
    for line in lines {
        let mut markings = T::default();
        for symbol in line.chars() {
            if filter(symbol) {
                markings = matched(markings);
            } else {
                markings = empty(markings);
            }
        }
        symbols.push(markings);
    }
    symbols
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
fn test_adjacent_gear_ratios() {
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
    let actual = adjacent_gear_ratios(input.to_string());
    assert_eq!(actual, 467835);
    assert!(true);
}

#[test]
fn test_gear_ratios_part_one() {
    let input = std::fs::read_to_string("./inputs/day03").unwrap();
    let actual = engine_schematic_sum(input);
    assert_eq!(actual, 553825);
}

#[test]
fn test_gear_ratios_part_two() {
    let input = std::fs::read_to_string("./inputs/day03").unwrap();
    let actual = adjacent_gear_ratios(input);
    assert_eq!(actual, 93994191);
}
