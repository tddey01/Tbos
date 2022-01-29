//1、迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑。
//2、创建迭代器：迭代器是惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
//3、每个迭代器都实现了iterator trait, iterator trait定义在标准库中：
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // type Item和Self::Item 这种用发叫做定义trait的关联类型
}

// next 是Iterator被要求实现的唯一个方法， next一次返回一个元素， 当迭代器结束时候， 返回None
fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); //到目前为止 不会对这个v1产生任何影响
                                 // for val in v1_iter {
                                 //     println!("{}", val);
                                 // }
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    };
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    };
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    };
    if let Some(v) = v1_iter.next() {
        println!("v = {}", v);
    };

    // ========================迭代可变引用=========================================
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = 3;
    }
    println!("v2 = {:?}", v2);
    // = =====================小费适配器============================

    let v3 = vec![1, 2, 3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum(); // 调用小费适配器sum求和
    println!("v3 = {}", total);

    // = =====================迭代适配器============================
    let v4 = vec![1, 2, 3];
    println!("v3 = {:?}", v4);
    let v5: Vec<_> = v4.iter().map(|x| x + 1).collect();
    println!("v3 = {:?}", v5);

    // = =================================================
    println!("++++++++++++");
    let v1 = vec![1, 12, 3, 45];
    println!("v1 = {:?}", v1);

    let v2: Vec<_> = v1.into_iter().filter(|x| *x > 5).collect();
    println!("v2 = {:?}", v2);
    println!("Hello, world!");
}
