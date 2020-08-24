fn main() {
    //if 条件判断
    let y = 2;
    if y == 1{
        println!("y = 1");
    }
    else{
        println!("y != 1");
    }

    //类三元运算
    let con = true;
    let x = if con {
        5
    }else{
        6
    };
    println!("x = {}",x);

    //loop 循环
    let mut x = 0;
    loop{
        x += 1;
        println!("回れ回れ回れ回れ回れ");
        if  x == 10 {
            break;
        }
    }

    //循环表达式
    let x = loop{
        x += 1;
        if x == 20{
            break x*2;
        }
    };
    println!("x = {}",x);

    //while 循环
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}",i);

    //for 循环
    let arr:[u32;5] = [1,2,3,4,5];
    //这里有两种写法
    //for element in arr.iter(){
    for element in &arr{
        println!("element = {}",element);
    }
}
