// This is a struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // This is an instance of the User struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let email = user1.email; // we use dot notation to obtain the value of a specific field
    println!("User1 email: {}", email);

    // Notice that the struct above is immutable, we can create mutable structs as well:
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("blah@foo.com"); // and we mutate the fields like so

    fn build_user_from_email_and_username(email: String, username: String) -> User { // We can return a User
         User {
             email, // This is shorthand notation for when variables and fields have the same name
             username,
             active: true,
             sign_in_count: 1
         }
    }

    let user3 = build_user_from_email_and_username(String::from("hello@bye.com"), String::from("ajay"));

    let user4 = User {
        active: false,
        ..user3
    }; // This user will have the same email, username and sign_in_count as user3

    // Tuple Structs:
    // These structs don't have named fields and can be used to create different types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

}
