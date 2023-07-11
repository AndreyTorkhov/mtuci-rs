pub struct MyVec<T> {
    data: Vec<T>,
}

impl<T> MyVec<T>{

    pub fn new() -> Self{
        MyVec{
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self{
        MyVec{
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len(){
            Some(self.data.remove(index))
        } else {
            None
        }
        
    }

    pub fn get(&self, index: usize) -> Option<&T>{
        self.data.get(index) as Option<&T>
    }

    pub fn resize(&mut self, new_size : usize , value: T) where T:Clone{
        self.data.resize(new_size, value)
    }

}

// ПРОВЕРКА РАБОТОСПОСОБНОСТИ
fn main() {
    let mut vec: MyVec<i32> = MyVec::new();
    let mut vec_cap: MyVec<i32> = MyVec::with_capacity(5);
    
    
    vec.push(4);
    vec.push(2);
    vec.push(6);
    vec.push(8);
    vec.push(1);
    vec.push(9);
    vec.push(3);

    println!("{:?}", vec.pop()); //3
    
    println!("{:?}", vec.pop()); //9
    
    println!("{:?}", vec.remove(0)); //4

    println!("{:?}", vec.get(0)); //2

    println!("{:?}", vec.get(10));  // None

    vec_cap.resize(8, 1);
    println!("{:?}", vec_cap.pop()); //1 
}