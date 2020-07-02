// Converting temperature from Fahrenheit to Celsius
use std::io;

fn main() {
    let mut temp_fahr = String::new();

    println!("Hi! Please print temperature in Fahrenheit:");
    io::stdin()
        .read_line(&mut temp_fahr)
        .expect("Failed to read line");

    let temp_fahr: f32 = match temp_fahr.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("error: {}", error);
            0.0
        }
    };

    let temp_cels = convert_temperature(temp_fahr);
    println!("{}", &temp_cels);
}

fn convert_temperature(temp_fahr: f32) -> f32 {
    (temp_fahr - 32.0) * 5.0 / 9.0
}
