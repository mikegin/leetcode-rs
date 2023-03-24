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
  pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
      let mut stack = vec![];
      let mut curr = &head;
      while curr.is_some() {
        stack.push(curr.as_ref().unwrap().val);
        curr = &curr.as_ref().unwrap().next
      }

      for i in 0..stack.len()/2 {
        if stack[i] != stack[stack.len() - 1 - i] {
          return false
        }
      }

      true
  }
}

struct Solution {}

#[test]
fn test1() {
  let two = ListNode::new(2);
  let mut one = ListNode::new(1);
  one.next = Some(Box::new(two));

  assert_eq!(Solution::is_palindrome(Some(Box::new(one))), false);
}

#[test]
fn test2() {
  let mut two = ListNode::new(2);
  let mut two2 = two.clone();
  let mut one = ListNode::new(1);
  let mut one2 = one.clone();
  two2.next = Some(Box::new(one2));
  two.next = Some(Box::new(two2));
  one.next = Some(Box::new(two));
  

  assert_eq!(Solution::is_palindrome(Some(Box::new(one))), true);
}