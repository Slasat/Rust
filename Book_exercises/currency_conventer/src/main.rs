use num::bigint::BigInt;
use num::rational::Ratio;
use num::traits::ToPrimitive;
use serde::Deserialize;
use std::collections::HashMap;
use std::io;

const APPID: &str = "3917e0cb50a9472ab78aabd8c6ac2998";
const URL: &str = "https://openexchangerates.org/api/latest.json";

struct Exchanger {
    amount_from: Ratio<BigInt>,
    rate: Ratio<BigInt>,
}

#[derive(Deserialize, Debug)]
struct OpenExchangeRates {
    disclaimer: String,
    license: String,
    timestamp: u32,
    base: String,
    rates: HashMap<String, f64>,
}

struct Rate {
    rates: HashMap<String, f64>,
}

impl Rate {
    pub fn new() -> Option<Self> {
        let rates: HashMap<String, f64> = match Rate::_get_rates() {
            Ok(rates) => rates,
            _ => return None,
        };
        Some(Self { rates: rates })
    }

    fn _get_rates() -> serde_json::Result<HashMap<String, f64>> {
        let result: String = match Rate::_run_http() {
            Ok(body) => body,
            _ => panic!("Can not get rates!"),
        };
        let parsed: OpenExchangeRates = serde_json::from_str(&result)?;
        Ok(parsed.rates)
    }

    fn _run_http() -> reqwest::Result<String> {
        let client = reqwest::blocking::Client::new();
        let res = client.get(URL).query(&[("app_id", APPID)]).send()?;
        let body = res.text();
        body
    }

    pub fn get(self: Rate, name: String) -> Option<f64> {
        match self.rates.get(&name) {
            Some(rate) => return Some(*rate),
            None => return None,
        }
    }
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
        let converted: Ratio<BigInt> = self.amount_from * self.rate;
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

fn get_currency_name() -> String {
    let mut in_line = String::new();
    io::stdin()
        .read_line(&mut in_line)
        .expect("Failed to read line");
    in_line.trim().to_string()
}

fn main() {
    let rate = match Rate::new() {
        Some(rate) => rate,
        None => {
            println!("Can not get rates");
            return;
        }
    };
    println!("Avaliable currencies:");
    for name in rate.rates.keys() {
        println!("{}", name);
    }
    println!("What target currency is?");
    let currency_name: String = get_currency_name();

    let rate: f64 = match rate.get(currency_name.to_string()) {
        Some(num) => num,
        None => panic!("Can not get rates for {}!", currency_name),
    };

    println!("Exchange rate from USD to {} is {}.", currency_name, rate);

    println!("How much USD do you have?");

    let from_currency: f64 = match get_float_value() {
        Some(num) => num,
        None => panic!("Can not get proper value!"),
    };

    let exchanger = match Exchanger::new(from_currency, rate) {
        Some(exchanger) => exchanger,
        None => return,
    };
    let converted: f64 = exchanger.convert();
    println!(
        "{} of USD whould be {} {}!",
        from_currency, converted, currency_name
    );
}
