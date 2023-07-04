// fn main() {
//     let four = IpaddrKind::V4;
//     let six = IpaddrKind::V6;

//     route(four);
//     route(six);
// }

// enum IpaddrKind {
//     V4,
//     V6
// }

// fn route(ip_kind: IpaddrKind) {}

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     }
// }


// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

// enum Option<T> {
//     None,
//     Some(T),
// }