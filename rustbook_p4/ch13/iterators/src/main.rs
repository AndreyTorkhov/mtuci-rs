// // use std::vec;

// // fn main() {
// //     // let v1 = vec![1, 3, 2];

// //     // let v1_iter = v1.iter();

// //     // for value in v1_iter {
// //     //     println!("Got: {}", value);
// //     // }

// //     let v1 = vec![1, 2, 3];

// //     // v1.iter().map(|x| x + 1);

// //     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

// //     assert_eq!(v2, vec![2, 3, 4]);
// // }


// #[derive(PartialEq, Debug)]
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn filters_by_size() {
//         let shoes = vec![
//             Shoe {
//                 size: 10,
//                 style: String::from("sneaker"),
//             },
//             Shoe {
//                 size: 13,
//                 style: String::from("sandal"),
//             },
//             Shoe {
//                 size: 10,
//                 style: String::from("boot"),
//             },
//         ];

//         let in_my_size = shoes_in_size(shoes, 10);

//         assert_eq!(
//             in_my_size,
//             vec![
//                 Shoe {
//                     size: 10,
//                     style: String::from("sneaker")
//                 },
//                 Shoe {
//                     size: 10,
//                     style: String::from("boot")
//                 },
//             ]
//         );
//     }
// }
fn main() {
    fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
        let mut v1: i32 = 1;
        let mut v2: i32 = 1;
        for i in 0..a.len() {
            v1 *= a[i];
            v2 *= b[i];
        }
        println!("{}", v1);
        println!("{}", v2);
        
        if v1 > v2 {
            v1 - v2
        } else {
            v2 - v1
        }
    }

    println!("{:?}", find_difference(&[3, 2, 5], &[1, 4, 4]));
}
