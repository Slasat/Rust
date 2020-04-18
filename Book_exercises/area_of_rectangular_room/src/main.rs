use std::io;

const FACTOR: f32 = 0.09290304;

fn main() {
    let mut in_str = String::new();
    println!("Choose the metrics: feet or meters");
    io::stdin()
        .read_line(&mut in_str)
        .expect("Can not read line");
    let metrics = String::from(in_str.trim());
    in_str = "".to_string();
    let convert_area: fn(u32) -> f32 = match metrics.as_str() {
        "meters" => to_feet_area,
        "feet" => to_meters_area,
        _ => {
            println!("Please write 'meters' or 'feet'");
            return;
        }
    };
    println!("What is the length of the room in {}?", metrics);
    io::stdin()
        .read_line(&mut in_str)
        .expect("Can not read line");
    let length: u32 = match in_str.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("What is the width of the room in {}?", metrics);
    in_str = "".to_string();
    io::stdin()
        .read_line(&mut in_str)
        .expect("Can not read line");
    let width: u32 = match in_str.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!(
        "You entered dimensions of {} {} by {} {}.",
        length, metrics, width, metrics
    );
    let area = calc_area(length, width);
    println!("The area is");
    println!("{} square {}", area, metrics);
    let converted_area: f32 = convert_area(area);
    match metrics.as_str() {
        "meters" => {
            println!("{:.2} square feet", converted_area);
            return;
        }
        "feet" => {
            println!("{:.2} square meters", converted_area);
            return;
        }
        _ => {
            println!("Can not determine metrics");
            return;
        }
    };
}

fn calc_area(length: u32, width: u32) -> u32 {
    length * width
}
fn to_meters_area(feet_area: u32) -> f32 {
    (feet_area.pow(2) as f32 * FACTOR).sqrt() as f32
}
fn to_feet_area(meters_area: u32) -> f32 {
    (meters_area.pow(2) as f32 / FACTOR).sqrt() as f32
}
