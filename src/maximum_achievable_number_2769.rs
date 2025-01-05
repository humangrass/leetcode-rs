use crate::solution::Solution;

impl Solution {
    /// 2769. Find the Maximum Achievable Number - https://leetcode.com/problems/find-the-maximum-achievable-number/description/
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + t * 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::the_maximum_achievable_x(4, 1), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::the_maximum_achievable_x(3, 2), 7);
    }
}
