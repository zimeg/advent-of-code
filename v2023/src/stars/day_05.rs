use crate::gifts::parser;
use std::str::Split;

struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
}

pub fn seed_to_location_min(input: String) -> u32 {
    let mut mappings = input.split("\n\n");
    let seeds = parser::gather_numbers(mappings.next().unwrap());
    let maps = create_mappings(mappings);
    let mut lowest = u64::MAX;
    for seed in seeds {
        let location = permute_location(seed, &maps);
        lowest = std::cmp::min(lowest, location);
    }
    lowest.try_into().unwrap()
}

pub fn seed_to_location_range(input: String) -> u32 {
    let mut mappings = input.split("\n\n");
    let ranges = parser::gather_numbers(mappings.next().unwrap());
    let maps = create_mappings(mappings);
    let mut lowest = u64::MAX;
    for (index, start) in ranges.clone().into_iter().enumerate() {
        if index % 2 != 0 {
            continue;
        }
        for seed in start..start + ranges.get(index + 1).unwrap() {
            let location = permute_location(seed, &maps);
            lowest = std::cmp::min(lowest, location);
        }
    }
    lowest.try_into().unwrap()
}

fn create_mappings(mappings: Split<&str>) -> Vec<Vec<Mapping>> {
    let mut maps: Vec<Vec<Mapping>> = vec![];
    for map in mappings {
        let mut conversions: Vec<Mapping> = vec![];
        for conversion in map.lines().skip(1) {
            let values = parser::gather_numbers(conversion);
            let mapping = Mapping {
                destination: values[0],
                source: values[1],
                range: values[2],
            };
            conversions.push(mapping);
        }
        maps.push(conversions);
    }
    maps
}

fn permute_location(mut seed: u64, mappings: &Vec<Vec<Mapping>>) -> u64 {
    for map in mappings {
        for conversion in map {
            if seed >= conversion.source && seed < conversion.source + conversion.range {
                seed = (seed - conversion.source) + conversion.destination;
                break;
            }
        }
    }
    seed
}

#[test]
fn test_seed_to_location_min_example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let value = seed_to_location_min(input.to_string());
    assert_eq!(value, 35);
}

#[test]
fn test_seed_to_location_range_example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    let value = seed_to_location_range(input.to_string());
    assert_eq!(value, 46);
}

#[test]
fn test_almanac_fertilizer_part_one() {
    let input = std::fs::read_to_string("./inputs/day05").unwrap();
    let actual = seed_to_location_min(input);
    assert_eq!(actual, 51752125);
}

#[test]
fn test_almanac_fertilizer_part_two() {
    let input = std::fs::read_to_string("./inputs/day05").unwrap();
    let actual = seed_to_location_range(input);
    assert_eq!(actual, 12634632);
}
