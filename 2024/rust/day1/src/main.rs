fn main() {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if a filename was provided
    if args.len() < 2 {
        println!("Please provide a filename as argument");
        return;
    }

    match std::fs::read_to_string(&args[1]) {
        Ok(contents) => {
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

            column1.sort();
            column2.sort();

            // Print results
            // println!("Sorted Column 1: {:?}", column1);
            // println!("Sorted Column 2: {:?}", column2);

            // Calculate total absolute difference
            let total_difference: i32 = column1
                .iter()
                .zip(column2.iter())
                .map(|(a, b)| (a - b).abs())
                .sum();

            // Print results
            println!("Total absolute difference: {}", total_difference);
        }

        Err(e) => println!("Error reading file: {}", e),
    }
}
