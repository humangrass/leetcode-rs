use crate::list_node::ListNode;
use crate::solution::Solution;

#[allow(dead_code)]
impl Solution {
    /// 206. Reverse Linked List - Best solution - https://leetcode.com/problems/reverse-linked-list/description/
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn reverse_list_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(node) => {
                let mut input_vec = node.as_vec();
                input_vec.reverse();
                ListNode::from_vec(input_vec)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::list_node::ListNode;

    #[test]
    fn test_case_1() {
        let input_list_node = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let output_list_node = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(input_list_node), output_list_node);
    }

    #[test]
    fn test_case_2() {
        let input_list_node = ListNode::from_vec(vec![1, 2]);
        let output_list_node = ListNode::from_vec(vec![2, 1]);
        assert_eq!(Solution::reverse_list(input_list_node), output_list_node);
    }

    #[test]
    fn test_case_3() {
        let input_list_node = ListNode::from_vec(vec![]);
        let output_list_node = ListNode::from_vec(vec![]);
        assert_eq!(Solution::reverse_list(input_list_node), output_list_node);
    }

    #[test]
    fn test_case_4() {
        let input_list_node = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let output_list_node = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list_2(input_list_node), output_list_node);
    }

    #[test]
    fn test_case_5() {
        let input_list_node = ListNode::from_vec(vec![1, 2]);
        let output_list_node = ListNode::from_vec(vec![2, 1]);
        assert_eq!(Solution::reverse_list_2(input_list_node), output_list_node);
    }

    #[test]
    fn test_case_6() {
        let input_list_node = ListNode::from_vec(vec![]);
        let output_list_node = ListNode::from_vec(vec![]);
        assert_eq!(Solution::reverse_list_2(input_list_node), output_list_node);
    }
}
