use std::fs;
use std::error;

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    const MUL: &str = "mul(";
    let mut is_mul_enabled = true;
    const DO: &str = "do()";
    const DONT: &str = "don't()";

    let mut found_word = String::from("");
    let mut found_number = String::from("");
    let mut result: u32 = 0;
    let mut values: Vec<u32>  = Vec::with_capacity(2);
    input.chars().for_each(|char| {
        if DO == &found_word {
            is_mul_enabled = true;
            found_word = String::from("");
        }
        if DONT == &found_word {
            is_mul_enabled = false;
            found_word = String::from("");
        }
        if  is_mul_enabled && MUL == &found_word {
            match char.to_digit(10) {
                Some(_) => found_number.push(char),
                None => {
                    // we only get into this block if we have found a non number
                    // if found_number is empty or the character we are looking at is not "," or
                    // ")", we need to continue looking => found_word = ""
                    if char != ',' && char != ')' {
                        values = Vec::with_capacity(2);
                        found_number = String::from("");
                        found_word = String::from("");
                    } 

                    // if found_number has a value, char is ",", values.len() = 0
                    else if char == ',' && values.len() == 0 {
                        // if found_number.len() > 3, we need to continue looking => found_word =
                        // found_number = ""
                        if found_number.len() > 3 {
                            found_number = String::from("");
                            found_word = String::from("");
                        } else {
                            // else we have found our first number => write found_number into value
                            values.push(found_number.parse().unwrap());
                            // reset found_number
                            found_number = String::from("");
                        }
                    }
                    // if found_number has a value, char is ")" and values.len() = 1
                    else if char == ')' && values.len() == 1 {

                        // if found_number.len() > 3, we need to continue looking => found_word =
                        // found_number = "" and values = Vec::with_capacity(2)
                        if found_number.len() < 4 {
                            values.push(found_number.parse().unwrap());
                            result += values[0] * values[1];
                        } 

                        values = Vec::with_capacity(2);
                        found_number = String::from("");
                        found_word = String::from("");

                        
                    }
                }
            }
        } else {
            found_word.push(char);

            if !MUL.starts_with(&found_word) && !DO.starts_with(&found_word) && !DONT.starts_with(&found_word) {
                found_word = String::from("");
            }
        }
    });

    println!("Result: {}", result);
    
    Ok(())
}

