use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut sum: u128 = 0;

    for line in lines {
        match line.unwrap().trim() {
            "-1" => {
                println!("{}", sum);
                return;
            }
            "0" => {
                println!("NaN");
                return;
            }
            "" => continue,
            val => match val.parse::<u128>() {
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
