struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut dummy_head = Box::new(ListNode::new(0));
    let mut p = l1;
    let mut q = l2;
    let mut carry = 0;
    let mut curr = &mut dummy_head;

    while p != None || q != None {
      let x: i32;
      let y: i32;
      if let Some(box_v) = p {
        let list = *box_v;
        x = list.val;
        p = list.next;
      } else {
        x = 0;
      }
      if let Some(box_v) = q {
        let list = *box_v;
        y = list.val;
        q = list.next;
      } else {
        y = 0;
      }

      let sum = carry + x + y;
      carry = (sum as f64 / 10_f64).floor() as i32;

      curr.next = Some(Box::new(ListNode::new(sum % 10)));
      // Option.as_mut Converts from &mut Option<T> to **Option<&mut T>**.
      // Option.unwrap 取得 Some 内容
      curr = curr.next.as_mut().unwrap();
    }

    if carry >= 1 {
      curr.next = Some(Box::new(ListNode::new(carry)));
      curr = curr.next.as_mut().unwrap();
    }

    dummy_head.next
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn add_two_numbers() {
    use super::ListNode;
    use super::Solution;
    let l1 = Some(Box::new(ListNode {
      val: 2,
      next: Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode::new(3))),
      })),
    }));
    let l2 = Some(Box::new(ListNode {
      val: 5,
      next: Some(Box::new(ListNode {
        val: 6,
        next: Some(Box::new(ListNode::new(4))),
      })),
    }));
    let l3 = Some(Box::new(ListNode {
      val: 7,
      next: Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode::new(8))),
      })),
    }));
    assert_eq!(Solution::add_two_numbers(l1, l2), l3);
  }
}
