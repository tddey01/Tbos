// fn main() {
//     let s1 = givers_ownership();
//     println!("s1 = {}", s1);

//     let mut s2 = String::from("hello");
//     let s3 = takes_and_gives_back(s2);
//     s2 = takes_and_gives_back(s3);

//     println!("s2 = {}", s2); //value borrowed here after move
//     println!("Hello, world!");
// }

// fn givers_ownership() -> String {
//     let s = String::from("hello");
//     s
// }

// fn takes_and_gives_back(s: String) -> String {
//     s
// }

//引用: 用法&,
//让我们创建一个指向值的应用，但是并不拥有它，因为不拥有这个值，所以，当引用离开其值指向的作用域后也不会被丢弃
//借用:&mut

fn main() {
    let mut s1 = String::from("hello");
    // let s = &s1;
    // let len = calcute_length(&s);
    // println!("s1 = {}", s1);
    // println!("len = {}", len);

    // let ms  = &mut s1;
    // modify_s(ms);
    // // modify_s(&mut s1);
    // println!("s1 = {}", s1);

    let r1 = &s1;
    let r2 = &s1;
    println!("{} {} ", r1, r2);
    let r3 = &mut s1;
    r3.push_str(", world");
}

// fn calcute_length(s: &String) -> usize {
//     s.len()
// }

// fn modify_s(s: &mut String) {
//     s.push_str(", world");
// }
