fn main() {
    let my_value: Option<i32> = None;

    match my_value {
        Some(my_value) => println!("{}", my_value),
        None => println!("No value in this variable."),
    }

    // also, here is a different way to use an option
    if let Some(number) = my_value {
        println!("{}", number);
    }
    // notice we don't have to take care of every pattern for this
    
}
