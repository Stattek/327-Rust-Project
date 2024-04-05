fn main() {
    boolean_operator_example();
    if_statement_test("chicken");
    if_statement_test("donkey");
    if_statement_test("horse");
}

fn boolean_operator_example() {
    /*
    We have familiar boolean operators:
        <
        >
        <=
        >=
        !=
        ==

    These are the same as most languages
    */

    // we can make a boolean value and print it
    let test = 2 < 3;
    println!("{}", test);

    let test2 = 2 >= 3;
    println!("{}", test2);

    // we can't use different types on the left and right sides
    // let test3 = 2.0 < 3; // this gives an error
    let test3 = 2.0 < (3 as f64);
    println!("{}", test3);

    /*
    When we have multiple conditions we can use logical operators
    These are familiar and in order of precedence:
        ! - NOT
        && - AND
        || - OR
    */
    let test4 = 2 < 3 && 4 > 3;
    println!("{}", test4);

    let test5 = 2 > 3 || 4 > 3;
    println!("{}", test5);

    let test6 = !test5;
    println!("{}", test6);
}

/**
This is an if statement test that takes an input string.

This input string is a very basic string literal.
It also takes a reference to the string (because taking a string
by value is weird in Rust and requires you to know its length)
 */
fn if_statement_test(animal_string: &str) {
    // basic if, else if, and else statements
    if animal_string == "chicken" {
        println!("{}", animal_string);
    } else if animal_string == "donkey" {
        println!("{}", animal_string);
    } else {
        println!("not chicken or donkey");
    }
}
