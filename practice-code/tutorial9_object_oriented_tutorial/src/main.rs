// A test of object-orientation in Rust
// by David Slay

// structs and enums provide the internals for the class

/**
Struct that represents a rectangle.

This is similar to a header in C++, but it only provides information
about the instance variables held in the class.
 */
struct Rectangle {
    width: u32,
    height: u32,
}

// This is an implementation block that gives the Rectangle struct
// some methods.
impl Rectangle {
    //Calculates and returns the area of the Rectangle
    fn area(&self) -> u32 {
        // takes a reference to self

        // now we return the area
        self.width * self.height
    }
}

fn main() {
    let my_rect = Rectangle {
        // note: these can be in any order, as you must name them
        // to create an object
        height: 12,
        width: 24,
    };

    println!("The area of my_rect: {}", my_rect.area());
}
