use test::stats::Stats;

pub struct  Post{
    state: Option<Box<dyn State>>,
    content:String,
}

impl Post {
    pub fn new() -> Post {
        Post{
            state:Some(Box::new(Draft {})),
            content:String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}


impl  Post {
    fn add_text(&mut self,text :&str) {
        self.content.push_str(text);
    }
}

impl Post {
    pub fn content(&self) ->&str {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
