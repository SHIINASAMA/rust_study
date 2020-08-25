fn main() {
    let some_number = Some(5);
    let some_str = Some(String::from("Hello"));
    let absent_number: Option<i32> = None;

    let x:i32 = 5;
    let y: Option<i32> = Some(5);
    let mut temp = 0;
    match y{
        Some(i) => temp = i,
        None => println!("do nothing")
    }
    let sum = x + temp;
    println!("sum = {}",sum);

    let result = plus_one(y);
    match result{
        Some(i) => println!("result = {}",i),
        None => println!("nothing:"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(x) => Some(x + 1),
    }
}