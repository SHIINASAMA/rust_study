#[derive(Debug)]
struct Dog{
    name: String,
    weight: f32,
    height: f32,
}

impl Dog{
    //动态方法
    fn get_name(&self) -> &str{
        &(self.name[..])
    }

    fn get_weight(&self) -> f32{
        self.weight
    }

    fn get_height(&self) -> f32{
        self.height
    }
    //静态方法
    fn show() {
        println!("Oh");
    }
}

impl Dog{
    //分成两个部分也可以
}

fn main() {
    let dog = Dog{
        name: String::from("旺财"),
        weight: 100.0,
        height: 70.0,
    };
    println!("dog = {:#?}",dog);

    println!("name = {}",dog.get_name());
    println!("weight = {}",dog.get_weight());
    println!("height = {}",dog.get_height());

    Dog::show();
}
