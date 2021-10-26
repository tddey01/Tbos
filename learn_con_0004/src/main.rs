fn main() {
    // if
    let y = 1;
    if y == 1 {
        println!("y = 1");
    }

    // if -  else
    let y = 2;
    if y == 1 {
        println!("y = 1");
    } else {
        println!("y != 1");
    }

    // if - else if -- else
    let y = 2;
    if y == 1 {
        println!("y = 1");
    } else if y == 2 {
        println!("y == 2");
    } else {
        println!("y != 0");
    }

    // let中使用if bool值判断
    let codition = false;
    let x = if codition { 5 } else { 6 }; // 类型要一至的
    println!("x = {}", x);

    // loop
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter == 10 {
            break;
        }
        // counter = counter + 1;
        counter += 1;
    }

    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter * 2;
        }
    };
    println!("resutl = {}", result);

    //  while 循环
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    // for 循环
    println! {"sssssss"};
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    // for element in arr.iter() {
    for element in &arr {
        println!("element = {}", element);
    }
}
