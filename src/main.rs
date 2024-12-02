use std::fs;
use std::error;

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    
    let mut data_with_deltas: Vec<Vec<i32>> = vec![];
    
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
            let mut pass = false;

            for i in 0..line.len() {
                let mut copy = line.clone();
                copy.remove(i);
                if is_safe(&copy) {
                    pass = true;
                    break;
                }
            }
            println!("line: {:#?}", line);
            println!("pass: {}", pass);

            pass
        }).collect();
    println!("Valid datasets: {:#?}", data.len());

    fn is_safe(line: &Vec<u32>) -> bool {
        let mut is_valid = true;
        let initial_trend = line[0] < line[1]; 
        let mut has_trend_changed = false;
        
        let mut i = 0;
        while i  < (line.len() - 1) && is_valid && !has_trend_changed {
            let diff = (line[i+1] as i32 - line[i] as i32).abs();
            if diff < 1 || diff > 3 {
                is_valid = false; 
            } 
            let current_tend = line[i] < line[i+1];
            if current_tend != initial_trend {
                has_trend_changed = true;
            }

            i += 1;
        }
        is_valid && !has_trend_changed
    }

    Ok(())
}

