use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let disk_map = fs::read_to_string("input.txt")?;

    let disk_map_parsed: Vec<u32> = disk_map.trim().chars().map(|el| el.to_digit(10).unwrap()).collect();

    // println!("disk_map_parsed: {:?}", disk_map_parsed);

    let checksum = defragment(disk_map_parsed);

    println!("checksum: {}", checksum); 
    Ok(())
}

 fn defragment_fast(disk_map: &mut Vec<i32>) -> &mut Vec<i32>{
    let mut search = -1;
    let mut count = 0;

    // iterate over the whole length
    let mut disk_map_copy = disk_map.clone();
    for i in 0..disk_map.len() {

        let index = disk_map.len() -1 - i;
        let el = disk_map_copy[index].clone();
        if index == 0 || (search != -1 && el != search)  {
            match find_empty_space(disk_map, count, index) {
               Some(value) => {
                    for j in 0..count {
                       disk_map[j+value] = search;
                       disk_map[index + count - j] = -1;
                    }
               },
               None => (),
            }
            search = -1;
            count = 0;
        }
        if el != -1 {
            search = el;
            count += 1;
        }

    };
    disk_map
}

fn calculate_checksum(disk_map: &mut Vec<i32>) -> usize { 
    disk_map
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, curr)|{
            if *curr == -1 {
                return acc;
            }
            acc + index * *curr as usize
        })
}

fn find_empty_space(disk_map: &Vec<i32>, space: usize, limit: usize) -> Option<usize> {
    let mut count = 0;
    for i in 0..(limit + 1) {
       if disk_map[i] == -1 {
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

fn defragment(disk_map: Vec<u32>) -> usize {
    let mut result: Vec<i32> = vec![];
    disk_map.into_iter().enumerate().for_each(|(i, el)| {
        if i % 2 == 0 {
            let mut block = vec![(i/2) as i32; el as usize];
            result.append(&mut block);
        } else {
            let mut free_space = vec![-1; el as usize];
            result.append(&mut free_space);
        }
    });
    // println!("fragmented: {:?}", result);

    let defragment_result = defragment_fast(&mut result);

    // println!("defragment_result: {:?}", defragment_result);

    calculate_checksum(defragment_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = defragment(String::from("1"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_2() {
        let result = defragment(String::from("2333133121414131402"));
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_3() {
        let result = defragment(12345);
        assert_eq!(result, 132);
    }

    fn test_4() {
        let result = defragment(6586278992486738267411111);
        // 6586278992486738267411111
        // 000000.....11111111......22.......33333333.........444444444..5555........666666.......777........88......9999999....10.11.12
        // 000000XIIXIX881111111177722...999999933333333666666...444444444..5555........................................................
    }
}
