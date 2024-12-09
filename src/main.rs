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
    
    let checksum = defragment(&mut result)
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, curr)|{
            if *curr == String::from(".") {
                return acc;
            }
            acc + index * curr.parse::<usize>().unwrap() as usize
        });


    println!("checksum: {}", checksum);
    Ok(())
}

fn defragment(disk_map: &mut Vec<String>) -> &mut Vec<String> {
    let mut i = 0;
    let mut j = disk_map.len() - 1;

    while i < j {
        while disk_map[i] != String::from(".") {
            i += 1;
            continue;
        }
        while disk_map[j] == String::from(".") {
            j -= 1;
            continue;
        }
        if i < j {
            disk_map[i] = disk_map[j].clone();
            disk_map[j] = String::from(".");
        } 
        i += 1;
        j -= 1;
    }
    disk_map

}
