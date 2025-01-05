use std::cmp::max;
use std::collections::HashMap;
use crate::solution::Solution;

#[allow(dead_code)]
impl Solution {
    // 3090. Maximum Length Substring With Two Occurrences - https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/description/
    pub fn maximum_length_substring_v1(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let max_repetitions = 2;

        let mut l = 0;
        let mut r = 0;
        let mut result = 0;
        let mut mapa = HashMap::new();

        while r < chars.len() {
            if mapa.is_empty() {
                *mapa.entry(chars[r]).or_insert(0) += 1;
                r += 1;
                result = max(result, r - l);
                continue;
            }

            if mapa.contains_key(&chars[r]) {
                if mapa[&chars[r]] < max_repetitions {
                    *mapa.entry(chars[r]).or_insert(0) += 1;
                    r += 1;
                    result = max(result, r - l);
                } else {
                    result = max(result, r - l);
                    mapa = HashMap::new();
                    l += 1;
                    r = l;
                }
            } else {
                *mapa.entry(chars[r]).or_insert(0) += 1;
                r += 1;
                result = max(result, r - l)
            }
        }

        result as i32
    }

    pub fn maximum_length_substring_v2(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let max_repetitions = 2;
        let mut l = 0;
        let mut result = 0;
        let mut mapa = HashMap::new();

        for r in 0..chars.len() {
            *mapa.entry(chars[r]).or_insert(0) += 1;

            while mapa[&chars[r]] > max_repetitions {
                if let Some(count) = mapa.get_mut(&chars[l]) {
                    *count -= 1;
                    if *count == 0 {
                        mapa.remove(&chars[l]);
                    }
                }
                l += 1;
            }

            // Update the result if the current subsegment satisfies max_repetitions the conditions
            result = max(result, r - l + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "bcbbbcba".to_string();
        assert_eq!(Solution::maximum_length_substring_v1(s), 4)
    }

    #[test]
    fn test_case_2() {
        let s = "aaaa".to_string();
        assert_eq!(Solution::maximum_length_substring_v1(s), 2)
    }

    #[test]
    fn test_case_3() {
        let s = "bcbbbcba".to_string();
        assert_eq!(Solution::maximum_length_substring_v2(s), 4)
    }

    #[test]
    fn test_case_4() {
        let s = "aaaa".to_string();
        assert_eq!(Solution::maximum_length_substring_v2(s), 2)
    }
}
