fn main() {
    /*
    Preface: Rust is statically-typed, and the compiler can usually infer
    the type

    Sometimes we might need to be explicit:
    e.g. let guess: u32 = "42".parse()
    */

    /*
    There are two subsets of types in Rust: scalar and compound
    */

    /*
    Scalar Types: (4 main ones)
     - Integers
     - Floating-point
     - Booleans
     - Chars

     Integer
      - Comes in unsigned (u) and signed (i) -- e.g. u8 or i8

     Floating
      - Comes in f32 and

     Boolean
      - true or false

     Char
      - Unicode Scalar Values
    */

    /*
    Compound Types:
     - Tuples
     - Arrays
    */

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup; // We can use pattern matching to extract values
    let _x = tup.0;
    let _y = tup.1;
    let _z = tup.2; // Or we can extract them out individually

    // Arrays:
    // Arrays have FIXED lengths (Vectors will have variable lengths)
    let arr = [1,2,3];
    let _ele0 = arr[0];

    // This will produce a runtime error and Rust will PANIC
    // -- Panic is the term Rust uses for when a program exits with an error
    // Rust will NOT allow the invalid memory access to continue
    let _ele_invalid = arr[4];
}
