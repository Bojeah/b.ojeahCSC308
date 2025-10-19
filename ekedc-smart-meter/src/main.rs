use std::io;

fn main() {
    println!("Enter your energy consumption in kWh:");

    // Read user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Convert to number
    let usage: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    // Determine rate
    let rate = if usage > 200.0 {
        30.0
    } else if usage > 100.0 {
        25.0
    } else {
        20.0
    };

    // Calculate total
    let total_bill = usage * rate;

    // Display formatted result
    println!("Energy Used: {:.0} kWh", usage);
    println!("Rate Applied: ₦{:.0} per kWh", rate);
    println!("Total Bill: ₦{:.2}", total_bill);
}
