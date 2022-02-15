//  1 模式的是Rust中特殊的语法，模式用来匹配值得结构体

// 2 模式如下内存组成
// （1） 字面值
// （2） 解构的数组、枚举、 结构体或者元组
// （3） 变量
// （4） 通配符
// （5） 占位符

// //  1 match 配置
// match VALUE{
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
//     PATTERN => EXPRESSION,
// }

// fn main() {
//     let a = 1;
//     match a {
//         0 => println!("Zero"),
//         1 => println!("One"),
//         _ => println!("other"),
//     };
//     println!("Hello, world!");
// }

// if let
// fn main() {
//     let color: Option<&str> = None;
//     let is_ok = true;
//     let age: Result<u8, _> = "33".parse();
//
//     if let Some(c) = color {
//         println!("color: {}", c);
//     } else if is_ok {
//         println!("is ok");
//     } else if let Ok(a) = age {
//         if a > 30 {
//             println!("oh, mature man");
//         } else {
//             println!("oh young man");
//         }
//     } else {
//         println!("in else");
//     }
// }

// while let
// 只要模式匹配就一只执行while循环
// fn main() {
//     let mut stack = Vec::new();
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);
//
//     while let Some(top) = stack.pop() {
//         println!("top = {}", top);
//     } //只要匹配Some（value)， 就会一直循环
// }

// // for 循环
// // 在for循环中，模式是直接跟随for关键字的值，例如， for x in y, x就是对应的模式
// fn main() {
//     let v = vec!['a', 'b', 'c'];
//     for (index, value) in v.iter().enumerate() {
//         println!("index = {} , values = {}", index, value);
//     }
// //  此次的模式是（index，value）
// }

// let
// let PATTERN = EXPRESSION
fn main() {
    let (x, y, z) = (1, 2, 3);
}
