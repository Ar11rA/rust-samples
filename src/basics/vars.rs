use std::mem;

pub fn add(a: u8, b :u8) -> u8 {
    println!("Value of a {} and size {}", a, mem::size_of_val(&a));
    return a + b;
}

pub fn is_palindrome(a: String) -> bool {
    return a.chars().rev().eq(a.chars());
}

pub fn get_type(n: u16) -> (u16, String) {
    return if n % 2 == 0 {
        (n, String::from("even"))
    } else {
        (n, String::from("odd"))
    }
}