use crate::solution::Solution;

#[allow(dead_code)]
impl Solution {
    // 3110. Score of a String - https://leetcode.com/problems/score-of-a-string/description/
    pub fn score_of_string(s: String) -> i32 {
        if s.len() == 1 {
            // unwrap: it's OK
            return s.chars().next().unwrap() as i32;
        }

        s.chars()
            .map(|c| c as i32)
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|pair| (pair[0] - pair[1]).abs())
            .sum()
    }

    pub fn score_of_string_2(s: String) -> i32 {
        let mut r = 1;
        let mut result = 0;
        let chars: Vec<i32> = s.chars().map(|c| c as i32).collect();

        if s.len() == 1 {
            return chars[0];
        }

        while r < s.len() {
            let mut score = chars[r - 1] - chars[r];
            score = score.abs();

            result += score;
            r += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "hello".to_string();
        assert_eq!(Solution::score_of_string(s), 13);
    }

    #[test]
    fn test_case_2() {
        let s = "zaz".to_string();
        assert_eq!(Solution::score_of_string(s), 50)
    }

    #[test]
    fn test_case_3() {
        let s = "hello".to_string();
        assert_eq!(Solution::score_of_string_2(s), 13);
    }

    #[test]
    fn test_case_4() {
        let s = "zaz".to_string();
        assert_eq!(Solution::score_of_string_2(s), 50)
    }
}
