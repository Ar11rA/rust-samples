use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Point<T, S> {
    x: T,
    y: S,
}

pub fn generic_ops() {
    let p: Point<f64, i64> = Point {
        x: 4.0,
        y: 1,
    };
    println!("Generic {:?}", p);
}

pub fn vector_ops() {
    let mut list: Vec<u8> = Vec::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("Vector {:?}", list);
    match list.get(1) {
        Some(elem) => println!("Found {}", elem),
        None => println!("Not found!")
    }
    while let Some(elem) = list.pop() {
        println!("Popped {}", elem);
    }
}

pub fn hashmap_ops() {
    let mut frequency1: HashMap<i32, i32> = HashMap::new();
    let nums = vec![1, 2, 1, 1, 2];
    for num in nums.iter() {
        *frequency1.entry(*num).or_insert(0) += 1;
    }
    println!("Frequency 1 map: {:?}", frequency1);
    let frequency2: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut acc, current| {
        *acc.entry(*current).or_insert(0) += 1;
        acc
    });
    println!("Frequency 2 map: {:?}", frequency2);
}

pub fn hashset_ops() {
    let mut set:HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    println!("{:?}", set);
    println!("{}", set.contains(&3));
}