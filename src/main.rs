use std::fs;
use std::error;

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;

    let mut count: u64 = 0;

    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect()).collect();

        for i in 0..matrix.len() {
            println!("Line #{}: {:?}", i, matrix[i]);
          }
        



  for i in 1..matrix.len() - 1 {
    let mut row: Vec<char> = vec![];
    for j in 1..matrix[i].len() - 1 {
        let character = matrix[i][j];
        if character == 'A' {
            println!("Coordinates of A: ({},{})", i, j);
            let left_top = matrix[i-1][j-1];
            let right_bottom = matrix[i+1][j+1];
            println!("left top is: ({}, {}) with a value of: {}", i-1, j-1, left_top);
            println!("right bottom is: ({}, {}) with a value of: {}", i+1, j+1, right_bottom);
            if left_top == 'M' && right_bottom == 'S' || left_top == 'S' && right_bottom == 'M' {
                println!("potential match!");
                let left_bottom = matrix[i+1][j-1];
                let right_top = matrix[i-1][j+1];
                println!("left bottom is: ({}, {}) with a value of: {}", i+1, j-1, left_bottom);
                println!("right top is: ({}, {}) with a value of: {}", i-1, j+1, right_top);
                if left_bottom == 'M' && right_top == 'S' || left_bottom == 'S' && right_top == 'M' {
                    println!("match!");
                    count += 1;
                }
            }

            /* if ((dbg!(matrix[i-1][j-1]) == 'S' && dbg!(matrix[i+1][j+1]) == 'M') || (dbg!(matrix[i-1][j-1])== 'M' && dbg!(matrix[i+1][j+1]) == 'S')) && ((dbg!(matrix[i+1][j-1]) == 'S' && dbg!(matrix[i-1][j+1]) == 'M') || (dbg!(matrix[i-1][j+1]) == 'M' && dbg!(matrix[i+1][j-1]) == 'S')) {
                println!("found A @: {},{}", i, j);
                count += 1;
            } */
        }
        row.push(matrix[j][i])
    } 
    // println!("Line #{}: {:?}", i-1, row);
  }


  println!("count: {}", count);
    
    Ok(())
}

