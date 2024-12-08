use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                positions.entry(c).or_insert_with(Vec::new).push((x, y));
            }
            if x > max_x {
                max_x = x;
            }
        }
        if y > max_y {
            max_y = y;
        }
    }
    println!("Positions:");
    for (c, pos) in positions.iter() {
        println!("{}: {:?}", c, pos);
    }
    let mut found_points: HashSet<(i32, i32)> = HashSet::new();
    for positions in positions.values() {
        let n = positions.len();
        if n >= 2 {
            for i in 0..n - 1 {
                for j in i + 1..n {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    let dx = x2 as i32 - x1 as i32;
                    let dy = y2 as i32 - y1 as i32;
                    // dx and dy indicate the direction of the line from x1,y1 to x2,y2
                    // we generate the point in the opposite direction from x1,y1, and in the same direction from x2,y2
                    // and check if they are the same
                    let (x3, y3) = (x1 as i32 - dx, y1 as i32 - dy);
                    let (x4, y4) = (x2 as i32 + dx, y2 as i32 + dy);
                    if x3 >= 0 && x3 <= max_x as i32 && y3 >= 0 && y3 <= max_y as i32 {
                        found_points.insert((x3, y3));
                    }
                    if x4 >= 0 && x4 <= max_x as i32 && y4 >= 0 && y4 <= max_y as i32 {
                        found_points.insert((x4, y4));
                    }
                }
            }
        }
    }
    println!("Found points visualization:");
    for y in 0..=max_y {
        for x in 0..=max_x {
            if found_points.contains(&(x as i32, y as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("Found points: {}", found_points.len());
}

fn part_two(input: &str) {
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                positions.entry(c).or_insert_with(Vec::new).push((x, y));
            }
            if x > max_x {
                max_x = x;
            }
        }
        if y > max_y {
            max_y = y;
        }
    }
    println!("Positions:");
    for (c, pos) in positions.iter() {
        println!("{}: {:?}", c, pos);
    }
    let mut found_points: HashSet<(i32, i32)> = HashSet::new();
    for positions in positions.values() {
        let n = positions.len();
        if n >= 2 {
            for i in 0..n - 1 {
                for j in i + 1..n {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    found_points.insert((x1 as i32, y1 as i32));
                    found_points.insert((x2 as i32, y2 as i32));
                    let dx = x2 as i32 - x1 as i32;
                    let dy = y2 as i32 - y1 as i32;
                    // dx and dy indicate the direction of the line from x1,y1 to x2,y2
                    // we generate the point in the opposite direction from x1,y1, and in the same direction from x2,y2
                    // and check if they are the same
                    let mut x3 = x1 as i32 - dx;
                    let mut y3 = y1 as i32 - dy;
                    while x3 >= 0 && x3 <= max_x as i32 && y3 >= 0 && y3 <= max_y as i32 {
                        found_points.insert((x3, y3));
                        x3 -= dx;
                        y3 -= dy;
                    }
                    let mut x4 = x2 as i32 + dx;
                    let mut y4 = y2 as i32 + dy;
                    while x4 >= 0 && x4 <= max_x as i32 && y4 >= 0 && y4 <= max_y as i32 {
                        found_points.insert((x4, y4));
                        x4 += dx;
                        y4 += dy;
                    }
                }
            }
        }
    }
    println!("Found points visualization:");
    for y in 0..=max_y {
        for x in 0..=max_x {
            if found_points.contains(&(x as i32, y as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("Found points: {}", found_points.len());
}
