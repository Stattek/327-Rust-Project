fn main() {
    test_function();

    // these should overflow
    println!("{}", check_overflow_into_i32((i32::MAX as i64) + 1));
    println!("{}", check_overflow_into_i32((i32::MIN as i64) - 1));
    // this shouldn't overflow
    println!("{}", check_overflow_into_i32(i32::MAX as i64));
    println!("{}", check_overflow_into_i32(i32::MIN as i64));

    block_expression_example();
    println!("{}", add_numbers(4, 6));
}

/**
At this point, I have already found out about functions, and
they are pretty basic.

They are written in snake_case
 */
fn test_function() {
    println!("test_function called");
}

/**
A simple function with a parameter and a return type.
The types for each parameter must be specified, and a return
type must be specified if the function is to return something.

This function takes an i64 parameter and returns a boolean.
This function returns a boolean value of true if converting
this i64 number to an i32 would cause an overflow.
*/
fn check_overflow_into_i32(number: i64) -> bool {
    return (number > (i32::MAX as i64) || number < (i32::MIN as i64));
}

/**
Since a block is an expreesion, we can assign a variable to the value
it returns. This could potentially be useful with calculating a complex
value and then returning it all in the declaration of a variable.
*/
fn block_expression_example() {
    let number = {
        let x = 3;
        x + 1 // this line doesn't have a semicolon because it is
              // returning this value of x+1.
              // If this line had no semicolon, it would cause an error.
    };

    println!("{}", number);
}

/**
This is an example of a function with no semicolon
that just retuns this value.

This feels a lot like functional programming, as
you could use this type of writing to make functions that
only return something instead of doing things more
imperatively.
 */
fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2 // as long as this is the last thing written in
                // a block, then this will return

    // return num1 + num2 // this is another way of writing this
}
