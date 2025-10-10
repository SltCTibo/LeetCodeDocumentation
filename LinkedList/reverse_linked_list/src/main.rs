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

struct Solution {

}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        match head {
            None => None,
            Some(n) => {
                let mut next = Some(n.clone());
                let mut ret: Option<Box<ListNode>> = None;
                while let Some(nxt) = next.take() {
                    let mut new_node = Box::new(ListNode::new(nxt.val));
                    new_node.next = ret;
                    ret = Some(new_node);
                    
                    if n.next == None {
                        return ret;
                    } else {
                        next = nxt.next;
                    }
                }

                ret
            }
        }
    }
}