use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::process;

fn get_middle_index(vec: &Vec<i32>) -> usize {
    vec.len() / 2 // For even lengths this gives the right middle index
}

fn check_sequence(numbers: &Vec<i32>, map: &HashMap<i32, HashMap<String, Vec<i32>>>) -> bool {
    let mut correct = true;
    // For each number in the sequence
    for (i, &current_num) in numbers.iter().enumerate() {
        // Get all numbers after current_num in the sequence
        let numbers_before = &numbers[..i];
        let numbers_after = &numbers[i + 1..];
        // Check if current number exists in map
        if let Some(relationships) = map.get(&current_num) {
            // Get "before" relationships from map
            if let Some(should_be_before) = relationships.get("before") {
                // Check that none of the numbers that should be before are actually after
                for &num in numbers_after {
                    if should_be_before.contains(&num) {
                        correct = false;
                    }
                }
            }

            // Get "after" relationships from map
            if let Some(should_be_after) = relationships.get("after") {
                // Check that all numbers that should be after are actually after
                for &num in numbers_before {
                    if should_be_after.contains(&num) {
                        correct = false;
                    }
                }
            }
        }
    }
    correct
}

fn topological_sort(numbers: &[i32], map: &HashMap<i32, HashMap<String, Vec<i32>>>) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();

    // Initialize in-degree and graph
    for &num in numbers {
        in_degree.insert(num, 0);
        graph.insert(num, Vec::new());
    }

    // Build the graph and compute in-degrees
    for &num in numbers {
        if let Some(relationships) = map.get(&num) {
            if let Some(should_be_after) = relationships.get("after") {
                for &after_num in should_be_after {
                    if in_degree.contains_key(&after_num) {
                        graph.get_mut(&num).unwrap().push(after_num);
                        *in_degree.get_mut(&after_num).unwrap() += 1;
                    }
                }
            }
        }
    }

    // Collect nodes with zero in-degree
    let mut queue = VecDeque::new();
    for (&num, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(num);
        }
    }

    // Perform topological sort
    let mut sorted = Vec::new();
    while let Some(num) = queue.pop_front() {
        sorted.push(num);
        if let Some(neighbors) = graph.get(&num) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // If sorted length is not equal to numbers length, there was a cycle
    if sorted.len() != numbers.len() {
        panic!("Cycle detected in the graph, cannot sort");
    }

    sorted
}

fn part_one(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut map: HashMap<i32, HashMap<String, Vec<i32>>> = HashMap::new();

    // Read until first empty line
    let first_section = contents.lines().take_while(|line| !line.is_empty());

    // Process the lines before empty line
    for line in first_section {
        let (num1, num2) = line.split_once('|').expect("Expected line to contain '|'");

        let num1: i32 = num1.trim().parse().expect("Failed to parse first number");
        let num2: i32 = num2.trim().parse().expect("Failed to parse second number");

        // Handle num1 -> num2 relationship (after)
        if let Some(inner_map) = map.get_mut(&num1) {
            inner_map
                .entry("after".to_string())
                .or_insert_with(Vec::new)
                .push(num2);
        } else {
            let mut inner_map = HashMap::new();
            inner_map.insert("after".to_string(), vec![num2]);
            map.insert(num1, inner_map);
        }

        // Handle num2 -> num1 relationship (before)
        if let Some(inner_map) = map.get_mut(&num2) {
            inner_map
                .entry("before".to_string())
                .or_insert_with(Vec::new)
                .push(num1);
        } else {
            let mut inner_map = HashMap::new();
            inner_map.insert("before".to_string(), vec![num1]);
            map.insert(num2, inner_map);
        }
    }

    // Print the hashmap in a readable format
    // for (key, inner_map) in &map {
    //     for (direction, numbers) in inner_map {
    //         println!("  {}: {:?}", direction, numbers);
    //     }
    // }

    // Read the second section (after empty line)
    let second_section = contents
        .lines()
        .skip_while(|line| !line.is_empty()) // Skip until empty line
        .skip(1) // Skip the empty line itself
        .filter(|line| !line.is_empty()); // Skip any additional empty lines

    // Process each line in second section
    let mut count = 0;
    let mut sum: i32 = 0;
    for line in second_section {
        let numbers: Vec<i32> = line
            .split(',')
            .map(|n| n.trim().parse().expect("Failed to parse number"))
            .collect();
        let correct = check_sequence(&numbers, &map);

        if correct {
            count += 1;
            let middle_index = get_middle_index(&numbers);
            sum += numbers[middle_index]; // Add the middle number to sum
        }
    }

    println!("Count: {}", count);
    println!("Total sum: {}", sum);
}

fn part_two(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut map: HashMap<i32, HashMap<String, Vec<i32>>> = HashMap::new();

    // Read until first empty line
    let first_section = contents.lines().take_while(|line| !line.is_empty());

    // Process the lines before empty line
    for line in first_section {
        let (num1, num2) = line.split_once('|').expect("Expected line to contain '|'");

        let num1: i32 = num1.trim().parse().expect("Failed to parse first number");
        let num2: i32 = num2.trim().parse().expect("Failed to parse second number");

        // Handle num1 -> num2 relationship (after)
        if let Some(inner_map) = map.get_mut(&num1) {
            inner_map
                .entry("after".to_string())
                .or_insert_with(Vec::new)
                .push(num2);
        } else {
            let mut inner_map = HashMap::new();
            inner_map.insert("after".to_string(), vec![num2]);
            map.insert(num1, inner_map);
        }

        // Handle num2 -> num1 relationship (before)
        if let Some(inner_map) = map.get_mut(&num2) {
            inner_map
                .entry("before".to_string())
                .or_insert_with(Vec::new)
                .push(num1);
        } else {
            let mut inner_map = HashMap::new();
            inner_map.insert("before".to_string(), vec![num1]);
            map.insert(num2, inner_map);
        }
    }

    // Read the second section (after empty line)
    let second_section = contents
        .lines()
        .skip_while(|line| !line.is_empty()) // Skip until empty line
        .skip(1) // Skip the empty line itself
        .filter(|line| !line.is_empty()); // Skip any additional empty lines

    // Process each line in second section
    let mut count = 0;
    let mut sum: i32 = 0;
    for line in second_section {
        let numbers: Vec<i32> = line
            .split(',')
            .map(|n| n.trim().parse().expect("Failed to parse number"))
            .collect();

        println!("================");
        println!("Original sequence: {:?}", numbers);

        // Fix the sequence using topological sort
        let fixed_numbers = topological_sort(&numbers, &map);

        if fixed_numbers == numbers {
            count += 1;
            println!("Sequence is correct");
        } else {
            println!("Fixed sequence: {:?}", fixed_numbers);
            let middle_index = get_middle_index(&fixed_numbers);
            sum += fixed_numbers[middle_index]; // Add the middle number to sum
        }
    }

    println!("Count: {}", count);
    println!("Total sum: {}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if a filename was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    // Get the filename from the arguments
    let filename = &args[1];
    part_one(filename);
    part_two(filename);
}
