use std::io;

fn main() {
    //user inputs foods they eat
    let mut food_list: Vec<String> = Vec::new();
    println!("\nOkay, now enter in the current foods you eat on a regular basis, line by line. enter STOP to see results ->");
    let mut input = String::new();
    while input.trim() != "STOP" {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        food_list.push(input.clone());

        println!("{}", input.trim());
        println!(
            "\nEnter another current food you eat on a regular basis, enter STOP to see results ->"
        );
    }
}
