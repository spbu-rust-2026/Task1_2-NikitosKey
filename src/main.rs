use num_bigint::BigUint;
use num_traits::Zero;
use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut sum = BigUint::zero();

    for line in lines {
        match line.unwrap().as_str().trim() {
            "-1" => {
                println!("{}", sum);
                return;
            }
            s if s.starts_with('0') => {
                println!("NaN");
                return;
            }
            "" => continue,
            val => match val.parse::<BigUint>() {
                Ok(parsed_num) => sum += parsed_num,
                _ => {
                    println!("NaN");
                    return;
                }
            },
        }
    }
    println!("{}", sum);
}
