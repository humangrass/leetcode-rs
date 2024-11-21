// 169. Majority Element - https://leetcode.com/problems/majority-element/description/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut result = nums[0];
    let mut counter = 1;

    for &i in &nums[1..] {
        if counter == 0 {
            result = i;
            counter = 1
        } else if i == result {
            counter += 1
        } else {
            counter -= 1
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![3, 2, 3];
        assert_eq!(majority_element(nums), 3)
    }

    #[test]
    fn test_case_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(nums), 2)
    }

    #[test]
    fn test_case_3() {
        let nums = vec![
            1, 1, 1, 1, 1,
            2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3,
        ];
        assert_eq!(majority_element(nums), 3)
    }
}