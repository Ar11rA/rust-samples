macro_rules! interpolate {
    (
        $(
            $x:expr; [ $( $y:expr ),* ]
        );*
    ) => {
        &[ $($( $x + $y ),*),* ]
    }
}

macro_rules! five_times {
    ($x:expr) => (5 * $x);
}

pub fn run_macros() {
    let arr: &[i32] = interpolate!(
        10; [1, 2, 3];
        20; [4, 5, 6]
    );
    println!("Array: {:?}", arr);
    println!("5 * 2 + 3 = {}", five_times!(2+3));
}