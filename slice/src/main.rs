fn main() {
    //1.字符串slice是String中的一部分值引用
    let s = String::from("Hello world");
    //let h = &s[0..5];
    //let h = &s[0..=4];
    let h = &s[..=4];
    println!("s = {}",s);
    println!("h = {}",h);

    //let w = &s[6..11];
    //let w = &s[6..=10];
    let w = &s[6..];
    //全集
    //let w = &s[..];
    println!("w = {}",w);

    //2.字面值就是slice
    //3.其他类型slice
    let a = [1,2,3,4];
    let sss = &a[1..3];
    println!("sss = {}{}",sss[0],sss[1]);
    println!("sss.len = {}",sss.len());
}
