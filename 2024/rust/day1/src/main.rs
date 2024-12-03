use std::collections::HashMap;

fn read_columns_from_file(filename: &str) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let contents = std::fs::read_to_string(filename)?;
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() >= 2 {
            if let Ok(num1) = parts[0].parse::<i32>() {
                column1.push(num1);
            }
            if let Ok(num2) = parts[1].parse::<i32>() {
                column2.push(num2);
            }
        }
    }

    Ok((column1, column2))
}

fn part_one() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filename as argument");
        return;
    }

    match read_columns_from_file(&args[1]) {
        Ok((mut column1, mut column2)) => {
            column1.sort();
            column2.sort();

            let total_difference: i32 = column1
                .iter()
                .zip(column2.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();

            println!("Total absolute difference: {}", total_difference);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn part_two() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filename as argument");
        return;
    }

    match read_columns_from_file(&args[1]) {
        Ok((column1, column2)) => {
            // Count occurrences in column2
            let mut counter: HashMap<i32, i32> = HashMap::new();
            for &num in column2.iter() {
                *counter.entry(num).or_insert(0) += 1;
            }

            // For each number in column1, multiply it by its count in column2
            let result: Vec<i32> = column1
                .iter()
                .map(|&num| {
                    let count = counter.get(&num).unwrap_or(&0);
                    num * count
                })
                .collect();

            println!("Results: {:?}", result);
            // If you want the sum of all results:
            println!("Total sum: {}", result.iter().sum::<i32>());
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
fn main() {
    part_one();
    part_two();
}
