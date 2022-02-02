struct Counter {
    conut: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { conut: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.conut += 1;
        if self.conut < 6 {
            Some(self.conut)
        } else {
            None
        }
    }
}

fn main() {
    let mut conuter = Counter::new();
    for i in 0..6 {
        if let Some(v) = conuter.next() {
            println!("i = {}  v = {}", i, v);
        } else {
            println!("i = {} at end", i);
            break;
        }
    }
    println!("Hello, world!");
}
