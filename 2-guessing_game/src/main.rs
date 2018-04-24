extern crate rand;
// Tells rust that we're using an external dependency

use std::io;
// Very few types added in std::prelude, std::io brings in I/O features
use std::cmp::Ordering;
// Ordering is an enum of type: Less, Greater, or Equal
use rand::Rng;
// Rng is a "trait" that defines methods that random number generators implement
// Must in in scope! -- Covered in more detail in chpt.10

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // rand::thread_rng() is the function that provides us the rng
    // Note: It is local to this thread
    // gen_range(a,b) uses the rng and generates a number in [a, b)    

    // Infinite loop
    loop {
        println!("Please input your guess.");

        // Note: Variables are immutable by default.
        // E.g. let guess; // immutable
        //      let mut guess; // mutable
        let mut guess = String::new(); // new is known as an associated function of String.
                                       // The function is implemented on the type rather than the instance. (i.e. A static method)
                                       // Here, new is creating a new, empty instance of a String type.

        // We could have written this as std::io::stdin()
        // The stdin function returns an instance of std::io::Stdin -- A type that represents a handle to stdin
        // & indicates a reference, which are immutable by default.
        // So, &mut VARNAME indicates that you want the VARNAME reference to be mutable. 
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); 
        // read_line returns a value of type io::Result, which is an enum that is either Ok or Err.
        // expect is an associated function of the Result type. 
        // When called on a Result of type Err -- the program will crash and display the error message
        // Otherwise, if Result is of type Ok, expect will return the value that Ok is holding
        // If expect is NOT called, the program will compile with warnings


        // Two things to note here!
        // 1) Rust is strongly typed, so when we call cmp (below), we must make sure that both secret_number and guess are numbers!
        // 2) We've reused the variable guess -- Rust allows us to shadow the previous value with a new one
        let guess: u32 = match guess.trim().parse() { 
	    Ok(num) => num,
	    Err(_) => continue, // Skips invalid inputs
	};
        //    .expect("Please type a number!"); // We used an expect before -- a match is better error handling    

        println!("You guessed: {}", guess); // {} is a placeholder

        // cmp is a method that takes in two values and returns an Ordering 	
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
	        println!("You win!");
		break;
            }
	}
    }
}
