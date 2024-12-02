use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let mut converted_array = split_contents_in_array(contents);

    let mut safe_counter = 0i32;
    let mut unsafe_arrays: Vec<&Vec<i32>> = Vec::new();

    for sub_array in &converted_array {
        println!("{:#?}", sub_array);

        if is_safe(sub_array) {
            safe_counter += 1;
        } else {
            unsafe_arrays.push(sub_array);
        }
    }

    println!("Final answer part 1 is: {:?}", safe_counter);
    println!(
        "Found {:?} unsafe arrays to check for part 2",
        unsafe_arrays.len()
    );

    let mut part2_counter = safe_counter; // Start with part 1 safe count

    for unsafe_array in unsafe_arrays {
        println!("Processing unsafe array: {:?}", unsafe_array);
        let mut found_safe = false;

        // Try removing each index once
        for i in 0..unsafe_array.len() {
            // Create a copy without the current index
            let mut test_array: Vec<i32> = unsafe_array.clone();
            test_array.remove(i);

            // Check if this variation is safe
            if is_safe(&test_array) {
                found_safe = true;
                break; // We found a safe variation, no need to check more
            }
        }

        if found_safe {
            part2_counter += 1;
        }
    }

    println!("Final answer part 2 is: {:?}", part2_counter);
}

fn is_safe(array: &Vec<i32>) -> bool {
    let mut is_first = true;
    let mut increasing: bool = false;
    let mut is_safe = true;

    for window in array.windows(2) {
        let diff = window[1] - window[0];

        // Set the baseline of increasing or decreasing
        if is_first {
            is_first = false;
            if diff > 0 {
                increasing = true;
            } else {
                increasing = false;
            }
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

    is_safe
}

fn split_contents_in_array(contents: String) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let mut row: Vec<i32> = Vec::new();

        for string_part in line.split_whitespace() {
            row.push(string_part.parse().unwrap());
        }

        result.push(row);
    }

    result
}
