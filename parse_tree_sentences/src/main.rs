fn main() {
    // first parse tree sentence
    for number in 1..10 {
        print!("{}", number);
    }

    // test code
    println!();

    // test code
    let animal_string = String::from("chicken");

    // second parse tree sentence
    if animal_string == "chicken" {
        println!("It's a chicken");
    } else if animal_string == "donkey" {
        println!("It's a donkey");
    } else {
        println!("Not a chicken or a donkey");
    }

    // test code for third parse tree sentence
    let num = 19;

    // third parse tree sentence
    match num {
        1..=10 => println!("This is a number 1-10"),
        _ => println!("This number is not 1-10"),
    };

    // testing fourth parse tree sentence
    let my_string = gives_ownership();
    println!("{}", my_string);
}

// fourth parse tree sentence
fn gives_ownership() -> String {
    String::from("From gives_ownership()")
}
