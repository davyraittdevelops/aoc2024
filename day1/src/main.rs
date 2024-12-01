use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./src/list.txt").unwrap();

    // print_contents(contents);

    let (mut left_array, mut right_array) = split_contents_in_two_arrays(contents);

    // Sorting vectors from low to high
    left_array.sort();
    right_array.sort();

    println!("Left array: {:?}", left_array);
    println!("Right array: {:?}", right_array);

    let mut distance_counter = 0i32;
    // Loop trough one of the arrays, size is the same so doesnt matter for now
    for (left_distance, right_distance) in left_array.iter().zip(right_array.iter()) {
        println!(
            "\n Left nmbr: {}, Right nmbr: {}",
            left_distance, right_distance
        );

        let distance = (left_distance - right_distance).abs();

        distance_counter += distance;
    }

    println!("Final answer is: {:?}", distance_counter);
}

fn split_contents_in_two_arrays(contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let lines: std::str::Lines<'_> = contents.lines();

    for line in lines {
        // println!("\n{line}");
        let mut count = 0u32;

        // Split string into two parts, first half is left and second half is right. Split by whitespace
        for string_part in line.split_whitespace() {
            if count == 0 {
                let string_part_to_string = string_part.to_string();
                let string_part_integer = string_part_to_string.parse::<i32>().unwrap();
                left_list.push(string_part_integer);
            }
            if count == 1 {
                let string_part_to_string = string_part.to_string();
                let string_part_integer = string_part_to_string.parse::<i32>().unwrap();
                right_list.push(string_part_integer);
            }

            count += 1;
        }
    }

    (left_list, right_list)
}
