fn main() {
    println!("Hello, world!");

    // Convention: Use snake case for functions and variable names
    parameter_less_function();

    another_function(5);

    // Statements vs Expressions:
    let x = 5; // Statement -- doesn't return anything
    let y = {
        let x = 3;
        x + 1 // Notice how this doesn't have a semicolon --> This is an expression returning the value of x + 1
    }; // This block is also an expression
}

fn parameter_less_function() {
    println!("Paramater-less function");
}

fn another_function(x: i32) { // The type of the parameter must be declared!
    println!("Another function: the value of x is {}", x);
}

fn five() -> i32 { // We can also declare the return type with: ->
    5 // you can use the return keyword to return from a function early
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // This wil return an error since this is a statement! (i.e. Does not return anything)
    x + 1 // this will properly return the value of x + 1
}
