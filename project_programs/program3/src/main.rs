//Program to serve as a diet planner application
//By Jake Ozer

use std::io;

use alloc::collections;

fn main() {

    //database of diet options
    let carnivore_diet = vec![
        "beef",
        "chicken",
        "pork",
        "bacon",
        "lamb",
        "venison",
        "duck",
        "rabbit",
        "turkey",
        "quail",
        "goose",
        "buffalo",
        "boar",
        "elk",
        "kangaroo",
    ];
    
    let vegan_diet = vec![
        "tofu",
        "beans",
        "quinoa",
        "kale",
        "spinach",
        "avocado",
        "grapes",
        "kale",
        "lentils",
        "broccoli",
        "cauliflower",
        "chickpeas",
        "mushrooms",
        "nuts",
        "berries",
    ];
    
    let pescatarian_diet = vec![
        "salmon",
        "tuna",
        "shrimp",
        "sardines",
        "tilapia",
        "trout",
        "catfish",
        "mackerel",
        "crab",
        "cod",
        "halibut",
        "anchovies",
        "swordfish",
        "clams",
        "oysters",
    ];


    //starting menu
    println!("---------------------------------------------------------------");
    println!("Welcome to the Nutri-Diet Incorporated's BRAND NEW Diet Planner");
    println!("---------------------------------------------------------------");
    println!("Choose your preferred diet from our scientific, research based options:\n");
    println!("1. The Paleo-Caveman Muscle Builder Extreme (Carnivore Diet)");
    println!("2. If Looks Could Kale (Vegan Diet)");
    println!("3. The Supernova Salmon Slim Down (Pescatarian Diet)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //shadowing input to be an i32 and hold our value
    let input: i32 = input.trim().parse().expect("Please enter a number");

    if input >= 1 && input <= 3 {
        match input {
            1 => println!("You entered: {}, THE PALEO CAVEMAN MUSCLE BUILDER EXTREME", input),
            2 => println!("You entered: {}, IF LOOKS COULD KALE", input),
            3 => println!("You entered: {}, THE SUPERNOVA SALMON SLIM DOWN", input),
            _ => println!("input not found."),
        }
    } else {
        println!("Input is out of range. Please enter a number between 1 and 3.");
    }

    //user inputs foods they eat
    let mut food_list: Vec<String> = Vec::new();
    println!("\nOkay, now enter in the current foods you eat on a regular basis, line by line. enter STOP to see results ->");
    let mut input = String::new();

    while input.trim().to_ascii_uppercase() != "STOP" {
        // reset the value of input to avoid errors
        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_ascii_uppercase() != "STOP" {
            food_list.push(input.clone().trim().to_string());
            println!(
                "\nEnter another current food you eat on a regular basis, enter STOP to see results ->"
            );
        }

        
    }



    println!("{:?}", food_list);
    print!("end program debug");
}

fn filter_for_diet(food_list: Vec<&str>, diet: Vec<&str>) -> Vec<&str> {

    food_list.iter().filter(|item| in_diet(item, diet)).collect();
}

fn in_diet(item: String, diet: Vec<&str>) -> bool {
    for food in diet {
        if item == food {
            return true;
        }
    }
    return false;
}