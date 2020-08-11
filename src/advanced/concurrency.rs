#[allow(unused_must_use)]
use std::thread;
use std::time;
use std::thread::JoinHandle;

pub fn do_something_async() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for _ in 1..5 {
            print!("A");
            thread::sleep(time::Duration::from_millis(300));
        }
    });

    for _ in 1..5 {
        print!("S");
        thread::sleep(time::Duration::from_millis(500));
    }

    println!();
    let result= handle.join();
    match result {
        Ok(_) => println!("working!"),
        Err(e) => println!("error parsing header: {:?}", e),
    }
}