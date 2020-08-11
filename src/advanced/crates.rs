// use random package
extern crate rand;
use rand::Rng;

pub fn generate_random_number() {
    let mut range = rand::thread_rng();
    let num:u32 = range.gen();
    println!("{}", num);
}