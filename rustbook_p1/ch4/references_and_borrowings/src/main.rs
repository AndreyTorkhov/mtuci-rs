fn main() {
//     let mut s1 = String::from("hello");

//     // let len = calculate_length(&s1);

//     // println!("length = {}", len);
//     chang(&mut s1);

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // variables r1 and r2 will not be used after this point

    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    // let reference_to_nothing = dangle();
}

// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }

// fn chang(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
