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

    let mut updates: Vec<Vec<u64>> = lines[1]
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
                        // return true here and false in the bottom to filter invalids
                        return true;
                    }
                }
                // println!("smaller numbers for {}: {:?} will be checked in: {:?}",line[i], smaller_page_numbers, remaining_line_elements);
            }
            false
        }).collect();


    // start with is_sorted = false;
    // set is_sorted to true at the beginning of the loop
    // start a nested loop
    // get the rules for the page of the outer loop
    // compare the pages of the inner rules to the loop
    // if find page inside rule, swap i and j
    // set is_sorted to false
    // if loop runs though without swapping, its sorted.

    for invalid_pages in &mut updates {
        let mut is_sorted = false;
        'label: while !is_sorted {
            is_sorted = true;
            for i in 0..invalid_pages.len() - 1 {
                let page = invalid_pages[i];
                let empty: Vec<u64> = Vec::new();
                let rules_for_page = rules.get(&page).unwrap_or(&empty);
                for j in (0+i)..invalid_pages.len() {
                    if rules_for_page.contains(&invalid_pages[j]) {
                        invalid_pages.swap(i,j);
                        is_sorted = false;
                        continue 'label;
                    }
                }
            }
        }
    }


    let mut count = 0;

     for i in 0..updates.len() {
        let update = &updates[i];
        count += update[update.len() / 2];
    }

    println!("{}", count); 
    Ok(())
}
