use crate::solution::Solution;

#[allow(dead_code)]
impl Solution {
    // 55. Jump Game - https://leetcode.com/problems/jump-game/
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut jump_size = nums[0];

        for i in 1..nums.len() {
            if i <= (jump_size as usize) {
                jump_size = std::cmp::max(jump_size, (i as i32) + nums[i])
            }
        }

        jump_size >= ((nums.len() as i32) - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(Solution::can_jump(nums), true)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 1, 0, 4];
        assert_eq!(Solution::can_jump(nums), false)
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::can_jump(nums), true)
    }
}
