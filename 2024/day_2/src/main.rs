use std::fs::File;
use std::i16;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

fn parse_lines(lines: &Vec<String>) -> Result<Vec<Vec<i16>>, std::num::ParseIntError> {
    let mut ragged_matrix: Vec<Vec<i16>> = Vec::new();
    for line in lines {
        // Use `map` and handle the error in a way that doesn't panic
        let char_array: Result<Vec<i16>, std::num::ParseIntError> =
            line.split_whitespace().map(|c| c.parse::<i16>()).collect();

        match char_array {
            Ok(parsed) => ragged_matrix.push(parsed),
            Err(e) => return Err(e),
        }
    }
    Ok(ragged_matrix)
}
fn get_differences(row: &Vec<i16>) -> Vec<i16> {
    row.windows(2).map(|x| (x[0] - x[1]) as i16).collect()
}

fn is_valid(numbers: &[i16]) -> bool {
    numbers.iter().all(|&x| x > 0 && x <= 3) || numbers.iter().all(|&x| x < 0 && x >= -3)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/input.txt";

    // Read the file
    let lines = read_file(path)?;

    // Parse the lines into a matrix
    let parsed_matrix = parse_lines(&lines)?;

    let result: Vec<i16> = parsed_matrix
        .iter()
        .map(|row| {
            let differences = get_differences(row);
            if is_valid(&differences) {
                1
            } else {
                0
            }
        })
        .collect();

    // Sum the 1s and 0s
    let sum: i16 = result.iter().sum();

    // Output the sum
    println!("Sum of 1s and 0s: {}", sum);
    Ok(())
}
