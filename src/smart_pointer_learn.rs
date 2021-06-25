pub fn box_learn() {
    let b = Box::new(6);
    println!("b={}", b);
    use crate::smart_pointer_learn::List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:#?}", list);
}

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
