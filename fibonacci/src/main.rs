use std::{io, num::ParseIntError};

fn main() {
    let n = loop {
        match get_input() {
            Ok(num) => break num,
            Err(err) => {
                println!("Failed to parse input: {}", err);
                continue;
            }
        }
    };

    let result = match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a: u128 = 0;
            let mut b: u128 = 1;

            let mut result: u128 = 0;

            for i in 2..=n {
                result = match a.checked_add(b) {
                    None => panic!("Overflow error. Failed on n: {}. The previous Fibonacci element ({}) was: {}.", i, i - 1, result),
                    Some(num) => num,
                };
                a = b;
                b = result;
            };

            result
        }
    };

    println!("Final result: {}", result);
}

fn get_input() -> Result<u16, ParseIntError> {
    let mut input_temp = String::new();

    println!("Enter n...");

    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    input_temp.trim().parse()
}
