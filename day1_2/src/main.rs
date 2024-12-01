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

    let mut similarity_score = 0i32;
    // This time, you'll need to figure out exactly how often each number from the left list appears in the right list.
    // Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.

    for left_number in left_array {
        println!("\n Left nmbr: {}", left_number);

        let nmbr_of_appearances = right_array.iter().filter(|&&x| x == left_number).count();
        println!("Appears {} times", nmbr_of_appearances);

        similarity_score += (left_number * nmbr_of_appearances as i32);
    }

    println!("Final answer is: {:?}", similarity_score);
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
