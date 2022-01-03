// 函数中的生命周期
// fn longest(x: &str, y: &str) -> &str {
// fn longest(x: &'a str, y: &'a str) ->  &'a str  {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_str<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// fn a_st<'a>(x: &'a str, y: &'a str) -> &'a str {  //error 错误例子
//     let r = String::from("abc");
//     r.as_str()
// }

fn main() {
    let s1 = String::from("abcde");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);

    let ss = get_str(s1.as_str(), s2.as_str());
    // let ss2 = as_str(s1.as_str(), s2.as_str());
    println!("ss = {}", ss);
    println!("Hello, world!");
}
