#[derive(Debug)]
struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

// 方法 rust 独有方法编写
impl Dog {
    fn get_name(&self) -> &str {
        &(self.name[..])
    }
    fn get_weight(&self) -> f32 {
        self.weight
    }
    // fn get_height(&self) -> f32 {
    //     self.height
    // }
    fn show() {
        println!("oh oh oh");
    }
}

// 语句块  可以把一个方法 分开多个写 是可以的
impl Dog {
    fn get_height(&self) -> f32 {
        self.height
    }
}
fn main() {
    let dog = Dog {
        name: String::from("wangcai"),
        weight: 100.5,
        height: 70.5,
    };

    println!("dog = {:#?}", dog); // {:#?} 格式化打印输出
    println!("dog name = {}", dog.get_name());
    println!("dog weight = {:?}", dog.get_weight());
    println!("dog height = {}", dog.get_height());

    Dog::show();
}
