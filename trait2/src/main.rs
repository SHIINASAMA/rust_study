trait GetName{
    fn get_name(&self) -> &String;
}

trait GetAge{
    fn get_age(&self) -> u32;
}

// trait_bound写法
// fn print_infomation<T: GetName + GetAge>(item: T){
//     println!("name = {},age = {}",item.get_name(),item.get_age());
// }

// 写法二
fn print_infomation<T>(item: T)where T: GetName + GetAge{
    println!("name = {},age = {}",item.get_name(),item.get_age());
}

#[derive(Debug)]
pub struct Student{
    pub name: String,
    pub age: u32,
}

impl GetName for Student{
    fn get_name(&self) -> &String{
        &self.name
    }
}

impl GetAge for Student{
    fn get_age(&self) -> u32{
        *&self.age
    }
}

//返回具有特征的对象
fn produce_item_whit_age() -> impl GetName + GetAge{
    Student{
        name: String::from("xiaoming"),
        age: 15,
    }
}

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 10};
    print_infomation(s);

    let s = produce_item_whit_age();
    print_infomation(s);
}
