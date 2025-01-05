use crate::solution::Solution;

#[allow(dead_code)]
impl Solution {
    // 5. Longest Palindromic Substring - https://leetcode.com/problems/longest-palindromic-substring/description/
    // O(n²)
    pub fn longest_palindrome(s: String) -> String {
        fn palindrome_length(chars: &[char], mut left: isize, mut right: isize) -> isize {
            while left >= 0 && right < chars.len() as isize && chars[left as usize] == chars[right as usize] {
                left -= 1;
                right += 1;
            }
            right - left - 1
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_length = 0;

        for i in 0..chars.len() {
            let len1 = palindrome_length(&chars, i as isize, i as isize);
            let len2 = palindrome_length(&chars, i as isize, i as isize + 1);
            let len = len1.max(len2);

            if len > max_length {
                max_length = len;
                start = i - ((len as usize - 1) / 2);
            }
        }

        let end = start + max_length as usize;
        s[start..end].to_string()
    }

    // O(n³)
    pub fn _longest_palindrome(s: String) -> String {
        fn is_palindrome(s: &str) -> bool {
            s.chars().eq(s.chars().rev())
        }

        let mut result = String::new();

        for left in 0..s.len() {
            for right in left + 1..=s.len() {
                let slice = &s[left..right];
                if is_palindrome(slice) && slice.len() > result.len() {
                    result = slice.to_string();
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        assert!(
            result == "bab" || result == "aba",
            "Expected `bab` or `aba`, but got `{}`",
            result,
        )
    }

    #[test]
    fn test_case_2() {
        let s = "cbbd".to_string();
        assert_eq!(Solution::longest_palindrome(s), "bb")
    }

    #[test]
    fn test_case_3() {
        let s = "a".to_string();
        assert_eq!(Solution::longest_palindrome(s), "a")
    }

    #[test]
    fn test_case_4() {
        let s = "aa".to_string();
        assert_eq!(Solution::longest_palindrome(s), "aa")
    }
}
