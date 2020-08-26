fn main() {
    //创建一个空的字符串
    let mut s0 = String::new();
    s0.push_str("Hello");
    println!("s0 = {}",s0);

    //通过字面值创建字符串
    let s1 = String::from("Hello");
    println!("s1 = {}",s1);
    let s1 = "Hello".to_string();
    println!("s1 = {}",s1);

    //编辑字符串
    let mut s2 = "Hello".to_string();
    s2.push_str(" world");
    println!("s2 = {}",s2);
    let ss = "!".to_string();
    s2.push_str(&&ss);
    println!("s2 = {}",s2);

    let mut s2 = String::from("tea");
    //push 添加单字符
    //push_str 添加多字符
    s2.push('m');
    println!("s2 = {}",s2);

    let s1 = "Hello".to_string();
    let s2 = " World".to_string();
    //注意这里 s1 没有所有权了
    let s3 = s1 + &s2;
    println!("s3 = {}",s3);

    //构造字符串
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    let s4 = format!("{} {} {}",s1,s2,s3);
    println!("s4 = {}",s4);

    let s4 = "你好".to_string();
    //let s5 = s4[0];
    println!("s4.len = {}",s4.len());
    let s5 = &s4[0..3];
    println!("s5 = {}",s5);

    //遍历
    for c in s4.chars(){
        println!("c = {}",c);
    }
    for b in s4.bytes(){
        println!("b = {}",b);
    }
}
