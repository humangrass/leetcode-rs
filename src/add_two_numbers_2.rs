// 2. Add Two Numbers - https://leetcode.com/problems/add-two-numbers/description/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;

        for &value in values.iter().rev() {
            let mut node = Box::new(ListNode::new(value));
            node.next = current;
            current = Some(node);
        }

        current
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        from_prev: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && from_prev == 0 {
            return None;
        }

        let val1 = l1.as_ref().map_or(0, |node| node.val);
        let val2 = l2.as_ref().map_or(0, |node| node.val);

        let sum = val1 + val2 + from_prev;

        let mut result = ListNode::new(sum % 10);
        result.next = helper(
            l1.and_then(|node| node.next),
            l2.and_then(|node| node.next),
            // integer division: 18 / 10 = 1
            sum / 10,
        );

        Some(Box::new(result))
    }

    helper(l1, l2, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let res = ListNode::from_vec(vec![7, 0, 8]);
        // 342 + 465 = 807
        assert_eq!(add_two_numbers(l1, l2), res)
    }

    #[test]
    fn test_case_2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let res = ListNode::from_vec(vec![0]);
        // 342 + 465 = 807
        assert_eq!(add_two_numbers(l1, l2), res)
    }

    #[test]
    fn test_case_3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let res = ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        // 9999999 + 9999 = 10009998
        assert_eq!(add_two_numbers(l1, l2), res)
    }
}