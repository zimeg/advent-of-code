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
