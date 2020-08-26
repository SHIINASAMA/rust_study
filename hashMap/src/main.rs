//导入模块 
use std::collections::HashMap;

fn main() {
    // 创建哈希表
    let mut scores: HashMap<String,i32> = HashMap::new();
    //插入时键不存在就插入，存在就覆盖
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Red"),20);

    // 第二种方式
    // let keys = vec![String::from("Blue"),String::from("Red")];
    // let values = vec![10,20];
    // let scores: HashMap<_ ,_> = keys.iter().zip(values.iter()).collect();

    let key = "Blue".to_string();
    if let Some(v) = scores.get(&key){
        println!("v = {}",v);
    }

    //遍历
    for (key,value) in &scores{
        println!("key = {},value = {}",key,value);
    }

    //键不存在时才插入
    scores.entry(String::from("Red")).or_insert(3);
}
