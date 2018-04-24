fn main() {
    let s = String::from("hello world");
    println!("{}", s); // "hello world"

    // [start..end] will reference the portion of the string from start and up to, but not including, end
    let hello = &s[0..5];
    println!("{}", hello); // "hello"
    let world = &s[6..11];
    println!("{}", world); // "world"

    // The following is also valid:
    let hello_world = &s[..];
    println!("{}", hello_world); // "hello"
    let hello = &s[..5];
    println!("{}", hello); // "hello"
    let world = &s[6..];
    println!("{}", world); // "world"

    // Why is this useful?
    /*
        Consider:
        let mut s = String::from("hello world");

        let word = first_word(&s); // Immutable reference

        s.clear(); // Mutable reference -- this will throw an error!
    */

    // String literals are slices!
    // e.g. let s = "Hello world!"; is a slice pointing to a specific point in the binary
    // This is why string literals are immutable!

    // String slices as parameters:
    /*
        Note: A String is a slice (type &str).

        fn first_word(s: &String) will take in Strings
        fn first_word(s: &str) will take in Strings AND string slices

    */

    // Slices are not limited to strings! For example, we could have &[i32] a slice of an array of ints
}
