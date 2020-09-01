fn main() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    // 到目前为止，不会对v1产生任何影响
    // for val in v1_iter{
    //     println!("val = {}",val);
    // }
    // 迭代器都实现了iterator特征

    if let Some(v) = v1_iter.next(){
        println!("v = {}",v);
    };
    if let Some(v) = v1_iter.next(){
        println!("v = {}",v);
    };
    if let Some(v) = v1_iter.next(){
        println!("v = {}",v);
    };
    if let Some(v) = v1_iter.next(){
        println!("v = {}",v);
    }else{
        println!("Done");
    }

    // 迭代可变引用
    let mut v2 = vec![1,2,3,4,5];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next(){
        *v = 3;
    };
    println!("{:?}",v2);

    // 消费适配器
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("total = {}",total);

    // 迭代适配器
    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("{:?}",v2);
}

// trait Iterator{
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     // type Item 和 Self::Item 这种用法叫做定义 trait 的关联类型
// }