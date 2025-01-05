use std::cmp::max;
use crate::solution::Solution;

#[allow(dead_code)]
impl Solution {
    // 485. Max Consecutive Ones - https://leetcode.com/problems/max-consecutive-ones/
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut s = 0;
        for i in nums {
            s = if i == 1 {
                s + 1
            } else {
                0
            };
            result = max(result, s)
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(Solution::find_max_consecutive_ones(nums), 3)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(Solution::find_max_consecutive_ones(nums), 2)
    }
}
