use crate::solution::Solution;

#[allow(dead_code)]
impl Solution {
    // 283. Move Zeroes - https://leetcode.com/problems/move-zeroes/description/
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[index] = nums[i];
                index += 1;
            }
        }
        for i in index..nums.len() {
            nums[i] = 0;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = &mut vec![0,1,0,3,12];
        Solution::move_zeroes(nums);
        assert_eq!(nums, &mut vec![1,3,12,0,0])
    }

    #[test]
    fn test_case_2() {
        let nums = &mut vec![0];
        Solution::move_zeroes(nums);
        assert_eq!(nums, &mut vec![0])
    }
}