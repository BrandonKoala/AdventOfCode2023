use std::collections::HashMap;
use regex::Regex;

fn create_number_hash_map() ->HashMap<&'static str, u32> {
    let mut number_hash_map: HashMap<&'static str, u32> = HashMap::new();

    number_hash_map.insert("one", 1);
    number_hash_map.insert("two", 2);
    number_hash_map.insert("three", 3);
    number_hash_map.insert("four", 4);
    number_hash_map.insert("five", 5);
    number_hash_map.insert("six", 6);
    number_hash_map.insert("seven", 7);
    number_hash_map.insert("eight", 8);
    number_hash_map.insert("nine", 9);

    // Return the hash map
    number_hash_map
}  

fn get_numbers_from_line(line: &str) -> Vec<u32> {
    let mut numbers_in_line: Vec<u32> = Vec::new();

    let number_hash_map: HashMap<&str, u32> = create_number_hash_map();

    // String to check. Defaults to empty string
    let mut string_to_check: String = String::new();

    for character in line.chars() {
        // If the character is a number, add it to the numbers
        if character.is_numeric() {
            numbers_in_line.push(character.to_digit(10).unwrap());
            string_to_check.clear();
        } else {
            // Add the character to the string to check
            string_to_check.push(character);

            // Check regex from hashmap to see if it matches
            let pattern = format!(
                r"{}",
                number_hash_map.keys().cloned().collect::<Vec<&str>>().join("|")
            );
            let re = Regex::new(&pattern).unwrap();

            // If the string matches, add the number to the numbers
            let captures: Option<regex::Captures<'_>> = re.captures(&string_to_check);
            if !captures.is_none() {
                // Get the matched string
                let matched_string: &str = captures.unwrap().get(0).unwrap().as_str();

                // Get the number from the hashmap and add it to the numbers
                let number: Option<&u32> = number_hash_map.get(&matched_string[..]);
                if !number.is_none() {
                    numbers_in_line.push(*number.unwrap());
                }
                
                // Clear the string, except for the last character
                string_to_check.clear();
                string_to_check.push(character);
            }
        }
    }

    // Return the numbers in line
    numbers_in_line
}

fn main() {
    println!("Hello, world!");

    // Read each line from input.txt
    let input: String = std::fs::read_to_string("./input.txt").unwrap();
    let lines: std::str::Lines<'_> = input.lines();

    let mut answer: u32 = 0;

    // Loop through each line and extract all numbers in each line
    for line in lines {
        let numbers_in_line: Vec<u32> = get_numbers_from_line(line);

        // Get the first and last number and join them together. If there is only one,
        // use the same number twice
        if numbers_in_line.len() == 0 {
            continue;
        }

        let first: u32 = numbers_in_line[0];
        let last: u32 = numbers_in_line[numbers_in_line.len() - 1];
        let number: u32 = first * 10 + last;

        // Print the number with the line
        println!("{}: {}", line, number);

        // Add the number to the answer
        answer += number;
    }

    // Print the answer
    println!("Answer: {}", answer);
}
