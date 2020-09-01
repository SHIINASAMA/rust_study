fn main() {
    // 定义并使用闭包
    let use_closure= || {
        println!("This is a closure.")
    };
    use_closure();

    // 捕捉环境中的变量
    let i = 1;
    let exe = |x| x + 1;
    let r = exe(5);
    println!("r = {}",r);

    println!("Hello, world!");
}

// 闭包与普通函数
fn add_one_v1(x: u32) -> u32{
    x + 1
}
// let add_one_v2 = |x: u32| -> u32{
//     x + 1
// };
// 简写
// let add_one_v3 = |x| {x + 1};
// let add_one_v4 = |x| x + 1;