fn main() {
    { // We'll begin with what we ended with in 5.2
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect = Rectangle { width: 30, height: 50 };

        println!(
            "The area of the rectangle is {:#?} square pixels.",
            area(&rect)
        );
    }

    // This area function only takes in Rectangles, in this case, we can make it a method
    // of Rectangle
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        // We can define area in the context of Rectangle by encapsulating it in an implementation
        // block associated to Rectangle
        impl Rectangle {
            fn area(&self /* We know what the type is, so we don't need to include it */) -> u32 {
                // notice we use &self, we don't want ownership here.
                // If we wanted to modify self we could use &mut self.
                // Rarely will you see just self (i.e. ownership is taken) unless we want to prevent the owner from
                // the original struct after calling the method.
                self.width * self.height
            }
        }

        let rect = Rectangle { width: 30, height: 50 };

        println!(
            "The area of the rectangle is {:#?} square pixels.",
            rect.area()
        );
    }

    // Methods with multiple arguments and associated functions
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn is_larger(&self, other: &Rectangle) -> bool { // We can add more arguments as we would expect
                self.area() > other.area()
            }
        }

        // Notice we can have multiple impl blocks for the same struct, this is valid code, just unnecessary.
        impl Rectangle {
            // This function doesn't use self, but it associated with the struct
            fn square(length: u32) -> Rectangle {
                Rectangle { width: length, height: length }
            }
        }

        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 40, height: 50 };
        let rect3 = Rectangle::square(3); // We can call square() by using the namespace operator

        println!("rect3 is {:#?}", rect3);
        println!("rect1 is larger than rect2: {}", rect1.is_larger(&rect2));
        println!("rect1 is larger than rect3: {}", rect1.is_larger(&rect3));
    }
}
