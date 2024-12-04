use std::env;

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // right
    (0, -1),  // left
    (1, 0),   // down
    (-1, 0),  // up
    (1, 1),   // diagonal down-right
    (-1, -1), // diagonal up-left
    (1, -1),  // diagonal down-left
    (-1, 1),  // diagonal up-right
];

fn search_word(matrix: &Vec<Vec<char>>, word: &str, row: i32, col: i32, dir: (i32, i32)) -> bool {
    let (rows, cols) = (matrix.len() as i32, matrix[0].len() as i32);
    let word_chars: Vec<char> = word.chars().collect();

    // Check if the word would fit in this direction
    let end_row = row + (word.len() as i32 - 1) * dir.0;
    let end_col = col + (word.len() as i32 - 1) * dir.1;
    if end_row < 0 || end_row >= rows || end_col < 0 || end_col >= cols {
        return false;
    }

    // Check each character of the word
    for (i, &word_char) in word_chars.iter().enumerate() {
        let curr_row = row + (i as i32 * dir.0);
        let curr_col = col + (i as i32 * dir.1);
        if matrix[curr_row as usize][curr_col as usize] != word_char {
            return false;
        }
    }
    true
}

fn find_word(matrix: &Vec<Vec<char>>, word: &str) -> Vec<(usize, usize, &'static str)> {
    let mut found_positions = Vec::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    for row in 0..rows {
        for col in 0..cols {
            for (dir, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
                if search_word(matrix, word, row as i32, col as i32, (dx, dy)) {
                    let direction = match dir {
                        0 => "right",
                        1 => "left",
                        2 => "down",
                        3 => "up",
                        4 => "down-right",
                        5 => "up-left",
                        6 => "down-left",
                        7 => "up-right",
                        _ => "unknown",
                    };
                    found_positions.push((row, col, direction));
                }
            }
        }
    }
    found_positions
}

fn check_x_pattern(matrix: &Vec<Vec<char>>, row: usize, col: usize, word: &str) -> bool {
    let word_len = word.len();

    // Check if we have enough space for the grid
    if row + word_len - 1 >= matrix.len() || col + word_len - 1 >= matrix[0].len() {
        return false;
    }

    // Check diagonal from top-left to bottom-right
    let mut diagonal1 = Vec::with_capacity(word_len);
    for i in 0..word_len {
        diagonal1.push(matrix[row + i][col + i]);
    }

    // Check diagonal from top-right to bottom-left
    let mut diagonal2 = Vec::with_capacity(word_len);
    for i in 0..word_len {
        diagonal2.push(matrix[row + i][col + word_len - 1 - i]);
    }

    // Check if either diagonal spells the word
    let target: Vec<char> = word.chars().collect();
    let target_rev: Vec<char> = word.chars().rev().collect();

    // Check all possible combinations:
    // 1. Both diagonals forward
    // 2. Both diagonals reversed
    // 3. Diagonal1 forward, Diagonal2 reversed
    // 4. Diagonal1 reversed, Diagonal2 forward
    (diagonal1 == target[..] && diagonal2 == target[..])
        || (diagonal1 == target_rev[..] && diagonal2 == target_rev[..])
        || (diagonal1 == target[..] && diagonal2 == target_rev[..])
        || (diagonal1 == target_rev[..] && diagonal2 == target[..])
}

fn part_one(filename: &str) {
    // Read file contents
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");
    // Parse input into a 2D matrix
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let word = "XMAS"; // Replace with your target word
    let found = find_word(&matrix, word);

    if found.is_empty() {
        println!("Word '{}' not found", word);
    } else {
        println!("Found {} words", found.len());
    }
}

fn part_two(filename: &str) {
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");
    // Parse input into a 2D matrix
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let word = "MAS"; // Can be changed to any word
    let mut found_positions = Vec::new();

    // Iterate through all possible grid positions
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if check_x_pattern(&matrix, row, col, word) {
                found_positions.push((row, col));
            }
        }
    }

    if found_positions.is_empty() {
        println!("No X patterns of '{}' found", word);
    } else {
        println!(
            "Found {} X patterns of '{}' ({}x{} grids):",
            found_positions.len(),
            word,
            word.len(),
            word.len()
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    part_one(filename);
    part_two(filename);
}
