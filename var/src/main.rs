const MAX_SIZE:u32 = 100000;
fn main() {
    //u32 就是 unsigned int32
    //mut 就是可变的
    //变量要求有初始值
    //并且类型是可以自动推导的
    let mut a:u32 = 1;
    //println! 是宏
    println!("a = {}",a);
    a = 1 + 1;
    println!("a = {}",a);

    //f32 32位浮点数
    //变量是运行重定义的(隐藏)
    let a:f32 = 1.1;
    println!("a = {}",a);

    //使用常量
    println!("MAX_SIZE = {}",MAX_SIZE);
}
