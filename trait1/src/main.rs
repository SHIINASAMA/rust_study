//定义trait
pub trait GetInfomation{
    fn get_name(&self) -> &String;
    fn get_age(&self) -> &u32;
}

//实现trait
#[derive(Debug)]
pub struct Student{
    pub name: String,
    pub age: u32,
}
impl GetInfomation for Student{
    fn get_name(&self) -> &String{
        &self.name
    }
    fn get_age(&self) -> &u32{
        &self.age
    }
}

#[derive(Debug)]
pub struct Teacher{
    pub name: String,
    pub age: u32,
    pub subject: String,
}
impl GetInfomation for Teacher{
    fn get_name(&self) -> &String{
        &self.name
    }
    fn get_age(&self) -> &u32{
        &self.age
    }
}

fn print_info(item: impl GetInfomation){
    println!("item: name = {},age = {}",item.get_name(),item.get_age());
}

fn main() {
    let s = Student{name: "xiaoming".to_string(), age: 10};
    let t = Teacher{name: "xiaohong".to_string(), age: 30,subject:"Rust".to_string()};
    //println!("student: name = {}, age = {}",s.get_name(),s.get_age());
    //println!("teacher: name = {}, age = {}",t.get_name(),t.get_age());

    print_info(s);
    print_info(t);
}
