use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error + 'static>>{
    let input = fs::read_to_string("input.txt")?;

    let mut board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

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
    let mut loops = 0;
for i in 0..board.len() {
    for j in 0..board[i].len() {
        let mut position = get_starting_position(&board);
        let mut direction = Direction::Top;
        let mut path: HashMap<([i32;2], Direction), usize> = HashMap::new();
         if board[i][j] != '^' && board[i][j] != '#' {
            board[i][j] = '#';
            loop {

                if let Some(value) = path.get(&(position.clone(), direction.clone())) {
                    loops += 1;
                    break;
                }

                let next_position = move_position(&position, &get_movement_vector(&direction));
                let value_for_next_position = get_board_value(&board, next_position);

                if value_for_next_position == 'X' {
                    break;
                }
                if value_for_next_position == '#' {
                    direction = rotate(&direction);
                    

                    continue;
                }

                
                path.entry((position, direction.clone())).or_insert(1);

                position = next_position.clone();

            } 
            board[i][j] = '.';
        }
    }
}

println!("loops: {}", loops);

    Ok(())

}

fn print_board(board: &Vec<Vec<char>>) {
    for line in board {
        println!("{:?}", line);
    }
}

fn move_position(position: &[i32; 2], direction: &[i32; 2]) -> [i32; 2] {
    let new_y = position[0] + direction[0];
    let new_x = position[1] + direction[1];

    [new_y, new_x]
    
}

#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

fn rotate(direction: &Direction/* , path: &mut HashMap<Direction, usize> */) -> Direction {
    match direction {
        Direction::Right => {
            Direction::Bottom
        },
        Direction::Bottom => {
            Direction::Left
        },
        Direction::Left => {
            Direction::Top
        },
        Direction::Top => {
            Direction::Right
        },
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


