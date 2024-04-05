fn main() {
    type_casting_example();
    arithmetic_example();
    declaring_numbers();

    let number = "4".to_string();
    println!(
        "Convert string \"{}\" into an i64: {}",
        number.to_string(), // ??? borrow checker didn't let me just use number here, so I did this and it worked
        convert_string_to_i64(number)  // we have to convert "4" to a String from a str
    );
}

fn type_casting_example() {
    let x: u8 = 12;
    let y: i8 = 10;

    // adding these numbers of different types is not allowed because this language is
    // very strongly typed
    // let z = x + y;

    // we must do a type conversion, like this:
    let z = (x as i8) + y; // note, we don't need the parentheses, I added them for clarity
    println!("x + y = {}", z);
}

fn arithmetic_example() {
    let num1: u8 = 255;
    let num2: u8 = 10;

    // we are going to add these two numbers, but u8 has a range of 0-255, so we will get an overflow
    // let result = num1 + num2;
    // this causes Rust to panic, as it does not want numbers to overflow
    // println!("Using u8: {} + {} = {}", num1, num2, result);

    // similarly, this would overflow
    // we can fix this by type casting both numbers to an unsigned integer of some sort
    // that fits this number
    // let result = num2 - num1;

    // doing integer division
    let result = num1 / num2;
    println!("{} / {} = {}", num1, num2, result);

    // floating point division
    let num1 = 255.0;
    let num2 = 10.0;
    let result = num1 / num2;
    println!("{} / {} = {}", num1, num2, result);

    // mod works with integers as expected, but it also works with floating points
    let result = num1 % num2;
    println!("{} % {} = {}", num1, num2, result);
}

fn declaring_numbers() {
    // we can specify the type of the number by adding the type after it
    // this is if you want to specify a literal value
    let num1 = 255.0f32;
    let num2 = 10.0f64;
    let num3 = 12u16;
    let num4 = -25i8;

    // this can also be done with an underscore if you want it to be more readable
    let num5 = -89_i64;

    // here is another way of writing numbers that is more readable
    // underscores are ignored in when writing number literals
    let num6 = 1_000_000_i64; // this is a nice way of writing 1,000,000

    // we can even use type casting to specify our variable type
    let num7 = 14_000 as i128;

    // we can even see the maximum value for an integer type like in C++
    let num8 = i32::MAX;
    let num9 = (i32::MAX as i64) + 1;

    // the last number in this experiences an overflow, as it wraps back around to its minimum value
    // due to being 1 above the max of i32. This is not caught by Rust
    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        num1,
        num2,
        num3,
        num4,
        num5,
        num6,
        num7,
        num8,
        num9,
        (num9 as i32)
    );
}

fn convert_string_to_i64(input_string: String) -> i64 {
    //this is a lot, but we use a few string methods
    // the first one gets rid of beginning and trailing whitespace
    // the second one parses the string into a value
    // and the third one converts this value into the proper type (i64 in this case)
    return input_string.trim().parse().unwrap();
}
