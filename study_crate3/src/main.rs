mod modA{
    #[derive(Debug)]
    pub struct A{
        pub number: i32,
        name: String,
    }

    impl A{
        pub fn new_A() -> A {
            A{
                number: 1,
                name: "A".to_string(),
            }
        }

        pub fn print_a(&self){
            println!("number = {},name = {}",self.number,self.name);
        }
    }

    pub mod mod_B{
        pub fn print_B(){
            println!("B");
        }

        pub mod mod_C{
            pub fn print_C(){
                println!("C");
                super::print_B();
            }
        }
    }
}

use modA::A as A1;
fn main() {
    let a = A1::new_A();
    a.print_a();
}
