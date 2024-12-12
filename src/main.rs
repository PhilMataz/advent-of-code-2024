use std::collections::HashMap;

fn main() {
    let stones: Vec<usize> = vec![5,62914,65,972,0,805922,6521,1639064];

    let mut map: HashMap<usize, usize> = HashMap::new();

    for el in &stones {
        map.insert(*el, 1);
    }
    for _ in 0..75 {
        let elements: Vec<(usize, usize)> = map.drain().collect();
    
        for (key, value) in elements {
            if key == 0 {
                let count = map.entry(1).or_insert(0);
                *count += value;
            } else if key.to_string().len() % 2 == 0 {
                let string_el = key.to_string();
                let mid = string_el.len() / 2;
                let count_left = map.entry(string_el[..mid].parse().unwrap()).or_insert(0);
                *count_left += value;
                let count_right = map.entry(string_el[mid..].parse().unwrap()).or_insert(0);
                *count_right += value;
            } else { 
                let count = map.entry(key * 2024).or_insert(0);
                *count += value;
            }
        }
    }

    println!("{:?}", map.iter().fold(0, |acc, (_key, value)| acc + value));

}
