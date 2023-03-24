// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;


fn create_rc_refcell(x: i32) -> Rc<RefCell<TreeNode>> {
  Rc::new(RefCell::new(TreeNode::new(x)))
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
          return 0;
        }

        let mut q = vec![];
        q.push(root);

        let mut result = 0;

        while q.len() > 0 {
          result += 1;
          let mut new_q = vec![];
          for i in q {
            if i.is_some() {
              let _i = i.as_ref().unwrap().borrow();
              if _i.left.is_some() {
                new_q.push(_i.left.clone());
              }
              if _i.right.is_some() {
                new_q.push(_i.right.clone());
              }
            }
          }
          q = new_q;
        }

        result
    }
}

struct Solution {}


#[test]
fn test1() {
  let seven = create_rc_refcell(7);
  let fifteen = create_rc_refcell(15);
  let twenty = create_rc_refcell(20);
  let nine = create_rc_refcell(9);
  let three = create_rc_refcell(3);

  three.borrow_mut().left = Some(nine);
  three.borrow_mut().right = Some(twenty.clone());
  twenty.borrow_mut().left = Some(fifteen);
  twenty.borrow_mut().right = Some(seven);


  assert_eq!(Solution::max_depth(Some(three)), 3);
}

#[test]
fn test2() {
  let one = create_rc_refcell(1);
  let two = create_rc_refcell(2);

  one.borrow_mut().right = Some(two);


  assert_eq!(Solution::max_depth(Some(one)), 2);
}