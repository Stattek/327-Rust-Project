/*
The Three Ownership Rules:
1. Each value in Rust has a variable called its owner
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped

Basically, ownership is another way of memory management
different from how C++ does it. It allows control over memory
but the compiler requires you to do things in a safe way,
and it enforces a borrowing system.
*/

fn main() {
    borrow_test1();
    borrow_test2();
    borrow_test3();
    borrow_test4();
}

fn borrow_test1() {
    {
        // this will automatically create a dynamic String on the heap
        let my_string = String::from("hello");
    } // now that this scope is over, my_string is out of scope and the value "hello" is deallocated from the heap

    let num1 = 5;
    let num2 = num1; // this will copy the value of num1 into num2

    let string1 = String::from("hello");
    let mut string2 = string1; // instead of making a copy or copying an address, this does a move
                               // this means that the value on the heap that string1 was pointing to is now what string2 is pointing to
                               // and now string1 is invalidated

    // this will cause an error because the value of string1 was moved to
    // string2, and now string2 is the new owner of this value.
    // println!("{}", string1); // causes an error

    // this makes a deep copy and does not take ownership of string2's value
    let string3 = string2.clone();

    string2 = String::from("world"); // since string2 owns this value, we can change it

    println!("{}", string3);
    println!("{}", string2);
}

fn borrow_test2() {
    let string = String::from("hello");
    takes_ownership(string);
    // println!("{}", string); // this is not allowed because takes_ownership will take ownership of string

    let number = 5;
    makes_copy(number); // this causes no problem because it makes a copy instead of taking ownership of number
    println!("{}", number);
}

fn takes_ownership(the_string: String) {
    println!("{}", the_string);
}

fn makes_copy(number: i32) {
    println!("{}", number);
}

fn borrow_test3() {
    let string1 = gives_ownership();
    let string2 = String::from("hello");
    let string3 = takes_and_gives_back(string2);
    println!("string1 = {}, string3 = {}", string1, string3,);
}

fn gives_ownership() -> String {
    let temp_string = String::from("from gives_ownership()");

    temp_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn borrow_test4() {
    let string1 = String::from("string1");
    // get the length of string1
    let length = calculate_length(&string1);
    println!("The length of {} is {}", string1, length);

    // let's make another String
    let mut string2 = String::from("string2");

    // we now pass a mutable reference to string2
    modify_string(&mut string2);

    // modify_string borrwed string2 and modified it, and we can still call
    // this function without having problems with the borrow checker
    println!("string2's value is: {}", string2);
}

/**
This function calculates the length of a
string.

This is an example of a function that does not take ownership
of the parameter passed in. This is because it has a reference that
points to the actual parameter. This process is calld "borrowing."
 */
fn calculate_length(the_string: &String) -> usize {
    let length = the_string.len(); // len() returns the length of a String
    length
}

/**
This function does a basic modification to a variable
passed in.

This is an example of borrowing a value and modifying it.
It takes a mutable reference to a String.
 */
fn modify_string(the_string: &mut String) {
    the_string.push_str("-modified");
}
