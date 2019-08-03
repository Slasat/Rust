fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    let te = "te";
    let st = "st";
    let test = format!("{}{}", te, st);
    take_ownership(&test);
    println!("{}", test);
}

fn take_ownership(s: &String){
    println!("{}", s);
}
