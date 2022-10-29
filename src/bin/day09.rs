use std::fs::File;
use std::io::{BufRead, BufReader};


/// Determine if goal is the sum of two different numbers from slice
fn is_sum_from(goal: i64, slice: &[i64]) -> bool {
    let mut data = vec![0; slice.len()];
    data.copy_from_slice(slice);
    data.sort();

    if data.len() < 2 {
        return false;
    }

    let mut bottom: usize = 0;
    let mut top: usize = data.len() - 1;

    let mut sum;
    while bottom < top {
        sum = data[bottom] + data[top];
        if goal > sum {
            bottom += 1;
        }
        else if goal < sum {
            top -= 1;
        }
        else if data[bottom] != data[top] {
            return true;
        }
        
    }
    false
}


/// Return the first number that is not a sum of two of the previous window_size numbers
fn first_number_not_a_sum(window_size: usize, numbers: &[i64]) -> Option<i64> {
    (0..(numbers.len() - window_size as usize))
        .map(|n| (n as usize, (n + window_size) as usize))
        .filter(|(start, end)| ! is_sum_from(numbers[*end], &numbers[(*start)..(*end)]))
        .map(|(_start, end)| numbers[end])
        .next()
}


/// Find the contigous slice of numbers that sum to goal
///
/// Returns the start and end index of the slice.
fn contigous_sum(goal: i64, numbers: &[i64]) -> Option<(usize, usize)> {

    let mut start: usize = 0;
    let mut end: usize = 1;

    let mut sum: i64 = 0;

    let mut current_slice: &[i64]; // = &numbers[0..0];

    while goal != sum && start < end && end < numbers.len() {
        current_slice = &numbers[start..=end];
        sum = current_slice.iter().sum();

        //eprintln!("{:>4} | {:>4} | {}<>{} {:?}", start, end, sum, goal, sum.cmp(&goal));

        match sum.cmp(&goal) {
            std::cmp::Ordering::Less => end += 1,
            std::cmp::Ordering::Equal => return Some((start, end)),
            std::cmp::Ordering::Greater => start += 1,
        }
        if start == end {
            end += 1;
        }
    }

    return None;
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window_size: usize = args
        .get(1).expect("missing window size")
        .parse::<>().expect("failed to parse window size");

    let filename = args.get(2).expect("missing filename");
    let input_file = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(input_file);

    let numbers: Vec<i64> = reader.lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<i64>().ok())
        .collect();

    // Part A
    let result = first_number_not_a_sum(window_size, &numbers);
    println!("Day 09 A: {:?}", result);

    // Part B
    if let Some(goal) = result {
        if let Some((start, end)) = contigous_sum(goal, &numbers[..]) {
            let (min, max) = &numbers[start..=end]
                .iter()
                .fold((std::i64::MAX, std::i64::MIN),
                      |(min, max), value| (i64::min(min, *value), i64::max(max, *value)));

            println!("Day 09 B: {} + {} = {}", min, max, min + max);
        }
    }
}

#[cfg(test)]
mod test {
    use super::is_sum_from;

    #[test]
    fn empty_slice_should_not_a_sum() {
        // Given
        let goal = 42;
        let numbers: Vec<i64> = vec![];

        // When
        let result = is_sum_from(goal, &numbers[0..0]);

        // Then
        assert_eq!(false, result);
    }

    #[test]
    fn trivial_list_should_be_a_sum() {
        // Given
        let goal = 3;
        let numbers: Vec<i64> = vec![1,2];

        // When
        let result = is_sum_from(goal, &numbers[0..numbers.len()]);

        // Then
        assert_eq!(true, result);
    }
}
