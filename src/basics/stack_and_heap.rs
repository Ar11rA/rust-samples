use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn set_origin() -> Point {
    return Point {
        x: 0.0,
        y: 0.0,
    };
}

pub fn run() {
    let p1: Point = set_origin(); // set to stack
    let p2: Box<Point> = Box::new(set_origin()); // set to heap (boxing)

    println!("p1 takes up {} size", mem::size_of_val(&p1));
    println!("p2 takes up {} size", mem::size_of_val(&p2));

    let p3: Point = *p2;

    println!("p3 values x {} and y {}", p3.x, p3.y);
}