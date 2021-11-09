pub mod factory {
    pub mod produec_refrigerator {
        pub fn produce_re() {
            println!("produce refrigerator!");
        }
    }
    pub mod produce_washing_machine {
        pub fn produce_washing_maching() {
            println!("produce washing machine!");
        }
    }
}

fn main() {
    factory::produec_refrigerator::produce_re();
    factory::produce_washing_machine::produce_washing_maching();
}
