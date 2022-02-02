//通过Rc<T>允许程序的多个部分之间只读的共享数据，因为相同位置的多个可变引用可能会造成数据竞争和不一致。
use std::rc::Rc;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("conut after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("conut after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("conut after bind c = {}", Rc::strong_count(&a));
    }

    println!("conut after creating  end d = {}", Rc::strong_count(&a));
    println!("Hello, world!");
}
