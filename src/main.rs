use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Error reading file!");

    
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];

    for line in contents.lines() {
        let mut remainder = line.trim().split("   ");

        // remainder.for_each(|el| println!("The element: {}", el));
        first_list.push(remainder.next().expect("Error while iterating line!").parse().expect("Error while parsing"));
        second_list.push(remainder.next().expect("Error while iterating line!").parse().expect("Error while parsing"));
    }
    first_list.sort();
    second_list.sort();


    
    let result: i32 = (0..first_list.len()).map(|x| (second_list[x] - first_list[x]).abs()).reduce(|acc, e| acc + e).unwrap();
    println!("{:#?}", result);

    let mut second_list_count: HashMap<i32, i32> = HashMap::new();

    second_list.into_iter().for_each(|el| {
        let count = second_list_count.entry(el).or_insert(0);
        *count += 1;
    });



     let result: i32 = first_list.into_iter().fold(0, |acc, el| {
        acc + second_list_count.get(&el).unwrap_or(&0) * el
    });
   
    println!("{:#?}", result);
    
}

