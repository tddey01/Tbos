fn main() {
    //bool
    let is_true: bool = true;
    println!("is_true = {}", is_true);

    let is_false: bool = false;
    println!("is_false = {} is_true = {}", is_false, is_true);

    //char 在rust里面  char是32位的
    let a = 'a';
    println!("a = {}", a);

    let b = '你';
    println!("b = {}", b);

    // i8 i16 i32 i64 u8 u16 u32 u64  f32 f64
    let c: i8 = -111;
    println!("c = {}", c);

    let d: f32 = 0.0009;
    println!("d = {}", d);

    //自适应类型 isize usize
    println!("max = {}", usize::max_value());

    // 数组
    // [Type; size] size也是数组的类型的一部分
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    println!("arr[0]={}", arr[0]);

    show(arr);

    // 元组类型
    let _tup: (i32, f32, char) = (-2, 3.69, '好');
    let tup = (-2, 3.69, '好');
    println!("----------------------");
    println!("i32 = {}", tup.0);
    println!("f32 = {}", tup.1);
    println!("char = {}", tup.2);
    println!("----------------------");

    let (x, y, z) = tup;
    println!("x = {} y = {}   z = {}", x, y, z);
}

fn show(arr: [u32; 5]) {
    println!("-----------");
    for i in &arr {
        println!("{}", i);
    }
}
