fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &sl will refer to the value of s1, but does not own it

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s will go out of scope here, but nothing happens because s does not own the string.

// Note: you cannot modify a reference
/*
    E.g
    fn change_string(s: &String) {
        s.push_str(", world!");
    }

    This will throw an error!
 */

 // Of course, we can fix this error by passing a mutable reference
 fn change_string(s: &mut String) {
     s.push_str(", world!");
 }

 // BUT, you can only have ONE mutable reference to a particular piece of data.
 // i.e. the following will fail:
 /*
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
*/

// The following will work:
/*
   let mut s = String::from("hello");

   {
        let r1 = &mut s;
   } // r1 goes out of scope, so this reference no longer exists!

   let r2 = &mut s;
*/

// You CANNOT combine mutable and immutable references:
/* 
   let mut s = String::from("hello");

   let r1 = &s;
   let r2 = &s;
   let r3 = &mut s;
*/

// IN SUMMARY:
// You can have ONE of the following at any given time:
//  - One mutable reference
//  - Any number of immutable references


// Dangling references:
// The following piece of code will fail:
/*
    fn dangle() -> &String {
        let s = String::from("hello")

        &s
    } // s goes out of scope here, so we are referencing deallocated memory!
*/
// The safe method here is to just return the string:
/*
    fn no_dangle() -> String {
        let s = String::from("hello")

        s
    } // s goes out of scope, but the value is moved, so we are good
*/
