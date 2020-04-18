use std::f64::consts;
use std::io;

const SQUARE_FEET_IN_GALLON: u32 = 350;

const LSHAPED_FIGURE: &str = "  ________
  |       |   D
A |       |______
  |              |  C
  |______________|
        B
    ";

trait Area {
    fn calculate_area(&self) -> f32;
}

struct Rectangular {
    length: u32,
    width: u32,
}

struct Round {
    radius: u32,
}

struct Lshaped {
    side_a: u32,
    side_b: u32,
    side_c: u32,
    side_d: u32,
}

impl Area for Rectangular {
    fn calculate_area(&self) -> f32 {
        (self.length * self.width) as f32
    }
}

impl Rectangular {
    pub fn new() -> Option<Self> {
        let mut input = String::new();
        println!("Enter length:");
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line");
        let length: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        println!("Enter width:");
        input = "".to_string();
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line");
        let width: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        Some(Self {
            length: length,
            width: width,
        })
    }
}

impl Area for Round {
    fn calculate_area(&self) -> f32 {
        consts::PI as f32 * (self.radius * self.radius) as f32
    }
}
impl Round {
    pub fn new() -> Option<Self> {
        let mut input = String::new();
        println!("Enter radius:");
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line");
        let radius: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        Some(Self { radius: radius })
    }
}

impl Area for Lshaped {
    fn calculate_area(&self) -> f32 {
        let side_a_top: u32 = self.side_a - self.side_c;
        let side_b_top: u32 = self.side_b - self.side_d;
        (side_a_top * side_b_top + self.side_b * self.side_c) as f32
    }
}
impl Lshaped {
    pub fn new() -> Option<Self> {
        println!("{}", LSHAPED_FIGURE);
        let mut input = String::new();
        println!("Enter side A:");
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line");
        let side_a: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        input = "".to_string();
        println!("Enter side B:");
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line");
        let side_b: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        input = "".to_string();
        println!("Enter side C:");
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line");
        let side_c: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        input = "".to_string();
        println!("Enter side D:");
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line");
        let side_d: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                return None;
            }
        };
        if side_a < side_c || side_b < side_d {
            println!(
                "Side A should be greater than side C and side B should be greater than side D"
            );
            return None;
        }
        Some(Self {
            side_a: side_a,
            side_b: side_b,
            side_c: side_c,
            side_d: side_d,
        })
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter shape: Square(S)/Round(R)/L-shaped(L)");
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line");
    let _area_calculator: Box<dyn Area> = match input.trim() {
        "S" => Box::new(match Rectangular::new() {
            Some(obj) => obj,
            None => return,
        }),
        "Square" => Box::new(match Rectangular::new() {
            Some(obj) => obj,
            None => return,
        }),
        "R" => Box::new(match Round::new() {
            Some(obj) => obj,
            None => return,
        }),
        "Round" => Box::new(match Round::new() {
            Some(obj) => obj,
            None => return,
        }),
        "L" => Box::new(match Lshaped::new() {
            Some(obj) => obj,
            None => return,
        }),
        "L-shaped" => Box::new(match Lshaped::new() {
            Some(obj) => obj,
            None => return,
        }),
        _ => {
            println!("Enter S, R or L");
            return;
        }
    };
    let area: f32 = _area_calculator.calculate_area();
    let cans_needed: u32 = (area as f32 / SQUARE_FEET_IN_GALLON as f32).ceil() as u32;
    println!(
        "You will need to purchase {} gallons of paint to cover {} square feet",
        cans_needed, area
    );
}
