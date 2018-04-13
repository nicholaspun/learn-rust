fn main() {
    /*
        Ownership Rules:
        1. Each value in Rust has a variable that’s called its owner.
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value will be dropped.
    */

    // e.g. Variable Scope
    let x = { // y is not yet declared, so it is not yet valid
        let y = 5; // y is now valid
        y
    }; // scope is over, so y is not longer valid
    println!("The value of x is: {}", x);

    // More complex example using the String type:
    // String type variables are stored on the heap (v. String literals -- which are immutable
    // and stored on the stack)
    let mut s = String::from("hello");
    println!("{}", s); // hello
    s.push_str(", world!");
    println!("{}", s); // hello world

    /*
        Understanding how the String type can be modified:
        - String::form requests heap memory
        - drop (a special function that gets called when a String goes out of scope) will return
          the memory for us

        In C++, this is the RAII pattern!
    */

    // E.g. Move:
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y); // Both x and y are 5 -- integer types have the "copy" trait

    let s1 = String::from("hello");
    let _s2 = s1;
    // println!(s1) // This will not work! The value of s1 has been MOVED to s2.
                    // String does not implement the "copy" trait


    // E.g. Clone:
    let s1 = String::from("hello");
    let _s2 = s1.clone(); // Actually performs a copy

    // Ownerships and Functions:

    // Case 1:
    {
        let s = String::from("hello");  // s comes into scope.

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here.

        let x = 5;                      // x comes into scope.

        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it’s okay to still
                                        // use x afterward.

    } // Here, x goes out of scope, then s. But since s's value was moved, nothing
      // special happens.

    // Case 2:
    {
        let _s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1.

        let s2 = String::from("hello");     // s2 comes into scope.

        let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3.
    } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
      // moved, so nothing happens. s1 goes out of scope and is dropped.

    // Case 3:
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope.

    a_string  // a_string is returned and moves out to the calling function.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
