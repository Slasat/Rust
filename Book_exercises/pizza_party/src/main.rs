use std::io;

const NUMBER_OF_SLICES: u32 = 8;

fn main() {
    second_version();
}

fn second_version() {
    let mut input = String::new();
    println!("How many people?");
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line");
    let people_count: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    input = "".to_string();
    println!("How many slices each person wants?");
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line");
    let slices_per_person_count: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let all_slices: u32 = people_count * slices_per_person_count;
    let pizzas_needed: u32 = ((all_slices / NUMBER_OF_SLICES) as f32).ceil() as u32;
    match pizzas_needed {
        1 => println!("You need to buy {} pizza!", pizzas_needed),
        _ => println!("You need to buy {} pizzas!", pizzas_needed),
    };
}

fn first_version() {
    let mut input = String::new();
    println!("How many people?");
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line");
    let people_count: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    input = "".to_string();
    println!("How many pizzas do you have?");
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line");
    let pizza_count: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    println!("{} people with {} pizzas", people_count, pizza_count);
    let slice_per_person = pizza_count * NUMBER_OF_SLICES / people_count;
    let slices_left = pizza_count * NUMBER_OF_SLICES % people_count;
    match slice_per_person {
        0 => println!("Can not share this amount of pieces evenly"),
        1 => println!("Each person gets {} piece of pizza", slice_per_person),
        _ => println!("Each person gets {} pieces of pizza", slice_per_person),
    };
    match slices_left {
        1 => println!("There is {} leftover piece", slices_left),
        _ => println!("There are {} leftover pieces", slices_left),
    };
}
