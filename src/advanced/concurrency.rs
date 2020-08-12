#[allow(unused_must_use)]
use std::thread;
use std::time;
use std::thread::JoinHandle;
use std::error::Error;

// threads
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
        Ok(r) => println!("working with {:?}!", r),
        Err(e) => println!("error parsing header: {:?}", e),
    }

    // above match equivalent to
    /* let r = handle.join().unwrap();
    println!("response is {:?}", r); */
}

// basic async await
async fn greet_async(name: String) -> Result<String, Box<dyn Error>> {
    println!("Hello {}", name);
    return Result::Ok("yo".to_string());
}

pub async fn greet() {
    let resp = greet_async("Yoda".to_string()).await;
    println!("Got {:?}", resp.unwrap());
}