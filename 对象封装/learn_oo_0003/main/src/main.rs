use gui::{Button, Screen, SelecBox};

fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelecBox {
                width: 60,
                height: 40,
                option: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
        ],
    };
    s.run();

    println!("Hello, world!");
}
