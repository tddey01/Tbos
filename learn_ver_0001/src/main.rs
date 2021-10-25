const MAX_POINTS: u32 = 100000;

fn main() {
    // 变量 定义
    // d定义变量用let  如果变量没有用mut 那么是不可变的
    // let name：type  
    let a = 1;
    println!("a = {}", a);

    let mut b: u32 = 1; //定义类型变量  可变性变量 不可变性变量
    println!("b = {}", b);

    b = 2;
    println!("edit var {}", b);

    // 隐藏
    let b: f32 = 1.1;
    println!("b = {}", b);

    // 常量
    println!("MAX_POINTS = {}", MAX_POINTS);
}
