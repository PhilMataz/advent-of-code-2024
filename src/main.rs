use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;

    let result: u64 = input.lines().map(|line| {
        let split_line: Vec<&str> = line.split(":").collect();
        let expected_sum: u64 = split_line[0].parse().unwrap();
        let numbers: Vec<u64> = split_line[1].split_whitespace().map(|el| el.parse().unwrap()).collect();

        return (expected_sum, numbers);

    }).filter(|el| {
        let expected_sum: u64 = el.0;
        let numbers: Vec<u64> = (el.1).clone();

        let binaries = get_binaries(numbers.len() - 1);

        // println!("expected_sum: {}", expected_sum);
        for binary in &binaries {
            let sum = numbers.clone().into_iter().enumerate().fold(0, |acc, (i, num)| {
                if i == 0 {
                    return num;
                }

                if binary[i-1] == '1' {
                    return acc + num;
                } else if binary[i-1] == '0' {
                    return acc * num;
                } else {
                    let power: usize = num.clone().to_string().len() - 1;
                    let base: u64 = 10;
                    
                    return acc * base.pow(power as u32) + num;
                }
            });
            if sum == expected_sum {
                return true;
            }
        }

        return false 
    }).fold(0, |acc, curr| acc + curr.0); 

    println!("filtered: {:?}", result);
    //println!("result: {:?}", computed_binary_array(&9, 0));

    // [12,12] => [1] 1
    // [12,12,12] => [1,1] 3
    // [12,12,12,12] => [1,1,1] 7 
    // [12,12,12,12,12] => [1,1,1,1] 15


    Ok(())
}

fn get_binaries(num: usize) -> Vec<Vec<char>> {
    let mut binaries: Vec<Vec<char>> = vec![];
    let mut index: usize = 0;
    loop {
        // println!("loop");
        let binary = computed_binary_array(&index, num);

        if binary.len() > num {
            break;
        }
        index += 1;
        binaries.push(binary);
        // println!("binaries: {:?}", binaries);
    } 
    binaries
}

fn computed_binary_array(num: &usize, pad: usize) -> Vec<char> {
    let mut result = 0;
    let mut input = *num;
    let mut power = 1;
    
    while input > 0 {
        result += (input %2 ) * power;
        input /= 2;
        power *= 10;
    }
    format!("{:0>width$}", result.to_string(), width=pad).chars().collect()

    // 20 
    // result += 0 * 1
    // input = 10;
    // power *= 10;
    // 10
    // result += 0 * 10
    // input = 5;
    // power = 100;
    // 5 
    // result += 1 * 100
}
