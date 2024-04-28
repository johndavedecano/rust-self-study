fn main() {
    let p1: Point<i32> = Point { X: 6, Y: 5 };
    let p2: Point<String> = Point {
        X: String::from("hello"),
        Y: String::from("world"),
    };

    let p3: Point2<String, i32> = Point2 {
        X: String::from("hello"),
        Y: 5,
    };

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
    X: T,
    Y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    X: T,
    Y: U,
}
