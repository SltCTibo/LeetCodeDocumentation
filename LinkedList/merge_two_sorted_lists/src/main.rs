// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match list1 {
            None => list2,
            Some(n) => {
                let mut first = Some(n.clone());
                let mut second = list2.clone();
                let mut ret: Option<Box<ListNode>> = None;
                let mut prev_node = &mut ret;
                while let Some(nxt) = first.take() {
                    let new_node: Box<ListNode>;
                    if let Some(nxt_sec) = second.take() {
                        if nxt.val > nxt_sec.val {
                            new_node = Box::new(ListNode::new(nxt_sec.val));
                            second = nxt_sec.next;
                            first = Some(nxt);
                        } else {
                            new_node = Box::new(ListNode::new(nxt.val));
                            first = nxt.next;
                            second = Some(nxt_sec);
                        }
                    } else {
                        new_node = Box::new(ListNode::new(nxt.val));
                        first = nxt.next;
                    }
                    prev_node = &mut prev_node.insert(new_node).next;
                }

                if let Some(nxt_sec) = second.take() {
                    prev_node = &mut prev_node.insert(nxt_sec).next;
                }

                ret
            }
        }

    }
}
