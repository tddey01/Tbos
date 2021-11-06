//1、字符串slice是String中一部分值的引用
//2、字面值就是slice
//3、其它类型slice

fn main() {
    let s = String::from("hello world");

    // let h = &s[0..5];
    // let h = &s[0..=4];
    // let h = &s[..=4];
    let h = &s[..5];
    println!("h= {}", h);

    // let w = &s[6..11];
    // let w = &s[6..=11];
    // let w = &s[6..];
    let w = &s[..];
    println!("w= {}", w);

    let s3 = "hh"; // &str 类型指针 不可变的引用

    let a = [1, 2, 3, 4];
    let sss = &a[1..3];
    println!("s3 = {}",sss[1]);
    println!("s3 = {}",sss[0]);
    println!("s3 = {}",sss.len());




}

