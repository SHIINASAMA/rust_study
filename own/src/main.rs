fn main() {
    let x:i32 = 1;
    {
        let y:i32 = 2;
        println!("x = {}",x);
        println!("y = {}",y);
    }

    //编译的时候数据类型和大小是固定的，就分配在栈上
    //编译的时候数据类型和大小不是固定的，就分配在堆上
    {
        //此时x分配在堆上
        //let mut x = String::from("Hello");
        //x.push_str(" World");
        let x = String::from("Hello");
        println!("{}",x);
        //变量离开作用域的时候会调用drop方法

        let y = x;
        println!("y = {}",y);
        //这里注释的错误的代码，此时的x已经是无效的了，
        //称之为x丧失了所有权
        //println!("x = {}",x);

        //克隆 clone
        //赋值的同时不会丧失所有权
        let s1 = String::from("Hello");
        //此时s1不会丧失所有权
        let s2 = s1.clone();
        println!("s1 = {}",s2);
    }
    
    //栈上的拷贝
    //copy trait
    let a = 1;
    let b = 2;
    //这里不会报错，因为栈上的拷贝属于值类型
    //实现了copy特征的类型才可以这样使用（深拷贝）
    println!("a = {}, b = {}",a,b);
    
    //常见的具有copy特征的类型
    //整型，浮点，布尔，字符（没有串），元组
    
    //函数和作用域不再赘述

}