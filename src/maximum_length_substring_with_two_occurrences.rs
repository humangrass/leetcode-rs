use std::cmp::max;
use std::collections::HashMap;

// 3090. Maximum Length Substring With Two Occurrences - https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/description/
pub fn maximum_length_substring(s: String) -> i32 {
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "bcbbbcba".to_string();
        assert_eq!(maximum_length_substring(s), 4)
    }

    #[test]
    fn test_case_2() {
        let s = "aaaa".to_string();
        assert_eq!(maximum_length_substring(s), 2)
    }
}
