use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;

fn main() {
    let _file_name = "input_data.txt";

    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(_file_name) {
        for line in lines {
            match line {
                Ok(chars) => {
                    let dictionary: HashMap<String, String> = get_word_digit_mapping();
                    let parsed_chars = replace_word_numbers_with_digits(chars, dictionary);
                    if let Ok(partial_digits) = get_clear_values(parsed_chars) {
                        sum = sum + partial_digits;
                    }
                }
                Err(e) => eprintln!("Error occurred: {}", e),
            }
        }
    }
    println!("Final result is: {}", sum);

}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_word_digit_mapping() -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    result.insert("one".to_string(), "o1e".to_string());
    result.insert("two".to_string(), "t2o".to_string());
    result.insert("three".to_string(), "t3e".to_string());
    result.insert("four".to_string(), "f4r".to_string());
    result.insert("five".to_string(), "f5e".to_string());
    result.insert("six".to_string(), "s6x".to_string());
    result.insert("seven".to_string(), "s7n".to_string());
    result.insert("eight".to_string(), "e8t".to_string());
    result.insert("nine".to_string(), "n9e".to_string());

    result
}

fn replace_word_numbers_with_digits(data: String, dictionary: HashMap<String, String>) -> String {
    let mut result: String = data;
    
    for (key,value) in &dictionary {
        result = result.replace(key, value)
    }

    result
}

fn get_clear_values(blured_data: String) -> std::result::Result<i32, ParseIntError> {
    let mut numbers: Vec<char> = Vec::new();

    for character in blured_data.chars() {
        if character.is_digit(10) {
            numbers.push(character);
        }
    }
    let result: std::result::Result<i32, ParseIntError>;
    if numbers.len() == 1 {
        result = format!("{}{}", numbers[0], numbers[0]).parse::<i32>();
    } else {
        result = format!("{}{}", numbers[0], numbers[numbers.len() - 1]).parse::<i32>();
    }
    result
}
