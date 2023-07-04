struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    // fn width(&self) -> bool {
    //     self.width > 0
    // }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let rec1 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec2 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rec hold rec1 {}", rec.can_hold(&rec1));
    println!("Can rec hold rec2 {}", rec.can_hold(&rec2));

    let sq = Rectangle::square(3);

    // if rec.width() {
    //     println!("The rectangle has a nonzero width; it is {}", rec.width);
    // }

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rec.area()
    // );


}
