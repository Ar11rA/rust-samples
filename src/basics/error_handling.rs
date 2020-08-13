use std::fs::File;

// unrecoverable errors
#[allow(dead_code)]
fn panic_error_example() {
    let v = vec![1, 2, 3];
    v[99];
}

// recoverable errors
#[allow(dead_code)]
fn rec_error_example() {
    let f = File::open("hello.txt");
    let _: File = match f {
        Ok(res) => res,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}