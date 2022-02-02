use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // todo!()
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 将MyBox变为&String  在将 &String 的解引用，改变为字符串 slice， &str

    println!("Hello, world!");
}

fn hello(name: &str) {
    println!("hello  {}", name);
}
//解引用多态与可变性交互：
//（1）当T：Deref<Target=U>时，从&T到&U
// (2)当T：DerefMut<Target=U>时，从&mut T 到&mut U
//（3）当T：Deref<Target=U>时，从&mut T到&U