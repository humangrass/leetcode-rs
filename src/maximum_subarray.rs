use std::cmp::max;

// 53. Maximum Subarray - https://leetcode.com/problems/maximum-subarray/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    let mut result = nums[0];
    let mut s = nums[0];

    for i in &nums[1..] {
        s = max(s, 0);
        s += i;
        result = max(result, s)
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums), 6)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1];
        assert_eq!(max_sub_array(nums), 1)
    }

    #[test]
    fn test_case_3() {
        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(max_sub_array(nums), 23)
    }
}