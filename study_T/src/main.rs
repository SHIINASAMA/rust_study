fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

//泛型
//PartiaOrd特征：表示可比较的
//Copy特征：表示可复制的
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

//在结构体中使用泛型
#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

//不同泛型
#[derive(Debug)]
struct Point2<T, U>{
    x: T,
    y: U,
}

//在方法中使用泛型
impl<T> Point<T>{
    fn get_x(&self) -> &T{
        &self.x
    }

    fn get_y(&self) -> &T{
        &self.y
    }
}

impl<T, U> Point2<T, U>{
    fn crate_point<V, W>(self,other: Point2<V, W>) -> Point2<T, W>{
        Point2{
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let ml = vec![1,2,3,4,3,4,7,23,3];
    let result = largest(&ml);
    println!("result = {}",result);

    let ml = vec!['a','b','r','c','g','k'];
    let result = largest(&ml);
    println!("result = {}",result);

    let integer = Point{
        x: 1,
        y: 2,
    };
    println!("{:#?}",integer);

    let float = Point
    {
        x: 1.2,
        y: 2.1,
    };
    println!("{:#?}",float);

    let a = Point2{
        x: 1,
        y: 'c',
    };
    println!("{:#?}",a);

    let _x = integer.get_x();
    let _y = integer.get_y();
    println!("x = {},y = {}",_x,_y);

    let p1 = Point2{x:1,y:2.1};
    let p2 = Point2{x:'H',y:12};
    let p3 = p1.crate_point(p2);
    println!("{:#?}",p3);
}