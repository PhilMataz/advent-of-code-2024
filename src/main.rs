use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error + 'static>>{
    let input = fs::read_to_string("input.txt")?;

    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

/*     println!("board: {:?}", board);

    let mut direction = Direction::Left; */

/*     let position = [5,5];

    let next_position = move_position(&position, get_movement_vector(direction));

    let result = board.get(0);
    println!("result {:?}", result);

    let some_board_value = get_board_value(board, [0,0]);

    println!("Board Value: {}", some_board_value); */

    fn get_starting_position(board: &Vec<Vec<char>>) -> [i32; 2] {
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == '^' {
                    return [i as i32, j as i32];
                }
            }
        }
        [-1, -1]
    }

    let mut position = get_starting_position(&board);
    let mut direction = Direction::Top;
    let mut moves = HashMap::new();
    moves.insert(position, 1);

loop{
        println!("Getting next position");
        let next_position = move_position(&position, &get_movement_vector(&direction));
        println!("Next possible position is: {:?}", next_position);
        println!("Getting value for position");
        let value_for_next_position = get_board_value(&board, next_position);
        println!("Value for next position: {}", value_for_next_position);

        if value_for_next_position == 'X' {
            println!("Out of bounds. Game finished!");
            break;
        }
        if value_for_next_position == '#' {
            println!("Hit obsticle, Rotating!");
            direction = rotate(&direction);
            continue;
        }

        println!("Moving to next position");

        
        position = next_position.clone();
        moves.insert(position, 1);
}    

println!("Number of Moves: {}", moves.len());

    Ok(())

}

fn move_position(position: &[i32; 2], direction: &[i32; 2]) -> [i32; 2] {
    let new_y = position[0] + direction[0];
    let new_x = position[1] + direction[1];

    [new_y, new_x]
    
}

#[derive(Debug)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

fn rotate(direction: &Direction) -> Direction {
    match direction {
        Direction::Right => Direction::Bottom,
        Direction::Bottom => Direction::Left,
        Direction::Left => Direction::Top,
        Direction::Top => Direction::Right,
    }
}

fn get_movement_vector(direction: &Direction) -> [i32;2] {
    match direction {
        Direction::Right => [0,1],
        Direction::Bottom => [1,0],
        Direction::Left => [0,-1],
        Direction::Top => [-1,0],
    }
}

fn get_board_value(board: &Vec<Vec<char>>, coordinates: [i32;2]) -> char {   
    let line = match board.get(coordinates[0] as usize) {
        None => &Vec::<char>::new(),
        Some(line) => line,
    };

    match line.get(coordinates[1] as usize) {
        None => 'X',
        Some(value) => value.clone()
    }  
}


