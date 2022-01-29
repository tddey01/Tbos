//1、迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。
//2、创建迭代器：迭代器是惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
//3、每个迭代器都实现了iterator trait, iterator trait定义在标准库中：
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter(); //到目前为止 不会对这个v1产生任何影响
    for val in v1_iter {
        println!("{}", val);
    }

    println!("Hello, world!");
}
