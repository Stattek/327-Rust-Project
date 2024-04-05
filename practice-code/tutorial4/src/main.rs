// we are going to use a prelude for the first time
// these are basically like import statements in Python or like the preprocessing
// statements from C++

// some things are included in the prelude by default

// a cargo (also called a package or library) holds code that we can use
use std::io;

fn main() {
    println!("Input a sentence:");

    // we haven't gone over this yet, but this is how you make a
    // string. It is similar to how it is done in Java

    // we need a string to take the input
    let mut input = String::new();

    // this is the standard input from the console
    // we are creating a mutable reference to this input variable
    // which will allow this function to directly modify the data stored in input
    // (by default, Rust does pass by value, but we can pass a reference explicitly
    // to change the input. Additionally, we will have to make this mutable so the
    // function call can actually change input, otherwise it would be immutable when passed, despite being mutable)
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line"); // this is catching any errors that will occur from this
                                        // this is because .read_line() returns a Result object, which we must handle

    println!("{}", input);
}
