fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    let te = "te";
    let st = "st";
    let mut test = format!("{}{}", te, st);
    change(&mut test);
    println!("{}", test);
}

fn change(s:&mut String) {
    s.push_str(" Bye!");
}
