// fn main() {

//     let _x = 2.0;

//     let _y: f32 = 3.0;

//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;

//     let t = true;

//     let f: bool = false;

//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';

//     // Кортеж
//     let typ: (i32, f64, u8) = (500, 6.4, 1);

//     let (x, y, z) = typ;

//     println!("The value of y: {}", y);

//     let five_hundred = typ.0;

//     let six_point_four = typ.1;

//     let one = typ.2;

//     // Массивы
//     let a = [1, 2, 3, 4, 5];

//     let a = [3; 5];

//     let first = a[0];
//     let second = a[1];

// }
use std::io;
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");
    
    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}