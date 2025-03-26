impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;

        let min_price = i32::MAX;

        for price in prices {
            min_price = min_price.min(price);

            current_profit = price - min_price;

            max_profit = max_profit.max(current_profit);
        }

        max_profit as i32
    }
}
