use std::io;

fn main() {
    println!("What is the quoute?");
    let mut quote = String::new();
    io::stdin().read_line(&mut quote).expect("Can not read line!");
    println!("Who said it?");
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Can not read line!");
    println!("{}", author.trim().to_owned() + " says, \"" + quote.trim() + "\"");
}
