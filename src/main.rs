use std::fs;
use std::error;

fn main() -> Result<(), Box<dyn error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;

    let mut count: u64 = 0;

    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect()).collect();

  for i in 0..matrix.len() {
      count += String::from_iter(matrix[i].clone()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
      count += String::from_iter(matrix[i].clone().into_iter().rev()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
      
  }



  for i in 0..matrix.len() {
    let mut row: Vec<char> = vec![];
    for j in 0..matrix[i].len() {
        row.push(matrix[j][i])
    } 
    count += String::from_iter(row.clone()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
    count += String::from_iter(row.clone().into_iter().rev()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
  }

let mut diagonal_matrix: Vec<Vec<char>> = vec![];



for i in 0..matrix.len() {
    let prefix = vec![' '; i];
    let suffix = vec![' '; matrix.len() - 1 - i];
    let result: Vec<char> = prefix.iter().chain(&matrix[i]).copied().chain(suffix).collect();
    diagonal_matrix.push(result)
}

let mut temp = vec![];
for i in 0..diagonal_matrix[0].len() {
    let mut row: Vec<char> = vec![];
    for j in 0..diagonal_matrix.len() {
        row.push(diagonal_matrix[j][i]);
    } 
    temp.push(row.clone());
    count += String::from_iter(row.clone()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
    count += String::from_iter(row.clone().into_iter().rev()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
  }

  diagonal_matrix = vec![];

  for i in 0..matrix.len() {
    let suffix = vec![' '; i];
    let prefix = vec![' '; matrix.len() - 1 - i];
    let result: Vec<char> = prefix.iter().chain(&matrix[i]).copied().chain(suffix).collect();
    diagonal_matrix.push(result)
}


  for i in 0..diagonal_matrix[0].len() {
    let mut row: Vec<char> = vec![];
    for j in 0..diagonal_matrix.len() {
        row.push(diagonal_matrix[j][i]);
    } 
    count += String::from_iter(row.clone()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
    count += String::from_iter(row.clone().into_iter().rev()).matches("XMAS").collect::<Vec<&str>>().len() as u64;
  }


  println!("count: {}", count);
    
    Ok(())
}

