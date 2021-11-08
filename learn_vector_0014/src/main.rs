//1、创建空的vector: Vec<T>
//2、创建包含初始值的vector
//3、丢弃vector
//4、读取元素
//5、更新
//6、遍历
//7、使用枚举
fn main() {
    //1
    let mut v: Vec<i32> = Vec::new();
    // v.push(1);

    // 2 带有初始值得vec
    let v = vec![1, 2, 3, 4];

    // 3  丢弃 vec
    {
        let v = vec![1, 2, 3, 4];
    }

    // 4
    let one: &i32 = &v[0];
    println!("one = {}", one);
    println!("one = {}", &one);
    println!("one = {}", *one);

    //(2） 推荐使用方法
    match v.get(1) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }

    // 5 更新
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    // 6 遍历
    // (1)不可变的遍历
    for i in &v2 {
        println!("i = {}", i);
    }
    // (2)可变的遍历
    println!("-----------------------");
    for i in &mut v2 {
        *i += 1;
        println!("i = {}", i);
    }

    //  7
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    }
    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001),
    ];

    // 8 补充
    let mut y = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("first = {}", first);
}
