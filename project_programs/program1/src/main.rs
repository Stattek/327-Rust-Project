/*
Rust Program 1. This is a guess the number game where the user can enter in a max range of numbers which will only work for
unsigned numbers. The user will then continue to guess and the program will tell the user if their guess is too high or too low.
The program will keep track of wrong guesses and once the user guesses the correct number, the number of incorrect guess will be displayed
and the program will end.

by James Bieber
*/

use rand::Rng;
use std::io;

fn main() {
    println!("Random Number Guess Game. Please enter a maximum value: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read user input");

    //Initializes variables used in the loop
    let max_value: u64 = input.trim().parse().unwrap();
    let number = rand::thread_rng().gen_range(1..=max_value);
    let mut guessed = false;
    let mut wrong_guesses: u64 = 0;

    while guessed != true
    //loop will end if the random number is guessed.
    {
        let mut guess_str = String::new(); // re-initializes the guess_str for each iteration.
        println!("Enter in a guess: ");
        io::stdin()
            .read_line(&mut guess_str)
            .expect("failed to read user input");
        let guess: u64 = guess_str.trim().parse().unwrap(); //trims and parses the user's input into a unsigned integer
        if guess == number
        //condition if user guessed correctly
        {
            println!("You guessed: {} The number is: {}", guess, number);
            println!("You number of wrong guesses is {}", wrong_guesses);
            println!("You won!");
            guessed = true;
        } else if guess > number
        //condition if user guessed a number larger than the randomly generated number
        {
            println!("You guessed: {} That is too high", guess);
            wrong_guesses += 1;
        } else
        //condition if user guessed a number smaller than the randomly generated number
        {
            println!("You guessed: {} That is too low", guess);
            wrong_guesses += 1;
        }
    }
}
