//This program act as a cashier at checkout where the user enter their item pirce amd quantity and the program will print out the total amount with tax
//by Yu Yu Hong (Charlie ) Zheng

use std::io;

fn main() {
    println!("Welcome to the shop checkout!");

    let mut total_cost = 0.0;
    let tax_rate = 0.08; //instaniat 8 percent tax rate

    loop {
        // Get user input for item cost and quantity
        let (item_cost, quantity) = get_user_input();

        // Calculate item total
        let item_total = item_cost * quantity as f64;
        total_cost += item_total;

        println!("Item total: ${:.2}", item_total);

        loop {
            // Ask if the user wants to add more items
            println!("Do you want to add more items? (y/n)");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");

            match choice.trim().to_lowercase().as_str() {
                "y" => break,
                "n" => {
                    // Calculate tax amount and grand total
                    let tax_amount = total_cost * tax_rate;
                    let grand_total = total_cost + tax_amount;

                    // Print the subtotal, tax amount, and grand total
                    println!("");
                    println!("--------------------------------------");
                    println!("Subtotal: ${:.2}", total_cost);
                    println!("Tax (8%): ${:.2}", tax_amount);
                    println!("Grand Total: ${:.2}", grand_total);
                    return;
                }
                _ => {
                    println!("Invalid input. Please enter 'y' or 'n'.");
                    continue;
                }
            }
        }
    }
}

fn get_user_input() -> (f64, i32) {
    loop {
        // Get user input for item cost
        println!("Enter the cost of the item (up to 2 decimal places):");
        let mut item_cost = String::new();
        io::stdin().read_line(&mut item_cost).expect("Failed to read input");

        // Validate item cost input
        if let Ok(cost) = item_cost.trim().parse::<f64>() {
            if cost >= 0.0 && cost.to_string().split('.').last().unwrap_or("").len() <= 2 {
                loop {
                    // Get user input for item quantity
                    println!("Enter the quantity of the item (integer only):");
                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).expect("Failed to read input");

                    // Validate item quantity input
                    if let Ok(qty) = quantity.trim().parse::<i32>() {
                        if qty >= 0 {
                            return (cost, qty);
                        }
                    }
                    println!("Invalid quantity. Please enter a non-negative integer.");
                }
            }
        }
        println!("Invalid cost. Please enter a non-negative value with up to 2 decimal places.");
    }
}