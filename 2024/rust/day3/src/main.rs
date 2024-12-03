use regex::Regex;

fn part_one() {
    let contents =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // Create regex pattern for mul(number,number)
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    // Find all matches and multiply each pair
    for cap in re.captures_iter(&contents) {
        if let (Ok(num1), Ok(num2)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
            sum += num1 * num2;
        }
    }

    println!("Part one: {}", sum);
}

fn part_two() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
        // need to remove newlines, wasnt an issue in part one
        .replace('\n', "");

    // First find all content between do() and don't()
    let section_re = Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    // Then find all mul(x,y) within those sections
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    // For each section between do() and don't()
    for section in section_re.captures_iter(&contents) {
        // Find all mul patterns in this section
        for cap in mul_re.captures_iter(&section[1]) {
            if let (Ok(num1), Ok(num2)) = (cap[1].parse::<u32>(), cap[2].parse::<u32>()) {
                sum += num1 * num2;
            }
        }
    }

    println!("Part two: {}", sum);
}

fn main() {
    part_one();
    part_two();
}
