use std::io;

fn main() {
    proper_one();
}

fn proper_one() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Can not read line");
    name = name.trim().to_string();
    match name.as_str() {
        "Vlad" => println!("Hi, master"),
        _ => println!("Hello, {}, nice to meet you!", name)
    };
}

fn one_liner() {
    println!("What is your name?");
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}, nice to meet you!", name.trim()),
        Err(e) => println!("{:?}", e)
    };
}
