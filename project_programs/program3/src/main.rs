//Program to serve as a diet planner application
//By Jake Ozer

use std::io;

enum Diet {
    Carnivore(Vec<&'static str>),
    Vegan(Vec<&'static str>),
    Pescatarian(Vec<&'static str>),
}

impl Diet {
    fn get_food_list(&self) -> &Vec<&'static str> {
        match self {
            Diet::Carnivore(food_list) => food_list,
            Diet::Vegan(food_list) => food_list,
            Diet::Pescatarian(food_list) => food_list,
        }
    }
}

fn main() {
    let carnivore_diet = Diet::Carnivore(vec![
        "beef", "chicken", "pork", "bacon", "lamb", "venison", "duck", "rabbit", "turkey",
        "quail", "goose", "buffalo", "salami", "burger", "steak",
    ]);

    let vegan_diet = Diet::Vegan(vec![
        "tofu", "beans", "quinoa", "kale", "spinach", "avocado", "grapes", "kale", "lentils",
        "broccoli", "cauliflower", "chickpeas", "mushrooms", "nuts", "berries",
    ]);

    let pescatarian_diet = Diet::Pescatarian(vec![
        "salmon", "tuna", "shrimp", "sardines", "tilapia", "trout", "catfish", "mackerel",
        "crab", "cod", "halibut", "anchovies", "swordfish", "clams", "oysters",
    ]);

    println!("---------------------------------------------------------------");
    println!("Welcome to the Nutri-Diet Incorporated's BRAND NEW Diet Planner");
    println!("---------------------------------------------------------------");
    println!("Choose your preferred diet from our scientific, research based options:\n");
    println!("1. The Paleo-Caveman Muscle Builder Extreme (Carnivore Diet)");
    println!("2. If Looks Could Kale (Vegan Diet)");
    println!("3. The Supernova Salmon Slim Down (Pescatarian Diet)");

    let diet_choice: Diet;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Please enter a number");

    match input {
        1 => {
            println!("You entered: {}, THE PALEO CAVEMAN MUSCLE BUILDER EXTREME", input);
            diet_choice = carnivore_diet;
        }
        2 => {
            println!("You entered: {}, IF LOOKS COULD KALE", input);
            diet_choice = vegan_diet;
        }
        3 => {
            println!("You entered: {}, THE SUPERNOVA SALMON SLIM DOWN", input);
            diet_choice = pescatarian_diet;
        }
        _ => {
            println!("Input not found.");
            return;
        }
    }

    let mut food_list: Vec<String> = Vec::new();
    println!("\nOkay, now enter in the current foods you eat on a regular basis, line by line. Enter STOP to see results ->");
    let mut input = String::new();

    while input.trim().to_ascii_uppercase() != "STOP" {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().to_ascii_uppercase() != "STOP" {
            food_list.push(input.trim().to_string().to_ascii_lowercase());
            println!("\nEnter another current food you eat on a regular basis, enter STOP to see results ->");
        }
    }

    let filtered_food_list = filter_for_diet(&food_list, &diet_choice);

    println!("Based on your current diet, you can only eat...\n");
    println!("{}", filtered_food_list.join(", "));
    println!("\nThanks for using the diet planner. Come back for more nutrition tips later!");
}

fn filter_for_diet(food_list: &Vec<String>, diet: &Diet) -> Vec<String> {
    let filtered_food_list: Vec<String> = food_list
        .iter()
        .filter(|food| diet.get_food_list().contains(&food.as_str()))
        .cloned()
        .collect();

    filtered_food_list
}
