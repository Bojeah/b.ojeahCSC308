use std::io;

fn main() {
    println!("Enter the temperature value:");

    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read input");
    let temperature: f64 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature. Please enter a number.");
            return;
        }
    };

    println!("Enter the unit (C for Celsius, F for Fahrenheit):");
    let mut unit_input = String::new();
    io::stdin()
        .read_line(&mut unit_input)
        .expect("Failed to read input");
    let unit = unit_input.trim().to_uppercase();

    if unit == "C" {
        let converted = (temperature * 9.0 / 5.0) + 32.0;
        println!("Temperature: {:.1}°C Converted: {:.1}°F", temperature, converted);
    } else if unit == "F" {
        let converted = (temperature - 32.0) * 5.0 / 9.0;
        println!("Temperature: {:.1}°F Converted: {:.1}°C", temperature, converted);
    } else {
        println!("Invalid unit. Please enter 'C' or 'F'.");
    }
}
