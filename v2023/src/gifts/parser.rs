use regex::Regex;

/// A value for each line in the `input` is computed using `func` and the total
/// sum of the results is returned.
pub fn sum_iteration(input: String, func: impl Fn(&str) -> u32) -> u32 {
    let lines = input.trim().split('\n');
    let mut sum = 0;
    for line in lines {
        sum += func(line);
    }
    sum
}

/// Collects the list of natural numbers in a string.
pub fn gather_numbers(line: &str) -> Vec<u64> {
    let regex = Regex::new(r"(?<n>[0-9]+)").unwrap();
    let values = regex.captures_iter(line);
    let mut numbers = vec![];
    for value in values {
        numbers.push(value["n"].parse().unwrap());
    }
    numbers
}

#[test]
fn test_sum_iteration_addition() {
    let input = "abcde
fghi
j
klmnop";
    let counter = |letters: &str| -> u32 { letters.len().try_into().unwrap() };
    let actual = sum_iteration(input.to_string(), counter);
    assert_eq!(actual, 16);
}

#[test]
fn test_gather_numbers() {
    let input = "example: 12 6 4 3 2 1";
    let actual = gather_numbers(input);
    assert_eq!(actual, vec![12, 6, 4, 3, 2, 1]);
}
