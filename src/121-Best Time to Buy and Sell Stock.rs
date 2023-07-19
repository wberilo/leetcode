pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = 0;
    let mut max = 0;
    let mut profit ;

    for i in 1..prices.len() {
        profit = prices[i] -  prices[buy];
        if profit > max {
            max = profit;
        }
        if prices[i] <  prices[buy] {
            buy = i;
        }
    }
    max
}
