// pass by ref
fn pass_by_ref(x: &mut i32) {
    *x += 1;
}

// pass by value
fn pass_by_val(mut _y: i32) {
    _y += 1;
}

// fn methods
struct Rectangle {
    length: u32,
    breadth: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.length * self.breadth;
    }
}

fn functional_rust(limit: u32) {
    let sum:u32 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| *x % 2 == 0)
        .fold(0, |sum, x| sum + x);
    println!("Sum till limit {}", sum);
}

pub fn test_functions() {
    let mut num:i32 = 1;
    println!("Before {}", num);
    pass_by_val(num);
    println!("After pass by val {}", num);
    pass_by_ref(&mut num);
    println!("After pass by ref {}", num);

    let rect = Rectangle{
        length: 4,
        breadth: 5
    };

    println!("Area of rectangle {}", rect.area());
    functional_rust(100);
}