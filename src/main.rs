use std::fmt;
use std::fs;
use std::error::Error;
use std::collections::HashMap;


fn main() -> Result<(), Box<dyn Error>> {
    let input_raw = fs::read_to_string("input.txt")?;
    let input: Vec<Vec<char>> = input_raw.lines().map(|line| line.chars().collect()).collect();

    let mut garden: HashMap<char, Vec<Region>> = HashMap::new();

    for (y, lines) in input.iter().enumerate() {
        for (x, kind) in lines.iter().enumerate() {
            let plot = Plot::new(*kind, y, x);
            let mut base = Region::new(plot.clone());

            
            if let Some(regions) = garden.get_mut(&kind) {
                regions.retain_mut(|region| {
                    if region.can_be_added(&plot) {
                        base.join(region);
                        return false;
                    }
                    true
                });
                regions.push(base);
            } else {
                garden.insert(*kind, vec![base]);
            }
        }
    }
    let total = garden.iter().fold(0, |acc, (_key, value)| {
        acc + value.iter().map(|region| region.get_price()).fold(0, |acc, curr| acc +curr)
    });
    println!("total: {}", total);
    Ok(())
}

#[derive(Debug)]
struct Region {
    kind: char,
    plots: Vec<Plot>
}

impl Region {
    pub fn new(plot: Plot) -> Self {
        Self {
            kind: plot.kind,
            plots: vec![plot]
        }
    }
    pub fn join(&mut self, region: &mut Region) -> &mut Self {
        if self.kind != region.kind {
            panic!("Regions need to be of same Kind.");
        }
        self.plots.append(&mut region.plots);

        self
    }
    
    pub fn can_be_added(&self, plot: &Plot) -> bool {
        self.plots.iter().any(|existing_plot| existing_plot.is_neighbour(&plot)) 
    }

    pub fn get_perimeter(&self) -> usize {
        let number_of_plots = self.plots.len();
        let mut result = number_of_plots * 4;
        for i in 0..number_of_plots {
            for j in 0..number_of_plots {
                if j == i {
                    continue;
                }
                if self.plots[i].is_neighbour(&self.plots[j]) {
                    result -= 1;
                }
            }
        }
        result
    }

    pub fn get_price(&self) -> usize {
        self.get_perimeter() * self.plots.len()
    }
}
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
struct Plot {
    kind: char,
    y: usize,
    x: usize,
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}|{})", self.y, self.x)
    }
}

impl Plot {
    pub fn new(kind: char, y: usize, x:usize) -> Self {
        Self {
            kind,
            y,
            x
        }
    }

    pub fn is_neighbour(&self, other: &Plot) -> bool {
        let delta_x = (self.y as i64 - other.y as i64).abs();
        let delta_y = (self.x as i64 - other.x as i64).abs();
        let is_y_adjacent = delta_x == 1 && delta_y == 0;
        let is_x_adjacent = delta_y == 1 && delta_x == 0;

        is_y_adjacent ^ is_x_adjacent
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_horizontal_neighbour() {
       let plot_a = Plot::new('A',0,0);
       let plot_b = Plot::new('A',0,1);

       assert!(plot_a.is_neighbour(&plot_b));
       assert!(plot_b.is_neighbour(&plot_a));
    }

    #[test]
    fn is_vertical_neighbour() {
       let plot_a = Plot::new('A',0,0);
       let plot_b = Plot::new('A',1,0);

       assert!(plot_a.is_neighbour(&plot_b));
       assert!(plot_b.is_neighbour(&plot_a));
    }

    #[test]
    fn is_diagonal() {
       let plot_a = Plot::new('A',0,0);
       let plot_b = Plot::new('A',1,1);

       assert!(!plot_a.is_neighbour(&plot_b));
       assert!(!plot_b.is_neighbour(&plot_a));
    }
    
    #[test]
    fn is_away() {
       let plot_a = Plot::new('A',0,0);
       let plot_b = Plot::new('A',2,0);

       assert!(!plot_a.is_neighbour(&plot_b));
       assert!(!plot_b.is_neighbour(&plot_a));
    }

    #[test]
    fn is_away_2() {
       let plot_a = Plot::new('A',1,2);
       let plot_b = Plot::new('A',3,3);

       assert!(!plot_a.is_neighbour(&plot_b));
       assert!(!plot_b.is_neighbour(&plot_a));
    }
}
/*
// ask claude: should I always borrow
fn get_next_position(current_position: Plot, direction: &Direction) -> Option<Direction> {
    match direction {
        Direction::Right => {
            let x
        }
    }
} */
