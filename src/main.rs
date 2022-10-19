use std::io;
// this set is called the prelude, you can see everything in it from the stnadard library docs
// https://doc.rust-lang.org/std/prelude/index.html

// If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.

fn main() {
// The fn syntax declares a new function, the parentheses, (), indicate there are no parameters, and the curly bracket, {, starts the body of the function. 

    println!("Guess the number!");

    println!("Please input your guess.");
    // This code is printing a prompt stating what the game is and requesting input from the user.

    let mut guess = String::new();
    // Next, we’ll create a variable to store the user input, like this:    

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}