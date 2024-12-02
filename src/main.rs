use std::fs;
use std::error;

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    
    let data: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line
                .trim()
                .split_whitespace()
                .map(|el| {
                    el.parse().unwrap()
                }).collect()
        )
        .filter(|line: &Vec<u32>| {
            let mut is_valid = true;
            let mut is_increasing = true;
            let mut is_decreasing = true;
            for i in 1..line.len() {
                let diff = (line[i-1] as i32 - line[i] as i32).abs();
                if diff < 1 || diff > 3 {
                    is_valid = false; 
                    break;
                }
                if line[i-1] < line[i] {
                    is_decreasing=false;
                }
                if line[i-1] > line [i] {
                    is_increasing = false;
                }
            }
            is_valid && (is_increasing || is_decreasing)
        }).collect();
    println!("Valid datasets: {:#?}", data.len());
    Ok(())
}

