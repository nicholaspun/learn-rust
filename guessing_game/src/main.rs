use std::io;
// Very few types added in std::prelude, std::io brings in I/O features

fn main() {
    println!("Guess the number!");

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

    println!("You guessed: {}", guess); // {} is a placeholder
}
