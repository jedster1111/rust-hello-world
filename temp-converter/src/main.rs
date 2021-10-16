use std::io;
use core::num::ParseIntError;

fn main() {
    let temp_celsius = loop {
        match get_input() {
            Ok(num) => break num,
            Err(error) => {
                println!("Error parsing input: {}", error);
                continue;
            }
        }
    };

    println!("Entered temp: {} °C", temp_celsius);

    let temp_fahrenheit = to_fahrenheit(temp_celsius);

    println!("Temp in fahrenheit: {} °F", temp_fahrenheit);
}

fn get_input() -> Result<i32, ParseIntError> {
    let mut input_temp = String::new();

    println!("Enter a temperature in Celsius.");

    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    input_temp.trim().parse()
}

fn to_fahrenheit(temp: i32) -> i32 {
    (temp * 9 / 5) + 32
}
