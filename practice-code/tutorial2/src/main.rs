// made by David Slay

fn main() {
    // rust is a statically and strongly typed language
    // when a variable is declared, it is given a type either implicitly or explicitly

    // an example of a variable implicitly being given the type i32 (32 bit integer)
    let num = 4;
    // num = "hello"; // this gives an error because num is not a string

    // an example of printing a variable in rust
    println!("num is: {}", num);

    // num = 5; // this gives an error beacuse this variable is not mutable
    // by default, variables are immutable

    //if we want to make a new variable that is mutable
    let mut num2 = 20;

    println!("num2 is {}", num2);

    num2 = 21; // now we can change it :)

    println!("num2 is {}\n", num2);

    shadowing_example();
    shadowing_example2();
    shadowing_example3();
    const_example()
}

/**
Here is an example of shadowing in Rust.
It is similar in the way that it works to SML.

This also demonstrates that we do not need to put functions'
definitions before they are run. This makes code easier to
write.
 */
fn shadowing_example() {
    let num = 5;
    println!("num in shadowing_example is: {}", num);
    let num = 19;
    println!(
        "num in shadowing_example after being shadowed is now: {}\n",
        num
    );
}

/**
Another example that shows how shadowing works.
It is pretty expected behavior.
 */
fn shadowing_example2() {
    let num = 2;
    println!("num in shadowing_example2 is: {}", num);
    // we can just put curly brackets to specify a scope (idk why you would do this,
    // but I'm doing it for this example)
    {
        let num = 3;
        println!(
            "num in shadowing_example2 in the inner brackets is: {}",
            num
        );
    }

    // this will have the original version of num, as expected
    println!(
        "num in shadowing_example2 in the outer brackets is: {}\n",
        num
    );
}

fn shadowing_example3() {
    let var = 10;
    println!("variable in shadowing_example3 is: {}", var);

    // as with shadowing in SML, we can shadow variables
    // with variables of different types.
    // This is not changing the original variable, it is
    // creating a new one with a different type, as Rust
    // is very strongly typed
    let var = "Hello";
    println!(
        "var in shadowing_example3 after being shadowed is: {}\n",
        var
    );
}

fn const_example() {
    // when defining a constant, we have to specify its type
    // with a type annotation, which we have seen before in SML
    const SECONDS_IN_MINUTE: i32 = 60;
    // SECONDS_IN_MINUTE = 42; //obviously you can't do this

    println!("SECONDS_IN_MINUTE: {}\n", SECONDS_IN_MINUTE);
}
