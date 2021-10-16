use std::{io, num::ParseFloatError};

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

    let temp_fahrenheit = to_fahrenheit(temp_celsius);

    println!("{} °C is {} °F", temp_celsius, temp_fahrenheit);
}

fn get_input() -> Result<f32, ParseFloatError> {
    let mut input_temp = String::new();

    println!("Enter a temperature in Celsius.");

    io::stdin()
        .read_line(&mut input_temp)
        .expect("Failed to read line");

    input_temp.trim().parse()
}

fn to_fahrenheit(temp: f32) -> f32 {
    (temp * 9. / 5.) + 32.
}
