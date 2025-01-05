use crate::solution::Solution;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    // 3. Longest Substring Without Repeating Characters - https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut result = 0;
        let mut mapa = HashMap::new();

        for (right, c) in s.chars().enumerate() {
            if let Some(&prev_index) = mapa.get(&c) {
                left = left.max(prev_index + 1);
            }
            mapa.insert(c, right);
            result = result.max(right - left + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3)
    }

    #[test]
    fn test_case_2() {
        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1)
    }

    #[test]
    fn test_case_3() {
        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3)
    }
}
