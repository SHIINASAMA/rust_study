//定义结构体
#[derive(Debug)]
struct User{
    name: String,
    account: String,
    nonce: u64,
    active: bool,
}

fn main() {
    //创建结构体实例
    let xiaoMing = User {
        name:String::from("小明"),
        account:String::from("10086"),
        nonce: 10000,
        active: true,
    };

    //修改结构体字段
    let mut xiaoHuang = User {
        name:String::from("小黄"),
        account:String::from("10086"),
        nonce: 10000,
        active: true,
    };
    xiaoHuang.nonce = 100;

    //参数名称和字段名称同名的简写方法
    let name = String::from("小小");
    let account = String::from("100000");
    let nonce = 123;
    let active = true;
    let xiaoXiao = User{
        // name: name,
        // account: account,
        // nonce: nonce,
        // active: active,
        name,
        account,
        nonce,
        active,
    };

    //从其他结构体创建实例
    let otherUser = User {
        name: String::from("otherUser"),
        ..xiaoXiao
    };
    println!("name = {}",otherUser.name);

    //元组结构体
    //1.字段无名称
    //2.圆括号
    struct Point(i32,i32);
    let a = Point(10,10);
    let b = Point(2,2);
    println!("a.x = {},a.y = {}",a.0,a.1);
    println!("b.x = {},b.y = {}",b.0,b.1);

    //没有任何字段的类单元结构体
    struct A{};

    //打印结构体
    println!("xiaoMing = {:?}",xiaoMing);
    println!("xiaoMing = {:#?}",xiaoMing);
}
