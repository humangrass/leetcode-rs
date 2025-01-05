/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
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

    pub fn as_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        vec.push(self.val);
        let mut current = self.next.as_ref();
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next.as_ref();
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let vec = vec![1, 2, 3, 4, 5];
        let input_list_node = ListNode::from_vec(vec.clone());
        let output_list_node = input_list_node.unwrap().as_vec();
        assert_eq!(output_list_node, vec);
    }

    #[test]
    fn test_case_2() {
        let vec = vec![];
        let input_list_node = ListNode::from_vec(vec.clone());
        let output_list_node = match input_list_node {
            Some(input_list_node) => input_list_node.as_vec(),
            None => vec![],
        };
        assert_eq!(output_list_node, vec);
    }
}
