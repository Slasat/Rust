use num::bigint::{BigInt, ToBigInt};
use num::rational::Ratio;
use num::traits::ToPrimitive;
use std::io;

struct Exchanger {
    amount_from: Ratio<BigInt>,
    rate: Ratio<BigInt>,
}

impl Exchanger {
    pub fn new(amount_from: f64, rate: f64) -> Option<Self> {
        let ratio_from: Ratio<BigInt> = match Ratio::from_float(amount_from) {
            Some(ratio) => ratio,
            None => {
                println!("Can not convert currency to rational number, thus can not convert it!");
                return None;
            }
        };
        let ratio_rate: Ratio<BigInt> = match Ratio::from_float(rate) {
            Some(ratio) => ratio,
            None => {
                println!("Can not convert rate to rational number, thus can not convert inputed currency!");
                return None;
            }
        };
        Some(Self {
            amount_from: ratio_from,
            rate: ratio_rate,
        })
    }

    pub fn convert(self) -> f64 {
        let converted: Ratio<BigInt> =
            self.amount_from * self.rate / Ratio::from_integer(100.to_bigint().unwrap());
        let mut converted_return: f64 =
            converted.numer().to_f64().unwrap() / converted.denom().to_f64().unwrap();
        converted_return = (converted_return * 100.0).round() / 100.0;
        converted_return
    }
}

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

fn main() {
    runHttp();
    println!("How much euros do you have?");
    let from_currency: f64 = match get_float_value() {
        Some(num) => num,
        None => panic!("Can not get proper value!"),
    };
    println!("What is exchsnge rate from euros to dollars USA?");
    let rate: f64 = match get_float_value() {
        Some(num) => num,
        None => panic!("Can not get proper value!"),
    };
    let exchanger = match Exchanger::new(from_currency, rate) {
        Some(exchanger) => exchanger,
        None => return,
    };
    let converted: f64 = exchanger.convert();
    println!("{} euros whould be {} dollars!", from_currency, converted);
}

fn runHttp() {}
