use eval::eval;
use regex::Regex;
use std::fs;

fn solve_part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut sum = 0i64;

    for capture in re.find_iter(input) {
        let capture_str = capture
            .as_str()
            .replace("mul(", "")
            .replace(")", "")
            .replace(",", "*");

        if let Ok(result) = eval(&capture_str) {
            let number: i64 = result.as_f64().unwrap() as i64;
            sum += number;
            println!("{} = {}", capture_str, number);
        }
    }

    sum
}

fn solve_part2(input: &str) -> i64 {
    let mut sum = 0;
    let mut enabled = true; // Start with multiplications enabled

    // Match all three types of instructions: mul(x,y), do(), and don't()
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();

    for cap in re.captures_iter(input) {
        if let Some(mul) = cap.get(1) {
            // Multiplication
            if enabled {
                let mul_str = mul
                    .as_str()
                    .replace("mul(", "")
                    .replace(")", "")
                    .replace(",", "*");

                if let Ok(result) = eval(&mul_str) {
                    let number = result.as_f64().unwrap() as i64;
                    println!("{} = {}", mul_str, number);
                    sum += number;
                }
            }
        } else if let Some(_) = cap.get(2) {
            // do()
            enabled = true;
            println!("Enabling multiplications");
        } else if let Some(_) = cap.get(3) {
            // don't()
            enabled = false;
            println!("Disabling multiplications");
        }
    }

    sum
}
fn main() {
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    // let part1_result = solve_part1(&contents);
    // println!("Part 1 result: {}", part1_result);

    let part2_result = solve_part2(&contents);
    println!("Part 2 result: {}", part2_result);
}
