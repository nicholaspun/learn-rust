fn main() {
    // if expressions

    let number = 3;

    if number < 5 { // We call each block an "arm"
    // Note that the condition MUST be a bool
    // Rust will NOT convert non-Boolean types to a Boolean
        println!("condition was true");
    } else if number == 5 {
        println!("Else if")
    } else {
        println!("condition was false");
    }

    // Note that if is an EXPRESSION, so we can do:
    let condition = true;
    let number2 = if condition {
        5
    } else {
        6
        // "six" // Suppose this were the return value of the else block
                 // This would NOT compile!
                 // Integers and Strings are not compatible
    };
    println!("The value of number2 is: {}", number2);

    // loop
    /*
    An infinite loop:

    loop {
        println!("loop! ")
    }

    We can break out of this with an if.
    Alternatively:
    */

    // while
    let mut while_example_number = 3;
    while while_example_number != 0 {
        println!("{}!", while_example_number);

        while_example_number = while_example_number - 1;
    }
    println!("LIFTOFF!");

    // for
    /*
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;

    We could loop through the array like this ^,
    Alternatively:
    */

    let arr = [10, 20, 30, 40, 50];

    for ele in arr.iter() {
        println!("{}", ele);
    }

    for ele in (1..4).rev() {
        println!("{}", ele);
    }

}
