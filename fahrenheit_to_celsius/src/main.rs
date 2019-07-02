use std::io;

fn main() {
    loop {
        println!("Enter a temperature in Fahrenheit.");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = fahrenheit_to_celsius(temperature);

        println!("{} degree Fahrenheit is equal to {} degree Celsius", temperature, celsius);

        break;
    }
}

fn fahrenheit_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) * 5 / 9
}
