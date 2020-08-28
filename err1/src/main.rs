//可恢复错误：通常指向用户报告错误和重试操作，是合理的情况。使用Result<T,E>来实现
//不可恢复错误：是bug的同义词，如数组访问越界，通过panic!来实现
use std::fs::File;
fn main() {
    // let f = File::open("Hello.txt");
    // let r = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("error:{:?}",error),
    // };

    //简写1
    //let f = File::open("Hello.txt").unwrap();

    //简写2
    let f = File::open("Hello.txt").expect("打开文件失败");
}
