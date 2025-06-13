use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T ;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
 
  assert_eq!(5, *(y.deref()));
  assert_eq!(5, x);

  let m = MyBox::new(String::from("Rust"));
  hello(&m);

  drop_pointer();
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

// drop 

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("droping Custom Smart Pointer {}", self.data)
    }
}

fn drop_pointer() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
}
