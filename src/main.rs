// use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let disk_map = "2333133121414131402"; //  fs::read_to_string("input.txt")?;

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

    println!("result: {:?}", result);

    let _defragmented_disk_map = defragment_fast(&mut result);
    
    // let checksum = calculate_checksum(defragment(&mut result));

    // println!("checksum: {}", checksum);
    Ok(())
}

 fn defragment_fast(disk_map: &mut Vec<String>) {
    // find the block from the right and it's length
    // find space from the left and to the left of the block
    // if there is no space, continue with the next block
    // if there is, place block there 
    // free up block space
    
    // find first block
    // initialize search with "."
    // initialize count with 0
    // get the first el
    // if it is not .
    // set search to it and count to 1
    // continue searching, if it is equal search, increment count
    // otherwise print the word, reset count and search
    let mut search = String::from(".");
    let mut count = 0;

    let disk_map_length = disk_map.len();
    disk_map.into_iter().rev().enumerate().for_each(|(index,el)| {
        // the next lines don't catch the first numbers, but this shouldn't be necessary as 
        // they wont need to be rearranged
        if search != String::from(".") && ( *el != search || index == 0 ) {
            
            match find_empty_space(disk_map, count, disk_map.len() - 1) {
               Some(value) => {
                    for i in 0..count {
                       // disk_map[i+value] = search.clone();
                       // disk_map[i+disk_map_length - index] = String::from(".");
                    }
               },
               None => (),
            }
            
            println!("{} appears {}x and stops at index: {}", search, count, disk_map.len() - index);
            // println!("Empty Space at index: {:?}", empty_space_index);
            search = String::from(".");
            count = 0;
        }
        if *el != String::from(".") {
            search = el.to_string();
            count += 1;
        }
    });
}


fn _defragment(disk_map: &mut Vec<String>) -> &mut Vec<String> {
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

fn _calculate_checksum(disk_map: &mut Vec<String>) -> usize { 
    disk_map
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, curr)|{
            if *curr == String::from(".") {
                return acc;
            }
            acc + index * curr.parse::<usize>().unwrap() as usize
        })
}

fn find_empty_space(disk_map: &mut Vec<String>, space: usize, limit: usize) -> Option<usize> {
    let mut count = 0;
    for i in 0..(limit + 1) {
       if disk_map[i] == "." {
           count += 1;
           if count == space {
               return Some(i - space + 1);
           }
       } else {
           count = 0;
       }
    }
    None    
}

