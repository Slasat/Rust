use num::bigint::{BigInt, ToBigInt};
use num::rational::Ratio;
use num::traits::ToPrimitive;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Check {
    check_list: Vec<(Ratio<BigInt>, u32)>,
    tax_rate: f32,
    ratio_tax_rate: Ratio<BigInt>,
}

impl Check {
    pub fn new() -> Self {
        let tax_rate = 5.5;
        let ratio_tax_rate: Ratio<BigInt> = match Ratio::from_float(tax_rate) {
            Some(ratio) => ratio,
            None => {
                panic!(
                    "Can not convert price to rational number, thus can not add it to tax list!"
                );
            }
        };
        Self {
            check_list: Vec::new(),
            tax_rate: tax_rate,
            ratio_tax_rate: ratio_tax_rate,
        }
    }

    pub fn add(&mut self, price: f32, quantity: u32) {
        let ratio_price: Ratio<BigInt> = match Ratio::from_float(price) {
            Some(ratio) => ratio,
            None => {
                panic!(
                    "Can not convert price to rational number, thus can not add it to tax list!"
                );
            }
        };
        self.check_list.push((ratio_price, quantity));
    }
    fn _get_subtotal(&self) -> Ratio<BigInt> {
        let mut subtotal: Ratio<BigInt> = Ratio::from_integer(0.to_bigint().unwrap());
        for row in &self.check_list {
            let ratio_quantity: BigInt = row.1.to_bigint().unwrap();
            let total_for_row = &row.0 * ratio_quantity;
            subtotal += total_for_row;
        }
        subtotal
    }
    pub fn get_subtotal(&self) -> f64 {
        self.to_float(self._get_subtotal())
    }
    fn _get_tax(&self) -> Ratio<BigInt> {
        let subtotal: Ratio<BigInt> = self._get_subtotal();
        let tax: Ratio<BigInt> = subtotal / 100.to_bigint().unwrap() * &self.ratio_tax_rate;
        tax
    }
    pub fn get_tax(&self) -> f64 {
        self.to_float(self._get_tax())
    }
    fn _get_total(&self) -> Ratio<BigInt> {
        let total: Ratio<BigInt> = self._get_subtotal() + self._get_tax();
        total
    }
    pub fn get_total(&self) -> f64 {
        self.to_float(self._get_total())
    }
    fn to_float(&self, ratio: Ratio<BigInt>) -> f64 {
        let numerator: f64 = match ratio.numer().to_f64() {
            Some(num) => num,
            None => panic!("Can not get numerator!"),
        };
        let denumerator: f64 = match ratio.denom().to_f64() {
            Some(num) => num,
            None => panic!("Can not get denominator!"),
        };
        numerator / denumerator
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for row in &self.check_list {
            output = output + format!("{} pices for ${}\n", row.1, row.0).as_str();
        }
        write!(f, "{}", output)
    }
}

fn get_price() -> Option<f32> {
    println!("Enter the price of item:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line.");
    let price: f32 = match input.trim().parse() {
        Ok(num) => {
            if num < 0.0 {
                println!("Price should be greater or equal to zero.");
                return None;
            } else {
                num
            }
        }
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };
    Some(price)
}

fn get_quantity() -> Option<u32> {
    println!("Enter the quantiry of item:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line.");
    let quantity: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };
    Some(quantity)
}

fn should_break() -> bool {
    println!("One more? [Y]/N");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Can not read line.");
    match input.trim() {
        "Y" => false,
        "N" => true,
        "" => false,
        _ => return should_break(),
    }
}

fn main() {
    let mut check: Check = Check::new();
    loop {
        let price: f32 = match get_price() {
            Some(num) => num,
            None => return,
        };

        let quantity: u32 = match get_quantity() {
            Some(num) => num,
            None => return,
        };
        check.add(price, quantity);
        if should_break() {
            break;
        }
    }
    let subtotal: f64 = check.get_subtotal();
    let tax: f64 = check.get_tax();
    let total: f64 = check.get_total();
    println!("{}", check);
    print!(
        "Subtotal: {:.2}\nTax: {:.2}\nTotal: {:.2}",
        subtotal, tax, total
    );
}
