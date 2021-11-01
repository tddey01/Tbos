fn takes_ownership(some_string: String)  -> String {
    println!("{}", some_string);
    some_string
}

fn makes_copy(i: i32) {
    println!("i = {}", i);
}

fn main() {
    let s = String::from("hello");
    let s1 = takes_ownership(s);


    println!("sq = {}",s1);
    let x = 5;
    makes_copy(x);
}


