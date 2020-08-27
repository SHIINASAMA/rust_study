mod factory{
    pub mod produce_refrigerator{
        pub fn produce_re(){
            println!("produce refrigerator");
        }
    }

    mod produce_washing_machine{
        fn produce_wm(){
            println!("produce_washing_machine");
        }
    }
}

fn main() {
    factory::produce_refrigerator::produce_re();
}
