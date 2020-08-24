fn main() {
    let is_true :bool = true;
    println!("is_true = {}",is_true);

    let is_false = false;
    println!("is_false = {}",is_false);

    //char在C、CPP中是8位的，在Rust中是32位的
    let a = 'a';
    println!("a = {}",a);
    let a = '薰';
    println!("a = {}",a);

    //i8,i16,i32,i64
    //u8,u16,u32,u64
    //f32,f64

    //自适应类型
    //随着平台的改变而改变
    println!("max = {}",isize::max_value());

    //数组
    let arr:[u32;5] = [1,2,3,4,5];
    println!("a[1] = {}",arr[1]);

    //元组
    let tup :(i32,u32,f32) = (-3,3,3.0);
    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);
    //元组的拆解
    let (x,y,z) = tup;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
}
