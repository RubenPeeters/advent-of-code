use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn part_one(filename: &str) {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find starting position and direction
    let mut start_pos = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                start_pos = Some((i as i32, j as i32));
                break;
            }
        }
    }

    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
    let mut visited = HashSet::new();
    let (mut curr_pos, mut curr_dir) = (start_pos.unwrap(), Direction::Up);
    visited.insert(curr_pos);

    loop {
        let (delta_row, delta_col) = curr_dir.get_delta();
        let next_pos = (curr_pos.0 + delta_row, curr_pos.1 + delta_col);

        // Check if we're out of bounds
        if next_pos.0 < 0 || next_pos.0 >= rows || next_pos.1 < 0 || next_pos.1 >= cols {
            break;
        }

        // Check if there's an obstacle ahead
        if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            curr_dir = curr_dir.turn_right();
        } else {
            curr_pos = next_pos;
            visited.insert(curr_pos);
        }
    }

    println!("The guard visits {} distinct positions.", visited.len());
}

fn is_loop(grid: &Vec<Vec<char>>, start_pos: (i32, i32)) -> bool {
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
    let mut visited = HashSet::new();
    let mut pos_with_dir = HashSet::new();
    let (mut curr_pos, mut curr_dir) = (start_pos, Direction::Up);

    visited.insert(curr_pos);
    pos_with_dir.insert((curr_pos, curr_dir));

    loop {
        let (delta_row, delta_col) = curr_dir.get_delta();
        let next_pos = (curr_pos.0 + delta_row, curr_pos.1 + delta_col);

        // Check if we're out of bounds
        if next_pos.0 < 0 || next_pos.0 >= rows || next_pos.1 < 0 || next_pos.1 >= cols {
            return false;
        }

        // Check if there's an obstacle ahead
        if grid[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            curr_dir = curr_dir.turn_right();
            // If we've seen this position and direction before, it's a loop
            if pos_with_dir.contains(&(curr_pos, curr_dir)) {
                return true;
            }
            pos_with_dir.insert((curr_pos, curr_dir));
        } else {
            curr_pos = next_pos;
            visited.insert(curr_pos);
            // If we've seen this position and direction before, it's a loop
            if pos_with_dir.contains(&(curr_pos, curr_dir)) {
                return true;
            }
            pos_with_dir.insert((curr_pos, curr_dir));
        }
    }
}

fn part_two_naive(filename: &str) {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find starting position
    let mut start_pos = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                start_pos = Some((i as i32, j as i32));
                break;
            }
        }
    }
    let start_pos = start_pos.unwrap();

    // Try placing an obstruction at each empty position
    let mut loop_positions = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' && (i as i32, j as i32) != start_pos {
                // Try placing an obstruction here
                grid[i][j] = '#';
                if is_loop(&grid, start_pos) {
                    loop_positions += 1;
                }
                // Remove the obstruction
                grid[i][j] = '.';
            }
        }
    }

    println!(
        "Number of possible obstruction positions: {}",
        loop_positions
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    part_one(&args[1]);
    part_two_naive(&args[1]);
}
