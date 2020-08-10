pub fn run_strings() {
    let str: &str = "hello strings in rust!";
    for c in str.chars().rev() {
        print!("{} ", c);
    }
    println!();

    let name = "Yoda";
    let greeting = format!("hello {}!", name);
    println!("{}", greeting);
}
