loop{

use std::{thread, time};
let duration = time::Duration::from_secs(60);
thread::sleep(duration);

let recent_price = get_most_recent_price();

prices.remove(0);
prices.push(recent_price);

if kMeansAlg(prices) {
    println!("Buy!");
 } else {
    print!("Don’t buy!");
 }
}