// 每一个引用都有生命周期，也就是引用保持有效的作用域。
// 大部分时候生命周期是隐含并可以推断的，正如大部分类型可以推断一样。
// 生命周期的主要目标是避免悬垂引用，编译器使用借用检查器来检查的生命周期是否有效。

fn main() {
    // 错误的例子
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // 此时x失效，r变成了悬垂指针

    let s1 = String::from("abcde");
    let s2 = String::from("asdfghjkl");
    let r = longest(s1.as_str(), s2.as_str());

    let n = "Hello".to_string();
    let a = A{name: &n};
    println!("{:#?}",a);
}

// 函数中的生命周期
// 此行错误
// fn longest(x: &str,y: &str) -> &str{
// 此行正确
// fn longest<'a>(x: &'a str,y: &'a str) -> &'a str{
fn longest<'c>(x: &'c str,y: &'c str) -> &'c str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn get_str<'a>(x: &'a str,y: &str) -> &'a str{
    x
}

// 错误例子
// fn a_str<'a>(x: &str, y: &str) -> &'a str{
//     let r = String::from("abc");
//     r.as_str()
// }


// 结构体中的生命周期
#[derive(Debug)]
struct A<'a>{
    name: &'a str,
}