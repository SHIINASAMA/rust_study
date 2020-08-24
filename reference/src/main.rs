// fn main() {
//     let s1 = gives_ownership();
//     println!("s1 = {}",s1);

//     let mut s2 = String::from("Hello");
//     let s3 = takes_and_gives_back(s2);
//     //此时s2不能再使用
//     println!("s3 = {}",s3);
//     s2 = takes_and_gives_back(s3);
//     //此时s2又能继续使用

//     println!("Hello");
// }

// fn gives_ownership() -> String {
//     let s = String::from("Hello");
//     s
//     //所有权转移
// }

// fn takes_and_gives_back(s:String) -> String {
//     s
//     //接收所有权并转移出去
// }

//& - 引用
fn main(){
    let s1 = String::from("Hello");
    let len = get_len(&s1);
    println!("len = {}",len);
}

fn get_len(s:&String) -> usize{
    s.len()
}