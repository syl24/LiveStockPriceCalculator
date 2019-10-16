
fn main(){

    
    //init mock array for 90% goal
    let mut prices: Vec<f32> = Vec::new();
    for i in 0..90{
        prices.push(5.5 + i as f32);
    } 
    
    let _ret: bool = k_means(&mut prices);
    println!("{}", _ret);
    
    //in integration this val below will be replaced by fn call to getMostRecentPrice();
    let recent: f32 = 1000.1;
    update_price_history(&mut prices, recent);
    k_means(&mut prices);
  
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

fn update_price_history(prices: &mut Vec<f32>, recent_price: f32){
    prices.remove(0);
    prices.push(recent_price);
}
