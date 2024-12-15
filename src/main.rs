use std::fmt;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_raw = fs::read_to_string("input.txt")?;

    

    let mut input = vec![];
    let mut temp = vec![];

    input_raw.lines().for_each(|line| {
        if line == "" {
            input.push(temp.clone());
            temp.clear();
        } else {
            let test: &str = line.split(":").collect::<Vec<&str>>()[1].trim();

            // println!("{:?}", test.split(&['=', '+']).collect::<Vec<&str>>());
            let tempitemp: Vec<usize> = test.split(",").map(|el| {
                let tempi = el.trim().split(&['=', '+']).collect::<Vec<&str>>();
                tempi[1].parse::<usize>().unwrap()
            }).collect();


           /*  println!("{:?}", tempitemp);
            println!("---"); */
            // println!("{:?}", ;
            temp.push([tempitemp[0], tempitemp[1]]);
        }
    });

    input.push(temp.clone());

   input.iter().for_each(|el| {
        println!("{:?}", el);
    }); 

    let tokens = input.iter().fold(0, |acc, curr | acc + get_tokens(curr[0], curr[1], curr[2])); 

    println!("tokens: {}", tokens);

    Ok(())

}

fn get_tokens(button_a: [usize;2], button_b: [usize;2], price: [usize;2]) -> usize {
    let [x1, x2] = button_a;
    let [y1, y2] = button_b;
    let [c1, c2] = price;

    // println!("c1: {}, c2: {}", c1, c2);

    let equation_1 = Equation::new(x1 as i64, y1 as i64, (c1 + 10000000000000) as i64);
    let equation_2 = Equation::new(x2 as i64, y2 as i64, (c2 + 10000000000000) as i64);

     println!("{}", equation_1);
    println!("{}", equation_2);

    let [a, b] = solve_linear_equations(&equation_1, &equation_2);

    if a < 0 || b < 0    /* || a > 100 || b > 100  */ || !equation_1.control(a, b) || !equation_2.control(a, b){
        return 0;
    } 

    println!("---");
    println!("{}", equation_1);
    println!("{}", equation_2);

    /*     if !(equation_1.control(a, b)) {
        /* println!("A is passing test: {}", equation_1.control(a, b));
        println!("B is passing test: {}", equation_2.control(a, b)); */
        println!("equation_1: {}", equation_1);
        println!("equation_2: {}", equation_2);
        println!("A: {}", a);
        println!("B: {}", b);
    } */


/*     println!("A is passing test: {}", equation_1.control(a, b));
    println!("B is passing test: {}", equation_2.control(a, b)); */
    // println!("B is passing test: {}", b); 


    (a as usize) * 3 + (b as usize)
}

fn solve_linear_equations(equation_1: &Equation, equation_2: &Equation) -> [i64;2] {
    let equation_1_1 = equation_1.multiply_by(equation_2.x);
    let equation_2_1 = equation_2.multiply_by(equation_1.x);

    let subtracted = equation_1_1.subtract(&equation_2_1);
    
    let y = subtracted.solve_by_y();
    
    let equation_1_y_substituted = equation_1.substitute_y(y);
    
    let x = equation_1_y_substituted.solve_by_x();

    [x, y]
}


struct Equation {
    x: i64,
    y: i64, 
    c: i64
}

impl Equation {
    pub fn new(x: i64, y: i64, c: i64) -> Self {
        Self {
            x,
            y,
            c
        }
    }
    
    pub fn multiply_by(&self, multiplier: i64) -> Equation {
        Self::new(self.x * multiplier, self.y * multiplier, self.c * multiplier)
    }

    pub fn subtract(&self, equation: &Equation) -> Equation {
        let x = self.x - equation.x;
        let y = self.y - equation.y;
        let c = self.c - equation.c;

        Self::new(x, y, c)
        
    }

    pub fn solve_by_y(&self) -> i64 {
        if self.x != 0 {
            panic!("X needs to be 0 if you want to solve by y");    
        }

        self.c / self.y
    }

    pub fn solve_by_x(&self) -> i64 {
        if self.y != 0 {
            panic!("Y needs to be 0 if you want to solve by x");    
        }

        self.c / self.x
    }

    pub fn substitute_y(&self, y: i64) -> Equation {
        let c = self.c - y * self.y;

        Self::new(self.x, 0, c)
    }

    pub fn control(&self, x: i64, y: i64) -> bool {
        (self.x * x + self.y * y) == self.c
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x + {}y = {}", self.x, self.y, self.c)
    }
}


