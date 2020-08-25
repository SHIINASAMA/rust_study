fn main() {
    //枚举的定义--CL模式
    enum IpAddressKind{
        V4,
        V6,
    };

    struct IpAddr{
        king: IpAddressKind,
        address: String,
    };

    let i1 = IpAddr {
        king: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let i2 = IpAddr {
        king: IpAddressKind::V6,
        address: String::from("127.0.0.1"),
    };

    //枚举的定义--RSL模式
    enum IpAddr2{
        V4(String),
        V6(String),
    };

    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("127.0.0.1"));

    //类型可以是不同的(元组)
    enum IpAddr3{
        V4(u8,u8,u8,u8),
        V6(String),
    };

    let i1 = IpAddr3::V4(127,0,0,1);
    let i2 = IpAddr3::V6(String::from("127.0.0.1"));

    let quit = Message::Quit;
    quit.prin();

    let mo = Message::Move{x:1,y:2};
    mo.prin();
}

//经典用法
enum Message{
    Quit,
    //Move属于结构体
    Move{x: i32,y: i32},
    Write(String),
    Change(i32,i32,i32),
}

impl Message{
    fn prin(&self){
        //* 解引用，match类似于switch
        match *self{
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("x = {},y = {}",x,y),
            Message::Change(a,b,c) => println!("a = {},b = {},c ={}",a,b,c),
            //其他的
            _ => println!("Writr"),
        }
    }
}