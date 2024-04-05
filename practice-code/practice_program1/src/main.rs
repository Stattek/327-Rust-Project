fn main() {
    println!("2 squared: {}", square(2));
    println!("3 squared: {}", square(3));

    for_loop_test();

    println!("Is 3 divisible by 2? -> {}", divisible_by(3, 2));
    println!("Is 3 divisible by 1? -> {}", divisible_by(3, 1));
    println!("Is 35 divisible by 5? -> {}", divisible_by(35, 5));
    println!("Is 0 divisible by 0? -> {}", divisible_by(0, 0));

    println!("True: {}", match_example(true));
    println!("False: {}", match_example(false));
}

/**
This is a test function that shows how to take
a parameter and also return a type.

This looks a lot like SML, and it feels like it inherits
more from functional programming

 */
fn square(number: i32) -> i32 {
    return number * number;
}

fn divisible_by(number1: u32, number2: u32) -> bool {
    let mut output = false;

    // if the number we are dividing by is not 0, and
    // number1 is divisible by number2, then the output is true
    if number2 != 0 && number1 % number2 == 0 {
        output = true;
    }

    return output;
}

/**
This is a test of for loops in Rust. It's very similar
to how for loops work in Python, where the for loop
takes an iterator from an iterable object, such
as an array in this case.
 */
fn for_loop_test() {
    // create an array
    let my_array = [1, 2, 3, 4, 5];

    // iterate through every element in my_array and print it
    for number in my_array {
        print!("{} ", number);
    }
    println!("\n");

    // second part of the test

    // prints numbers from 1 to 9 because it is inclusive on the left
    // side but exclusve on the right side
    for number in 1..10 {
        print!("{} ", number);
    }
    println!();

    // if we wanted to do that last test but with 10 included
    for number in 1..=10 {
        print!("{} ", number);
    }
    println!("\n");
}

fn match_example(my_bool: bool) -> char {
    let output = match my_bool {
        true => 't',
        false => 'f',
    };

    return output;
}
