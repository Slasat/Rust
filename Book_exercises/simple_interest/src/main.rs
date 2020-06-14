use num::bigint::BigInt;
use num::rational::Ratio;
use num::traits::ToPrimitive;
use std::io;

fn get_float_value() -> Option<f64> {
    let mut in_line = String::new();
    io::stdin()
        .read_line(&mut in_line)
        .expect("Failed to read line");
    let number: f64 = match in_line.trim().parse() {
        Ok(num) => num,
        Err(_) => return None,
    };
    Some(number)
}

fn get_int_value() -> Option<u32> {
    let mut in_line = String::new();
    io::stdin()
        .read_line(&mut in_line)
        .expect("Failed to read line");
    let number: u32 = match in_line.trim().parse() {
        Ok(num) => num,
        Err(_) => return None,
    };
    Some(number)
}

fn calculate_interest(init: f64, rate: f64, years: u32) -> Option<f64> {
    let ratio_init: Ratio<BigInt> = match Ratio::from_float(init) {
        Some(ratio) => ratio,
        None => {
            println!("Can not convert currency to rational number, thus can not convert it!");
            return None;
        }
    };
    let ratio_rate: Ratio<BigInt> = match Ratio::from_float(rate / 100.0) {
        Some(ratio) => ratio,
        None => {
            println!("Can not convert currency to rational number, thus can not convert it!");
            return None;
        }
    };
    let constant: Ratio<BigInt> = Ratio::from_float(1.0).unwrap();
    let ratio_years: Ratio<BigInt> = Ratio::from_float(years as f64).unwrap();
    let ratio_result: Ratio<BigInt> = &ratio_init * (&constant + &ratio_years * &ratio_rate);
    let result: f64 =
        ratio_result.numer().to_f64().unwrap() / ratio_result.denom().to_f64().unwrap();
    Some(result)
}

fn main() {
    println!("Init");
    let init: f64 = match get_float_value() {
        Some(res) => res,
        None => panic!("Can not get init value!"),
    };
    println!("Rate");
    let rate: f64 = match get_float_value() {
        Some(res) => res,
        None => panic!("Can not get init value!"),
    };
    println!("Years");
    let years: u32 = match get_int_value() {
        Some(res) => res,
        None => panic!("Can not get init value!"),
    };
    let mut i: u32 = 1;
    while i <= years {
        let result = match calculate_interest(init, rate, i) {
            Some(res) => res,
            None => panic!("Can not calculate result"),
        };
        println!(
            "After {} years, with rate {}%, the investment will be worth ${}",
            i, rate, result
        );
        i += 1;
    }
}
