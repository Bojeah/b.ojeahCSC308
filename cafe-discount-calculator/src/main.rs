use std::io;

fn main() {
    
    println!("Enter your bill amount (₦):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let bill: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let discount_rate = if bill > 10000.0 {
        0.15
    } else if bill > 5000.0 {
        0.10
    } else {
        0.0
    };

    let discount_amount = bill * discount_rate;
    let final_bill = bill - discount_amount;

    println!("Original Bill: ₦{:.2}", bill);
    println!(
        "Discount Applied: {}%",(discount_rate * 100.0).to_string()
        
    );
    println!("Final Bill: ₦{:.2}", final_bill);
}
