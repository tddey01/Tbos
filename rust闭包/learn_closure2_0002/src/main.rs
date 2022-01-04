// 实现一个缓存 值处理第一次传入的值并保存

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calcuation: T,
    valueL: Option<u32>,
}

impl <T> Cacher<T>
where
T:Fn(u32) ->u32
{
    fn new(calcuation:T)->Cacher<T>{
        Cacher{
            calcuation,
        }
    }
}
fn main() {
    println!("Hello, world!");
}
