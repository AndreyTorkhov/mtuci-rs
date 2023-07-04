// fn main() {
//     let width = 30;
//     let height = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, height)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.height * rectangle.width
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("{:#?}", rect);
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect);
}