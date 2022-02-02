//1、Drop trait类似于其它语言中的析构函数，当值离开作用域的时候执行此函数的代码。
//

struct Dog {
    name: String,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog  {} leave", self.name);
    }
}

fn main() {
    let _a = Dog {
        name: String::from("wangcai"),
    };
    {
        let _b = Dog {
            name: String::from("laoda"),
        };
        println!("+++++++++++++++++++++++++++");
    }
    println!("============================");
    println!("Hello, world!");
}
