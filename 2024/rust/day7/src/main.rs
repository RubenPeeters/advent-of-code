use std::env;
use std::fs;

fn evaluate_part_one(nums: &[i64], ops: &[&str]) -> i64 {
    let mut result = nums[0];
    for i in 0..ops.len() {
        match ops[i] {
            "+" => result += nums[i + 1],
            "*" => result *= nums[i + 1],
            "||" => {
                let concat = format!("{}{}", result, nums[i + 1]);
                result = concat.parse().expect("Failed to parse concatenated number");
            }
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn can_make_value_part_one(target: i64, nums: &[i64]) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }

    let ops_count = nums.len() - 1;
    let combinations = 1 << ops_count; // 2^ops_count combinations

    for i in 0..combinations {
        let mut ops = Vec::new();
        for j in 0..ops_count {
            ops.push(if (i & (1 << j)) == 0 { "+" } else { "*" });
        }
        if evaluate_part_one(nums, &ops) == target {
            return true;
        }
    }
    false
}

fn evaluate(nums: &[i64], ops: &[&str]) -> i64 {
    let mut result = nums[0];
    for i in 0..ops.len() {
        match ops[i] {
            "+" => result += nums[i + 1],
            "*" => result *= nums[i + 1],
            "||" => {
                let concat = format!("{}{}", result, nums[i + 1]);
                result = concat.parse().expect("Failed to parse concatenated number");
            }
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn can_make_value(target: i64, nums: &[i64]) -> bool {
    if nums.len() == 1 {
        return target == nums[0];
    }

    let ops_count = nums.len() - 1;
    let combinations = 3_i64.pow(ops_count as u32); // Now 3^ops_count combinations

    for i in 0..combinations {
        let mut ops = Vec::new();
        let mut n = i;
        for _ in 0..ops_count {
            ops.push(match n % 3 {
                0 => "+",
                1 => "*",
                2 => "||",
                _ => unreachable!(),
            });
            n /= 3;
        }
        if evaluate(nums, &ops) == target {
            return true;
        }
    }
    false
}

fn part_one() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let mut sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: i64 = parts[0].trim().parse().expect("Invalid target value");
        let nums: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
            .collect();

        if can_make_value_part_one(target, &nums) {
            sum += target;
        }
    }

    println!("Total calibration result: {}", sum);
}

fn part_two() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    let mut sum = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: i64 = parts[0].trim().parse().expect("Invalid target value");
        let nums: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
            .collect();

        if can_make_value(target, &nums) {
            sum += target;
        }
    }

    println!("Total calibration result: {}", sum);
}

fn main() {
    part_one();
    part_two();
}
