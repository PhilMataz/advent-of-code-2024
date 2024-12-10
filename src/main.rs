use std::fs;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    // create a struct for the antenna
    // struct has an x and a y coordinate for its position (Position will be a struct)
    // struct has a frequency
    // struct has a function that can receive another antenna as an argument, which calculates 
    // the antinode, if the frequencies match.
    // the calculation will be my_antenna.x + 2*(other_antenna.x - my_antenna.x)
    // the calculation will be my_antenna.y + 2*(other_antenna.y - my_antenna.y)
    // if the frequencies don't match, it will return None
    // otherwise it will return Some(Position)
    //
    // in main, create a map of the antennas
    // iterate over the map and create a HashMap that holds Antennas per
    // frequency
    // 
    // Then iterate over the HashMap and compute the antinodes
    // If they are valid (they are in the bounds of the map)
    // Create a HashMap of unique valid antinodes
    let raw_input = fs::read_to_string("input.txt")?;
    let map_size = raw_input.lines().count() as i64;

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut antinodes: HashMap<Position, usize> = HashMap::new();

    raw_input.lines().enumerate().for_each(|(y,line)| line.chars().enumerate().for_each(|(x, el)| {
        if el != '.' {
            let antennas_for_frequency = antennas.entry(el).or_insert(vec![]);
            antennas_for_frequency.push(Position::new(y as i64, x as i64));
        }
    }));

    for (_key, value) in antennas.into_iter() {
        for i in 0..value.len() - 1 {
            for j in (i+1)..value.len() {
                let mut positions: Vec<Position> = vec![];

                positions.append(&mut get_antinodes(map_size, &value[i], &value[j]));
                positions.append(&mut get_antinodes(map_size, &value[j], &value[i]));

                positions.into_iter().for_each(|position| {antinodes.entry(position).or_insert(1); });
                
                /* let frequency_for_i = value[i].distance(&value[j]);
                let frequency_for_j = value[j].distance(&value[i]);
                if is_valid_position(map_size, &frequency_for_i) {
                    antinodes.entry(frequency_for_i).or_insert(1);
                }
                if is_valid_position(map_size, &frequency_for_j) {
                    antinodes.entry(frequency_for_j).or_insert(1);
                } */
            }
        }
    }
    println!("Antinodes: {:#?}", antinodes);
    println!("Unique Locations: {:#?}", antinodes.len());

    Ok(())
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct Position {
    y: i64,
    x: i64,
}

impl Position {
    pub fn new(y: i64, x:i64) -> Self {
        Self {
            x,
            y
        }
    }
    pub fn distance(&self, position: &Position, repeater: i64) -> Position {
        let x = self.x + repeater * (position.x - self.x); 
        let y = self.y + repeater * (position.y - self.y); 

        Position::new(x,y)
    }
}

fn is_valid_position(map_size: i64, position: &Position) -> bool {
    position.x >= 0 && position.x < map_size && position.y >= 0 && position.y < map_size
}

fn get_antinodes(map_size: i64, position_a: &Position, position_b: &Position) -> Vec<Position> {
    let mut repeater: i64 = 1;
    let mut result: Vec<Position> = vec![];
    loop {  
        let position = position_a.distance(position_b, repeater);
        if !is_valid_position(map_size, &position) {
            break;
        }
        result.push(position);
        repeater += 1;
    }
    result
}
