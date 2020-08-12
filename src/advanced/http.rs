use std::error::Error;
use std::time;
use serde::Deserialize;
use tokio::task;
use reqwest::Response;

#[derive(Debug, Deserialize)]
struct Quote {
    _id: String,
    tags: Vec<String>,
    content: String,
    author: String,
    length: u32
}

fn get_quote_alpha() -> reqwest::Url {
    let url: String = "http://api.quotable.io/random".to_string();
    reqwest::Url::parse(&url).unwrap()
}

async fn get_quote_beta() -> Result<Response, reqwest::Error> {
    let resp: Result<Response, reqwest::Error> = reqwest::get(get_quote_alpha()).await;
    println!("Got some response as: {:?}", resp);
    return resp;
}

pub async fn method_alpha() -> Result<(), Box<dyn Error>> {
    println!("Fetching quotes by method 1! at {:?}", time::SystemTime::now());
    let resp1: Response = reqwest::get(get_quote_alpha()).await?;
    let q1: Quote = resp1.json().await?;
    println!("Got quote 1 {:?}", q1);
    let resp2: Response = reqwest::get(get_quote_alpha()).await?;
    let q2: Quote = resp2.json().await?;
    println!("Got quote 2 {:?}", q2);
    let resp3: Response = reqwest::get(get_quote_alpha()).await?;
    let q3: Quote = resp3.json().await?;
    println!("Got quote 3 {:?}", q3);
    println!("Done at {:?}", time::SystemTime::now());
    return Ok(());
}

pub async fn method_beta() -> Result<(), Box<dyn Error>> {
    println!("Fetching quotes by method 2! at {:?}", time::SystemTime::now());
    let resp1 = task::spawn(get_quote_beta());
    let resp2 = task::spawn(get_quote_beta());
    let resp3 = task::spawn(get_quote_beta());

    let q1: Quote = resp1.await??.json().await?;
    println!("Got quote 1 {:?}", q1);
    let q2: Quote = resp2.await??.json().await?;
    println!("Got quote 2 {:?}", q2);
    let q3: Quote = resp3.await??.json().await?;
    println!("Got quote 3 {:?}", q3);

    println!("Done at {:?}", time::SystemTime::now());
    return Ok(());
}

#[derive(Debug, Deserialize)]
struct Network {
    origin: String
}

pub async fn deserialize_simple() -> Result<bool, Box<dyn Error>> {
    let resp: Network = reqwest::get("https://httpbin.org/ip")
        .await?
        .json()
        .await?;
    println!("IP details: {:?}", resp);
    return Ok(true);
}