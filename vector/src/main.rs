fn main() {
    //创建Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    //创建带初始值的Vector
    let mut v = vec![1,2,3];

    //读取元素
    let one: &i32 = &v[0];
    println!("one = {}",one);

    match v.get(1){
        Some(value) => println!("value = {}",value),
        _ => println!("nothing"),
    }

    //不可变遍历
    for i in &v{
        println!("i = {}",i);
    }

    //可变遍历
    for i in &mut v {
        *i += 1;
        println!("i = {}",i);
    }

}
