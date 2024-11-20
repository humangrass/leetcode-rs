use std::cmp::{max, min};

// 121. Best Time to Buy and Sell Stock - https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut max_profit = 0;
    let mut min_price = prices[0];

    for &price in &prices[1..] {
        min_price = min(min_price, price);
        max_profit = max(max_profit, price - min_price);
    }

    max_profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(nums), 0)
    }
}