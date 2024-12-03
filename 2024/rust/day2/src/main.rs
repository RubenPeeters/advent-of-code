fn part_one() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Please provide a filename as argument");
        return;
    }

    match std::fs::read_to_string(&args[1]) {
        Ok(contents) => {
            let mut monotonic_count = 0;
            for line in contents.lines() {
                let numbers: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if is_valid_sequence(&numbers) {
                    monotonic_count += 1;
                }
            }
            println!("Total safe sequences: {}", monotonic_count);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn is_valid_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 1..numbers.len() {
        let diff = (numbers[i] - numbers[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false; // Invalid difference
        }
        if numbers[i] <= numbers[i - 1] {
            is_increasing = false;
        }
        if numbers[i] >= numbers[i - 1] {
            is_decreasing = false;
        }
    }

    is_increasing || is_decreasing
}

fn part_two() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a filename as argument");
        return;
    }

    match std::fs::read_to_string(&args[1]) {
        Ok(contents) => {
            let mut fixable_count = 0;

            for line in contents.lines() {
                let numbers: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                // Try removing each number one at a time
                for skip_index in 0..numbers.len() {
                    let modified_sequence: Vec<i32> = numbers
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != skip_index)
                        .map(|(_, &n)| n)
                        .collect();

                    if is_valid_sequence(&modified_sequence) {
                        fixable_count += 1;
                        break; // Found a valid solution, no need to try removing other numbers
                    }
                }
            }
            println!("Total fixable sequences: {}", fixable_count);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn main() {
    part_one();
    part_two();
}
