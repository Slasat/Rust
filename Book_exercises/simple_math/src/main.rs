use std::io;

fn main() {
    println!("What is the first number?");
    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Can not read line!");
    let first_number: u32 = match first.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("What is the second number?");
    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("Can not read line!");
    let second_number: u32 = match second.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!(
        "{} + {} = {}\n{} - {} = {}\n{} * {} = {}\n{} / {} = {}\n",
        first_number,
        second_number,
        my_sum(first_number, second_number),
        first_number,
        second_number,
        my_dif(first_number, second_number),
        first_number,
        second_number,
        my_mult(first_number, second_number),
        first_number,
        second_number,
        my_div(first_number, second_number)
    );
}

fn my_sum(first: u32, second: u32) -> u32 {
    first + second
}

fn my_dif(first: u32, second: u32) -> u64 {
    (first - second) as u64
}

fn my_mult(first: u32, second: u32) -> u32 {
    first * second
}

fn my_div(first: u32, second: u32) -> u64 {
    (first / second) as u64
}
