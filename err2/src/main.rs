use std::io;
use std::io::Read;
use std::fs::File;

// 传播错误
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("Hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s){
//         Ok(_) => Ok(s),
//         Err(error) => Err(error),
//     }
// }

// 简写 1 使用 ？
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("Hello.txt")?;

//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// 简写 2
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("Hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


fn main() {
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("s = {}",s),
        Err(error) => println!("error = {:?}",error),
    }
}
