struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog  {} leave", self.name);
    }
}

fn main() {
    let a = Dog {
        name: String::from("wangcai"),
    };

    let b = Dog {
        name: String::from("laoda"),
    };
    drop(b);
    drop(a);
    println!("============================");
    println!("Hello, world!");
}
 // 提前释放对象