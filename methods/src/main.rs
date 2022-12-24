#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Quadrilateral {
    width: u32,
    height: u32,
}

impl Quadrilateral {
    fn isSquare(&self) -> bool {
        self.width == self.height
    }

    fn isRectangle(&self) -> bool {
        self.width != self.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn getSquare(width: u32) -> Quadrilateral {
        Quadrilateral {
            width: width,
            height: width,
        }
    }

    fn getRectangle(width: u32, height: u32) -> Quadrilateral {
        Quadrilateral {
            width: width,
            height: height,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
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

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
