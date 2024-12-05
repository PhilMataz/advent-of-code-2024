use std::fs;
use std::error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.split("\n\n").collect();

    let mut rules = HashMap::new();

    lines[0]
        .lines()
        .for_each(|line| {
            let elements: Vec<u64> = line.split("|").map(|rule_values| rule_values.parse().unwrap()).collect();
            let values = rules.entry(elements[1]).or_insert(vec![]);
            values.push(elements[0]); 
        });

    let updates: Vec<Vec<u64>> = lines[1]
        .lines()
        .map(|line| line.split(",")
            .map(|update| update.parse().unwrap())
            .collect())
        .filter(|line: &Vec<u64>| {
            for i in 0..line.len() - 1 {
                let empty: Vec<u64> = Vec::new();
                let smaller_page_numbers = rules.get(&line[i]).unwrap_or(&empty);
                let remaining_line_elements = &line[i+1..line.len()];
                for el in smaller_page_numbers {
                    if remaining_line_elements.contains(el) {
                        return false;
                    }
                }
                // println!("smaller numbers for {}: {:?} will be checked in: {:?}",line[i], smaller_page_numbers, remaining_line_elements);
            }
            true
        }).collect();

    let mut count = 0;

    for i in 0..updates.len() {
        let update = &updates[i];
        count += update[update.len() / 2];
    }

    println!("{}", count);
    Ok(())
}
