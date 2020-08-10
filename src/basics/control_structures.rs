pub fn run_if_else(num: u8) -> bool {
    return if num % 2 == 0 {
        true
    } else {
        false
    };
}

pub fn run_ternary_if_else(num: u8) -> bool {
    let is_even: bool = if num % 2 == 0 { true } else { false };
    return is_even;
}

pub fn run_match(sport: String) -> String {
    let goat: &str = match sport.as_str() {
        "Basketball" => "MJ",
        "Cricket" => "Sachin",
        "Football" => "Messi",
        _ => ":-("
    };
    return String::from(goat);
}

pub fn run_while(limit: u8) {
    let mut i: u8 = 0;
    let mut j: u8 = 1;
    let mut ctr: u8 = 0;
    print!("{} {} ", i, j);
    while ctr < limit - 2 {
        let res: u8 = i + j;
        print!("{} ", res);
        i = j;
        j = res;
        ctr = ctr + 1;
    }
    println!()
}

pub fn run_for() {
    for i in 1..10 {
        print!("{} ", i);
    }
}