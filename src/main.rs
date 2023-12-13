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
                    if let Ok(partial_digits) = get_clear_values(chars) {
                        sum = sum + partial_digits;
                    }
                }
                Err(_) => print!("Error occured"),
            }
        }
    }
    println!("Final result is: {sum}")
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
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
