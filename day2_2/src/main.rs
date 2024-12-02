use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let (mut converted_array) = split_contents_in_array(contents);
    let mut safe_counter = 0i32;

    for sub_array in &converted_array {
        println!("{:#?}", sub_array);

        let mut is_first = true;
        let mut increasing: bool = false;
        let mut is_safe = true;

        for window in sub_array.windows(2) {
            let diff = window[1] - window[0];

            // Set the baseline of increasing or decreasing
            if is_first {
                is_first = false;
                if diff > 0 {
                    increasing = true;
                } else {
                    increasing = false;
                }
                println!(
                    "This sub array is {}",
                    if increasing {
                        "increasing"
                    } else {
                        "decreasing"
                    }
                );
            }

            // Check if pattern maintains and diff is within bounds
            if increasing {
                // If increasing, diff should be positive and <= 3
                if diff <= 0 || diff > 3 {
                    is_safe = false;
                    break; // No need to check further
                }
            } else {
                // If decreasing, diff should be negative and >= -3
                if diff >= 0 || diff < -3 {
                    is_safe = false;
                    break; // No need to check further
                }
            }
        }

        // After the loop, check if sequence was safe
        if is_safe {
            safe_counter += 1;
        }
    }

    println!("Final answer is: {:?}", safe_counter);
}

fn split_contents_in_array(contents: String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();

        for string_part in line.split_whitespace() {
            // Convert string to i32, using unwrap() for simplicity
            // In production code, you'd want proper error handling
            row.push(string_part.parse().unwrap());
        }

        result.push(row);
    }

    result
}
