use chrono::{DateTime, Datelike, Utc};
use std::io;

fn main() {
    println!("What is your current age?");
    let mut in_current_age = String::new();
    io::stdin()
        .read_line(&mut in_current_age)
        .expect("Can not read line!");
    println!("At what  age would you like to retire?");
    let current_age: u32 = match in_current_age.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let mut in_retirement_age = String::new();
    io::stdin()
        .read_line(&mut in_retirement_age)
        .expect("Can not read line!");
    let retirement_age: u32 = match in_retirement_age.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    if retirement_age < current_age {
        println!("You already can retire");
        return;
    }
    let years_left: u32 = retirement_age - current_age;
    let utc: DateTime<Utc> = Utc::now();
    println!("You have {} years left until you can retire.", years_left);
    println!("{}", utc.date().year() + years_left as i32);
}
