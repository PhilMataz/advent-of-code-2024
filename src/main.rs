use std::fs;

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
}

