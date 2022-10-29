use std::fs::File;
use std::io::{BufRead, BufReader};


fn validate(goal: i32, slice: &[i32]) -> bool {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let window_size: usize = args
        .get(1).expect("missing window size")
        .parse::<>().expect("failed to parse window size");

    let filename = args.get(2).expect("missing filename");
    let input_file = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(input_file);

    let numbers: Vec<i32> = reader.lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    // TODO: use this to go through numbers
    let result = (0..(numbers.len() - window_size as usize))
        .map(|n| (n as usize, (n + window_size) as usize))
        .filter(|(start, end)| ! validate(numbers[*end], &numbers[(*start)..(*end)]))
        .map(|(_start, end)| numbers[end])
        .next();
    println!("{:?}", result);

}

#[cfg(test)]
mod test {
    use super::validate;

    #[test]
    fn empty_slice_should_not_validate() {
        // Given
        let goal = 42;
        let numbers: Vec<i32> = vec![];

        // When
        let result = validate(goal, &numbers[0..0]);

        // Then
        assert_eq!(false, result);
    }

    #[test]
    fn trivial_list_should_validate() {
        // Given
        let goal = 3;
        let numbers: Vec<i32> = vec![1,2];

        // When
        let result = validate(goal, &numbers[0..numbers.len()]);

        // Then
        assert!(result);
    }
}
