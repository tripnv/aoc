use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

fn parse_lines(lines: &Vec<String>) -> Result<(Vec<i32>, Vec<i32>), std::num::ParseIntError> {
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();

    println!("{:?}", lines);

    for line in lines {
        let segments: Vec<&str> = line.split_whitespace().collect();
        println!("{:?}", segments);
        if segments.len() >= 2 {
            first_col.push(segments[0].parse::<i32>()?);
            second_col.push(segments[1].parse::<i32>()?);
        }
        // if the length of the segments is < 2 -> panic
    }
    Ok((first_col, second_col))
}

fn calculate_diffs(seq_1: &Vec<i32>, seq_2: &Vec<i32>) -> Vec<i32> {
    let mut seq_1_sorted = seq_1.clone();
    let mut seq_2_sorted = seq_2.clone();
    seq_1_sorted.sort();
    seq_2_sorted.sort();

    seq_1_sorted
        .iter()
        .zip(seq_2_sorted.iter())
        .map(|(a, b)| (b - a).abs())
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/input.txt";

    let lines = read_file(&path).unwrap();
    let (col_1, col_2) = parse_lines(&lines)?;
    let diffs = calculate_diffs(&col_1, &col_2);

    let total_diffs: i32 = diffs.iter().sum();
    let count_diffs: i32 = (diffs.len() - 1).try_into().unwrap();

    println!("Summed differences: {}", total_diffs);
    println!("Num lines: {:?}", count_diffs);
    Ok(())
}
