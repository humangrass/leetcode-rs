use crate::solution::Solution;
use itertools::Itertools;

#[allow(dead_code)]
impl Solution {
    /// 46. Permutations -  https://leetcode.com/problems/permutations/description/
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::backtrack(&mut nums, 0, &mut result);

        result
    }

    fn backtrack(nums: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i);
            Self::backtrack(nums, start + 1, result);
            nums.swap(start, i);
        }
    }

    /// 46. Permutations - using itertools -  https://leetcode.com/problems/permutations/description/
    pub fn permute_using_itertools(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length = nums.len();
        nums.into_iter().permutations(length).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    fn to_set(vec: Vec<Vec<i32>>) -> HashSet<Vec<i32>> {
        vec.into_iter().collect()
    }

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3];
        let result = Solution::permute(nums);
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        assert_eq!(to_set(result), to_set(expected));
    }

    #[test]
    fn test_case_2() {
        let nums = vec![0, 1];
        let result = Solution::permute(nums);
        let expected = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(to_set(result), to_set(expected));
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1];
        let result = Solution::permute(nums);
        let expected = vec![vec![1]];
        assert_eq!(to_set(result), to_set(expected));
    }
}
