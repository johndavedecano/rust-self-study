use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1.3, y: 4.4 };
    let p2 = Point { x: 3.7, y: 1.4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}
