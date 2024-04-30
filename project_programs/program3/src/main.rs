//Program to serve as a diet planner application
//By Jake Ozer

use std::io;

fn main() {
    println!("---------------------------------------------------------------");
    println!("Welcome to the Nutri-Diet Incorporated's BRAND NEW Diet Planner");
    println!("---------------------------------------------------------------");
    println!("Choose your preferred diet from our scientific, research based options:\n");
    println!("1. The Paleo-Caveman Muscle Builder Extreme (Carnivore Diet)");
    println!("2. If Looks Could Kale (Vegan Diet)");
    println!("3. The Supernova Salmon Slim Down (Pescatarian Diet)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line"); 
    let input: i32 = input.trim().parse().expect("Please enter a number");

    if input >= 1 && input <= 3 {
        match input {
            1=>println!("You entered: {}, THE PALEO CAVEMAN MUSCLE BUILDER EXTREME", input),
            2=>println!("You entered: {}, IF LOOKS COULD KALE", input),
            3=>println!("You entered: {}, THE SUPERNOVA SALMON SLIM DOWN", input),
            _=>println!("input not found."),
        }
       
    } else {
        println!("Input is out of range. Please enter a number between 1 and 3.");
    }

    println!("\nOkay, now enter in the current foods you eat on a regular basis, line by line ->");
}
