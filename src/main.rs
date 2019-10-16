#![deny(warnings)]

extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate serde;
// #[macro_use] extern crate serde_derive;
extern crate serde_json;

// use futures::Future;
use serde_json::Value;
use reqwest::{Client, Response};
use std::str::FromStr;
use std::env;
use std::{thread, time};

// use std::collections::LinkedList;
// use std::vec::Vec;
use serde::export::Vec;

fn get_most_recent_price(equity: &str ) -> f32 {
    let res = fetch(equity);
    match res {
    Ok(v) => {
    let mut tmp_prices = v; 
    let length = tmp_prices.len();
     println!("The most recent price is {:?}",  tmp_prices[length - 1]);
    // println!("{:?}", tmp_prices[length - 1]);
    return tmp_prices[length - 1]; // get earliest price stock 
    },
    Err(_e) =>  return 0.00,
}
}

//called when a new stock price is available, max every minute
//returns true for buy signal false otherwise
fn k_means(prices: &mut Vec<f32>) -> bool{
    let mut thirty_sum: f32 = 0.0;
    let mut x = 89;
    while x >= 60{
        println!("price at:{} is {}", x, thirty_sum);
        thirty_sum += prices[x];
        x = x - 1;
    }
    
    let mut nintey_sum: f32 = 0.0;
    let mut k = 0;
    while k < 90 {
        nintey_sum += prices[k];
        k = k + 1;
    }
    println!("thirty sum: {}", thirty_sum);
    println!("nintey sum: {}", nintey_sum);
    
    let thirty_day_average: f32 = thirty_sum / 30.0;
    let nintey_day_average: f32 = nintey_sum / 90.0;
    println!("thirty day average: {}", thirty_day_average);
    println!("nintey day average: {}", nintey_day_average);
    
    if thirty_day_average <= nintey_day_average{
        return true;
    } else {
        return false;
    }
}
/*
fn update_price_history(prices: &mut Vec<f32>, recent_price: f32){
    prices.remove(0);
    prices.push(recent_price);
}
*/
// use std::collections::HashMap
fn fetch(equity: &str) -> std::result::Result<Vec<f32>, reqwest::Error> {
    let client = Client::new();
    let mut own_string: String = "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol=".to_owned();
    let bor_string: &str = equity;
    own_string.push_str(bor_string);
    let bor_string2: &str = "&interval=1min&apikey=IVO96IWUXGF22KP9";
    own_string.push_str(bor_string2); 
    //println!("{}", own_string);
    let json = |mut res : Response | {
        res.json::<Value>()
    };
    let request1 =
        client
            .get(&own_string)
            .send()
            .and_then(json);
         request1.map(|res1|{
            let obj = res1.as_object().unwrap();
            let meta = obj["Time Series (1min)"].as_object().unwrap();
            let mut stocks: Vec<f32> = Vec::new();
            let start = meta.len() -90;
            let mut index = 0;
            for x in meta {
                if index >= start {
                let cur_stock = x.1.as_object().unwrap();
                let open = cur_stock["1. open"].as_str().unwrap();
                let x = f32::from_str(open).unwrap();
              // println!("{:?}", x);
                stocks.push(x);
                index = index + 1;
                }
                index = index + 1;
            }
           // println!("{:?}", stocks.len());
            return stocks;
        })
        .map_err(|err| {
            println!("stdout error: {}", err);
            return err;
        })
}
fn main() {    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide the equity of your choosing. Need 1 arg");
        return;
    }
    let query = &args[1];
    println!("Retrieving stock prices from {:?}", query);
 let x = fetch(query);
match x {
    Ok(res) => {
    let mut tmp_prices = res; 
    loop{
let duration = time::Duration::from_secs(60);
thread::sleep(duration);

let recent_price = get_most_recent_price(query);
tmp_prices.remove(0);
tmp_prices.push(recent_price);
if k_means(&mut tmp_prices) {
    println!("Buy!");
 } else {
    print!("Don’t buy!");
 }
}
    },
    Err(e) => println!("error parsing header: {:?}", e),
}
}