use std::fs;
use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    
    let input_raw = fs::read_to_string("input.txt")?;
    
    let rows = input_raw.lines().count();

    let mut trailhead_indexes: Vec<usize> = vec![];

    let input: Vec<usize> = input_raw.lines().enumerate().fold(vec![],|mut acc,(i, curr)| {
        let line: Vec<usize> = curr.chars().enumerate().map(|(j,el)| {
            let current_el = el.to_digit(10).unwrap() as usize;
            if current_el == 0 {
                trailhead_indexes.push(j + i * rows);
            }
            current_el
        }).collect();
        acc.extend(line);
        acc
    });

    
    let total_score: usize = trailhead_indexes.into_iter().map(|el| {
        let mut coordinates = vec![el];

        get_next_coordinates(&mut coordinates, el, &input, rows);

        let unique_coordinates: HashSet<usize> = coordinates.clone().into_iter().collect();

        let trailhead_score = unique_coordinates.into_iter().map(|el| input[el]).fold(0, |acc, curr| {
            if curr == 9 {
                return acc + 1
            }
            acc
        });


        trailhead_score
    }).fold(0, |acc, curr| acc+curr);

    println!("trailheads: {:?}", total_score);

    fn get_next_coordinates(coordinates: &mut Vec<usize>, current_index: usize, input: &Vec<usize>, size: usize) {
        let directions: Vec<Direction> = 
            vec![Direction::Right, Direction::Bottom, Direction::Left, Direction::Top];

        for direction in directions {
            if let Some(index) = get_next_index(current_index, &direction, size, size) {
                if input[current_index] + 1 == input[index] {
                    coordinates.push(index);
                    get_next_coordinates(coordinates, index, input, size);
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left
}

fn get_next_index(current_index: usize, direction: &Direction, rows: usize, cols: usize) -> Option<usize> {
    match direction {
        Direction::Right => {
            let result = current_index + 1;
            if result % cols == 0 {
                return None;
            }
            Some(result)
        },
        Direction::Bottom => {
            let result = current_index + rows;
            if current_index + rows >= rows * rows {
                return None;
            }
            Some(result as usize)
        },
        Direction::Left => {
            if current_index % cols == 0 {
                return None;
            }
            Some(current_index - 1)
        },
        Direction::Top => {
            let result: i32 = current_index as i32 - rows as i32;
            if result < 0 {
                return None;
            }
            Some(result as usize)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_left() {
        let index = 0;
        let rows = 4;

        let right = get_next_index(index, Direction::Right, rows, rows);
        let bottom = get_next_index(index, Direction::Bottom, rows, rows);
        let left = get_next_index(index, Direction::Left, rows, rows);
        let top = get_next_index(index, Direction::Top, rows, rows);

        assert_eq!(right, Some(1));
        assert_eq!(bottom, Some(4));
        assert_eq!(left, None);
        assert_eq!(top, None);
    }

    #[test]
    fn top_right() {
        let index = 3;
        let rows = 4;

        let right = get_next_index(index, Direction::Right, rows, rows);
        let bottom = get_next_index(index, Direction::Bottom, rows, rows);
        let left = get_next_index(index, Direction::Left, rows, rows);
        let top = get_next_index(index, Direction::Top, rows, rows);

        assert_eq!(right, None);
        assert_eq!(bottom, Some(7));
        assert_eq!(left, Some(2));
        assert_eq!(top, None);
    }

    #[test]
    fn bottom_right() {
        let index = 15;
        let rows = 4;

        let right = get_next_index(index, Direction::Right, rows, rows);
        let bottom = get_next_index(index, Direction::Bottom, rows, rows);
        let left = get_next_index(index, Direction::Left, rows, rows);
        let top = get_next_index(index, Direction::Top, rows, rows);

        assert_eq!(right, None);
        assert_eq!(bottom, None);
        assert_eq!(left, Some(14));
        assert_eq!(top, Some(11));
    }

    #[test]
    fn bottom_left() {
        let index = 12;
        let rows = 4;

        let right = get_next_index(index, Direction::Right, rows, rows);
        let bottom = get_next_index(index, Direction::Bottom, rows, rows);
        let left = get_next_index(index, Direction::Left, rows, rows);
        let top = get_next_index(index, Direction::Top, rows, rows);

        assert_eq!(right, Some(13));
        assert_eq!(bottom, None);
        assert_eq!(left, None);
        assert_eq!(top, Some(8));
    }

    #[test]
    fn top_center() {
        let index = 2;
        let rows = 4;

        let right = get_next_index(index, Direction::Right, rows, rows);
        let bottom = get_next_index(index, Direction::Bottom, rows, rows);
        let left = get_next_index(index, Direction::Left, rows, rows);
        let top = get_next_index(index, Direction::Top, rows, rows);

        assert_eq!(right, Some(3));
        assert_eq!(bottom, Some(6));
        assert_eq!(left, Some(1));
        assert_eq!(top, None);
    }

    #[test]
    fn bottom_center() {
        let index = 13;
        let rows = 4;

        let right = get_next_index(index, Direction::Right, rows, rows);
        let bottom = get_next_index(index, Direction::Bottom, rows, rows);
        let left = get_next_index(index, Direction::Left, rows, rows);
        let top = get_next_index(index, Direction::Top, rows, rows);

        assert_eq!(right, Some(14));
        assert_eq!(bottom, None);
        assert_eq!(left, Some(12));
        assert_eq!(top, Some(9));
    }

    #[test]
    fn middle() {
        let index = 6;
        let rows = 4;

        let right = get_next_index(index, Direction::Right, rows, rows);
        let bottom = get_next_index(index, Direction::Bottom, rows, rows);
        let left = get_next_index(index, Direction::Left, rows, rows);
        let top = get_next_index(index, Direction::Top, rows, rows);

        assert_eq!(right, Some(7));
        assert_eq!(bottom, Some(10));
        assert_eq!(left, Some(5));
        assert_eq!(top, Some(2));
    }
}