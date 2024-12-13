use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let input_raw = fs::read_to_string("input.txt")?;
    let input: Vec<Vec<char>> = input_raw.lines().map(|line| line.chars().collect()).collect();
    // println!("{:?}", input);
    // iterate the map,
    // for each element, check the surrounding blocks, if they are different
    // from the one we are currently looking at, increase the perimiter count 
    // perimiter is saved in a HashMap, where el is the key and  usize is the value

    const DIRECTIONS: [Direction; 4] = [Direction::Right, Direction::Bottom, Direction::Left, Direction::Top];

    /*
    AAAA
    BBCD
    BBCC
    EEEC
    */

    let mut perimeter: HashMap<char, [usize; 2]> = HashMap::new();
    let mut area: HashMap<char, usize> = HashMap::new();


    for (y, row) in input.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            perimeter.entry(*el)
                .and_modify(|coord| coord[0] += 1)
                .or_insert([1,0]);
            for direction in DIRECTIONS {
                let [y_direction, x_direction] = direction.coordinates();

                let new_y = y as i64 + y_direction;
                let new_x = x as i64 + x_direction;

                if new_x >= 0 || new_y >= 0 {
                    input.get(new_y as usize)
                        .and_then(|line| line.get(new_x as usize))
                        .map(|next_el| {
                            if next_el != el {
                                perimeter.entry(*el)
                                .and_modify(|coord| coord[1] += 1)
                                .or_insert([0,1]);
                            }
                        })
                        .unwrap_or_else(|| {
                            perimeter.entry(*el)
                                .and_modify(|coord| coord[1] += 1)
                                .or_insert([0,1]);
                        });
                } else {
                    perimeter.entry(*el)
                                .and_modify(|coord| coord[1] += 1)
                                .or_insert([0,1]);
                }

                
                // println!("{}, {}", y, x);
            }
        }
    } 
    println!("{:#?}", perimeter);
    println!("{:?}", perimeter.iter().fold(0, |acc, (_key, value)| {
        let [area, perimeter] = value;
        return acc + area * perimeter
    }));
    
    Ok(())
}

#[derive(Debug)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

impl Direction {
    pub fn coordinates(&self) -> [i64; 2] {
        match self {
            Direction::Right => [0, 1],
            Direction::Bottom => [1, 0],
            Direction::Left => [0, -1],
            Direction::Top => [-1, 0],
        }
    }
}

/* struct Position {
    x: usize,
    y: usize
}

impl Position {
    pub fn new(y: i64, x:i64) -> Self {
        Self {
            x,
            y
        }
    }
}


// ask claude: should I always borrow
fn get_next_position(current_position: Position, direction: &Direction) -> Option<Direction> {
    match direction {
        Direction::Right => {
            let x
        }
    }
} */