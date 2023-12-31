use std::ops::Deref;

fn main() {
    // let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct MyBox<T>(T);

impl<T> MyBox<T>  {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}