// fn main() {
//     let mut user = User {
//         active: true,
//         username: String::from("Илья"),
//         email: String::from("helium@internet.ru"),
//         sign_in_count: 1,
//     };

//     // let user2 = User {
//     //     active: user.active,
//     //     username: user.username,
//     //     email: String::from("Привет, как дела?"),
//     //     sign_in_count: user.sign_in_count,
//     // };

//     let user3 = User {
//         email: String::from("Вот так бывает!"),
//         ..user
//     };

//     user.email = String::from("Фантазёр ты меня называла!!!");
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user = User {
//         email : "123",
//         username : "123",
//         active: true,
//         sign_in_count: 1,
//     }
// }
