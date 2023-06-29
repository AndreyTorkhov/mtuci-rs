fn main() {
    // let s = "hello";

    // {                           // s is not valid here, it’s not yet declared
    //     let s = "hello";  // s is valid from this point forward

    //     // do stuff with s
    // }                           // this scope is now over, and s is no longer valid

    // let mut s = String::from("hellow");
    
    // s.push_str(", world");

    // println!("{}", s);

    // {
    //     let s = String::from("hello"); // s is valid from this point forward

    //     // do stuff with s
    // }                                  // this scope is now over, and s is no
    //                                    // longer valid


    // let x = 5;
    // let y = x;

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s = String::from("hello");

    // takes_ownership(&s);

    // println!("{}", s);

    // let x = 5;

    // println!("{}", x);

    // makes_copy(x);

    // let s1 = gives_ownership();

    // let s2 = String::from("");

    // let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("{}, {}", s2, len);

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
// fn gives_ownership() -> String {
//     let some_string = String::from("yours");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }