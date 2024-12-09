use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let disk_map =  fs::read_to_string("input.txt")?;

    let mut result: Vec<String> = vec![];
    disk_map.chars().enumerate().for_each(|(i, el)| {
        if i % 2 == 0 {
            let mut block = vec![(i/2).to_string();
                el   
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap()];
            result.append(&mut block);
        } else {
            let mut free_space = vec![String::from("."); el.to_digit(10).unwrap().try_into().unwrap()];
            result.append(&mut free_space);
        }
    });
    
    let mut i = 0;
    let mut j = result.len() - 1;

    while i < j {
        while result[i] != String::from(".") {
            i += 1;
            continue;
        }
        while result[j] == String::from(".") {
            j -= 1;
            continue;
        }
        println!("result for i({i}): {}", result[i]);
        println!("result for j({j}): {}", result[j]);
        if i == 29 {
            println!("{:?}", result);
        }
         if i < j {
            result[i] = result[j].clone();
            result[j] = String::from(".");
        } 
        i += 1;
        j -= 1;
    }

    let checksum = result
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, curr)|{
            if curr == String::from(".") {
                return acc;
            }
            acc + index * curr.parse::<usize>().unwrap() as usize
        });

    println!("checksum: {}", checksum);
    Ok(())
}


