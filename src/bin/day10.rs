use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Part A: Count different sized jumps between adapters
fn adapter_difference_count(adapters: &[i32]) -> (i32, i32) {
    let mut numbers = vec![0; adapters.len() + 1];  // Need space for initial zero
    // To include jump from charging outlet (already 0 from initialization)
    numbers[1..].copy_from_slice(adapters);

    numbers.sort();

    let (ones, threes) = numbers.windows(2).map(|w| {
        match w {
            &[a, b] => b - a,
            _ => 0,
        }
    })
    // Start with one three to include the jump from last adapter to the device
    .fold((0, 1), |(ones, threes), d| {
        match d {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        }
    });

    (ones, threes)
}

/// Part B: Count the number of ways the adapters can be combined
fn number_of_identical_chains(adapters: &[i32]) -> u64 {
    let mut numbers = vec![0; adapters.len()];
    numbers.copy_from_slice(adapters);
    numbers.sort_by_key(|&w| Reverse(w));

    let mut cache: HashMap<i32, u64> = HashMap::new();
    // Prepopulate cache with the possible solutions for the last adapter.
    cache.insert(numbers[0], 1);

    numbers[1..].iter()
        .for_each(|&a| {
            // Find all combinations we can reach that are in the cache
            let sum = (a..=(a+3)).filter_map(|k| cache.get(&k)).sum();
            //eprintln!("{:>3}:{}", a, sum);
            cache.insert(a, sum);
        });

    let combinations = (0..=3).filter_map(|k| cache.get(&k)).map(|&v| v).sum();

    combinations
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("missing filename");
    let input_file = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(input_file);

    let numbers: Vec<i32> = reader.lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    // Part A
    let (ones, threes) = adapter_difference_count(&numbers[..]);
    println!("Day 10 A: {} x {} = {}", ones, threes, ones * threes);

    // Part B
    let combinations = number_of_identical_chains(&numbers[..]);
    println!("Day 10 B: {}", combinations);
}
