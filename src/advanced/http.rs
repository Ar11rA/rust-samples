use std::error::Error;
use std::time;
use tokio::task;

fn get_quote_alpha() -> reqwest::Url {
    let url: String = "http://api.quotable.io/random".to_string();
    reqwest::Url::parse(&url).unwrap()
}

async fn get_quote_beta() -> Result<(), Box<dyn Error + Send + 'static>> {
    reqwest::get(get_quote_alpha()).await;
    println!("Got quote");
    return Ok(());
}

pub async fn method_alpha() -> Result<(), Box<dyn Error>> {
    println!("Fetching quotes by method 1! at {:?}", time::SystemTime::now());
    let _resp1 = reqwest::get(get_quote_alpha()).await?;
    println!("Got quote 1");
    let _resp2 = reqwest::get(get_quote_alpha()).await?;
    println!("Got quote 2");
    let _resp3 = reqwest::get(get_quote_alpha()).await?;
    println!("Got quote 3");
    println!("Done at {:?}", time::SystemTime::now());
    return Ok(());
}

pub async fn method_beta() -> Result<(), Box<dyn Error>> {
    println!("Fetching quotes by method 2! at {:?}", time::SystemTime::now());
    let resp1 = task::spawn(get_quote_beta());
    let resp2 = task::spawn(get_quote_beta());
    let resp3 = task::spawn(get_quote_beta());

    let _ = resp1.await?;
    let _ = resp2.await?;
    let _ = resp3.await?;

    println!("Done at {:?}", time::SystemTime::now());
    return Ok(());
}