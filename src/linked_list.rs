// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_slice(v: &[i32]) -> Option<Box<ListNode>> {
        v.iter()
            .rev()
            .fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
    }

    pub fn to_vec(mut list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        while let Some(inner) = list {
            res.push(inner.val);
            list = &inner.next;
        }
        res
    }
}
