//1、HashMap<K, V>
//2、创建HashMap
//3、读取
//4、遍历
//5、更新

use std::collections::HashMap;
fn main() {
    //创建hashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 19);
    scores.insert(String::from("Red"), 30);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    // 读取
    let k = String::from("Blue");
    // v = 10
    if let Some(v) = scores.get(&k) {
        //get 返回的是一个Option
        println!(" v = {}", v);
    };
    let k = String::from("Yellow");
    let v = scores.get(&k);
    match v {
        Some(value) => println!("value = {}", value),
        None => println!("None"),
    };

    // 遍历
    println!("======================");
    // 遍历 会以任意的顺序遍历出来
    for (key, values) in &scores {
        println!("{} = {}", key, values);
    }

    // 直接插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.insert(String::from("one"), 3);
    println!("ss {:?}", ss);

    // 键值不存的时候插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.entry(String::from("one")).or_insert(3);
    println!("ss {:?}", ss);

    // 根据旧值类更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);
}
