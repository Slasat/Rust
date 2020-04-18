use std::io;

fn main() {
    let mut input = String::new();
    println!("What is the input");
    loop{
        io::stdin().read_line(&mut input).expect("Can not read line!");
        if input.trim() == "" {
            println!("Please, enter something!");
        }else{
            break
        }
    }
    println!("{} has {} characters.", input.trim(), input.trim().len());
}
