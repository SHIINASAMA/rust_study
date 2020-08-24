fn main() {
    other_function();

    other_function1(1);

    let a = other_function2(1, 1);
    println!("a = {}",a);

    //表达式会计算一些值
    let y = {
        let x = 1;
        x + 12
    };
    println!("y = {}",y);
}

fn other_function(){
    println!("This is a other function");
}

//定义函数的参数是必须指定类型
fn other_function1(a:i32){
    println!("a = {}",a);
}

fn other_function2(a:i32,b:i32) -> i32{
    //return a + b;
    //注意下面这种返回方式
    a + b
}