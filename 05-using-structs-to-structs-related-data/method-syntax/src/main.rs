/*
    Method are similar to functions.
    Unlike functions, methods are defined within the context of a struct (
    or an enum or a trait object), and their first parameter is always `self`,
    which represents the instance of the struct the method is being called on.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
    `impl` (implementation) block for Struct. Everything within this `impl` block will be associated the Struct type.

    `self` in the signature and everywhere within the body, (`self` are similar to `self` in Python language)

    The `self` is actually short for `self: &Self`
*/

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Multiple impl Blocks
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {0}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3); // to call this associated function use the `::` syntax with the structure name
    println!(
        "The area of the sq is {} area pixels",
        sq.area()
    );
}
