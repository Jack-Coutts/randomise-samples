

use std::fs::File;
use std::io::{self, BufRead, Write, stdin};
use std::path::Path;
use rand::seq::SliceRandom;

fn main() -> io::Result<()> {
    // Get input file path from the user
    println!("Enter the path to the CSV file you want to shuffle:");
    let mut input_path = String::new();
    stdin().read_line(&mut input_path)?;
    let input_path = input_path.trim();

    // Get output file base name from the user (without extension)
    println!("Enter the desired base name for the output files (without extension):");
    let mut base_output_path = String::new();
    stdin().read_line(&mut base_output_path)?;
    let base_output_path = base_output_path.trim();

    // Ask the user which rows should not be shuffled
    println!("Enter the row numbers (starting from the 3rd row, separated by commas) that shouldn't be shuffled:");
    let mut non_shuffled_rows = String::new();
    stdin().read_line(&mut non_shuffled_rows)?;
    let non_shuffled_indices: Vec<usize> = non_shuffled_rows
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .map(|i| i + 2 - 1) // Adjust for 0-based indexing and add two for headers
        .collect();


    // Read the file
    let file = File::open(Path::new(&input_path))?;
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Split off header lines and non-shuffled rows
    let headers = vec![lines.remove(0), lines.remove(0)];
    
    // Separate the non-shuffled rows
    let mut non_shuffled_lines: Vec<String> = non_shuffled_indices.iter().map(|&i| lines[i - 2].clone()).collect();
    for &i in non_shuffled_indices.iter().rev() {
        lines.remove(i - 2);
    }


    // Shuffle the remaining lines
    let mut rng = rand::thread_rng();
    lines.shuffle(&mut rng);

    // Insert non-shuffled lines back into their positions
    for &i in &non_shuffled_indices {
        lines.insert(i - 2, non_shuffled_lines.remove(0));
    }

    // Write to the output files (.seq and .csv)
    let seq_output_path = format!("{}.seq", base_output_path);
    let csv_output_path = format!("{}.csv", base_output_path);

    let mut seq_output_file = File::create(Path::new(&seq_output_path))?;
    let mut csv_output_file = File::create(Path::new(&csv_output_path))?;

    for header in &headers {
        writeln!(seq_output_file, "{}", header.replace(",", "\t"))?;
        writeln!(csv_output_file, "{}", header)?;
    }

    for line in &lines {
        writeln!(seq_output_file, "{}", line.replace(",", "\t"))?;
        writeln!(csv_output_file, "{}", line)?;
    }
    
    println!("File shuffled and saved as {} and {}", seq_output_path, csv_output_path);

    Ok(())
}