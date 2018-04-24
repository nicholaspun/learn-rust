fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    /*
    This won't work! You cannot reassign an immutable variable!
    The error: "Cannot assign twice to immutable variable" will occur during compliation
    x = 6;
    println!("The value of x is: {}", x);
    */

    // If we want to have a mutable variable ...
    let mut y = 5;
    println!("The value of mutable y is: {}", y);
    y = 6;
    println!("Now the value of mutable y is: {}", y);

    // We can define constants:
    const MAX_POINTS: u32 = 100000;

    // Shadowing:
    let shadow_x = 5;
    println!("The value of shadow_x is: {}", shadow_x);

    let shadow_x = shadow_x + 1;
    println!("The value of shadow_x is: {}", shadow_x);

    let shadow_x = shadow_x * 2;
    println!("The value of shadow_x is: {}", shadow_x);
    // Note: Here we are effectively creating a new variable
    // This also works:

    let spaces = "   "; // string
    let spaces = spaces.len(); // number
    // However ...
    let mut mutable_spaces = "   ";
    // mutable_spaces = mutable_spaces.len();
    // This will NOT work --> we cannot change the type of mutable_spaces

}
