fn main() {

    { // Program 1
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    { // Program 2 - Refactoring with tuples
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        let rect = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect)
        );
    }

    { // Program 3 - Refactoring with structs
        struct Rectangle {
            width: u32,
            height: u32
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect = Rectangle {
            width: 30,
            height: 50
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect)
        );
    }

    { // Program 4 - Refactoring with structs and adding functionality with derived traits
      /*
        We want something that can do:
        println!("rect is: {}", rect);
        which as of Program 3 will throw the error "the trait bound `Rectangle: std::fmt::Display` is not satisfied"
      */
        #[derive(Debug)] // This allows us to "opt-in" to printing out structs
        struct Rectangle {
            width: u32,
            height: u32
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        let rect = Rectangle {
            width: 30,
            height: 50
        };

        // Outputs: rect1 is Rectangle { width: 30, height: 50 }
        println!(
            "The area of the rectangle is {:?} square pixels.",
            area(&rect)
        );

        // Outputs:
        /*
            rect1 is Rectangle {
                width: 30,
                height: 50
            }
        */
        println!(
            "The area of the rectangle is {:#?} square pixels.",
            area(&rect)
        );
    }
}
