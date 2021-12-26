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

fn main() {
    let s1 = String::from("abcde");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(), s2.as_str());
    println!("r = {}", r);
    println!("Hello, world!");
}
