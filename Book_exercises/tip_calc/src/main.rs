use std::io;
use num::bigint::BigInt;
use num::rational::Ratio;
use num::traits::ToPrimitive;

fn get_value() -> Option<f64> {
    let mut in_line = String::new();
    io::stdin().read_line(&mut in_line)
        .expect("Failed to read line");
    let number:f64 = match in_line.trim().parse(){
        Ok(num) => num,
        Err(_) => return None,
    };
    Some(number)
}

fn calculate_tip(bill:f64, tip_rate:f64) -> f64 {
    let _bill: Ratio<BigInt> = Ratio::from_float(bill).unwrap();
    let mut tip: Ratio<BigInt> = Ratio::from_float(tip_rate).unwrap();
    tip = tip / Ratio::from_float(100.0).unwrap();
    let total_tip = _bill * tip;
    let mut converted_total_tip:f64 = total_tip.numer().to_f64().unwrap() / total_tip.denom().to_f64().unwrap();
    converted_total_tip = (converted_total_tip * 100.0).ceil() / 100.0;
    converted_total_tip
}


fn main() {
    loop {
        let mut bill:f64;
        loop {
            println!("What is the bill amount?");
            bill = match get_value(){
                Some(num) => num,
                None => {
                    println!("Wrong value given!");
                    continue
                }
            };
            if bill <= 0.0 {
                println!("Wrong amount: bill should be greater than or equal to 0");
                continue
            }
            break;
        }
        let mut tip:f64;
        loop {
            println!("What is the tip rate?");
            tip = match get_value(){
                Some(num) => num,
                None => {
                    println!("Wrong value given!");
                    continue
                }
            };
            if tip <= 0.0 {
                println!("Wrong amount: tip should be greater than or equal to 0");
                continue;
            }
            break;
        }
        let _total_tip = calculate_tip(bill, tip);
        println!("Tip: ${:.2}", _total_tip);
        println!("Total: ${:.2}", bill + _total_tip);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculations() {
        assert_eq!(calculate_tip(100_f64, 10_f64), 10_f64);
        assert_eq!(calculate_tip(45.5_f64, 13.5_f64), 6.15_f64);
        assert_eq!(calculate_tip(200_f64, 15_f64), 30_f64);
        assert_eq!(calculate_tip(10_f64, 15_f64), 1.50_f64);
    }
    #[test]
    fn proper_round() {
        assert_eq!(calculate_tip(11.25_f64, 15_f64).to_string(), "1.69");
    }

}

