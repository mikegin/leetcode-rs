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


      //   fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
      //     match (left, right) {
      //         (Some(l), Some(r)) =>
      //             l.borrow().val == r.borrow().val &&
      //             Solution::is_mirror(l.borrow().left.clone(), r.borrow().right.clone()) &&
      //             Solution::is_mirror(l.borrow().right.clone(), r.borrow().left.clone()),
      //         (None, None) => true,
      //         _ => false,
      //     }
      // }
  
      // pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
      //     if let Some(root_node) = root {
      //         Solution::is_mirror(root_node.borrow().left.clone(), root_node.borrow().right.clone())
      //     } else {
      //         false
      //     }
      // }
      
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
          return true;
        }

        let r = root.as_ref().unwrap().borrow();

        let mut left_q = vec![r.left.clone()];
        let mut right_q = vec![r.right.clone()];

        while left_q.len() > 0 || right_q.len() > 0 {
          if left_q.len() != right_q.len() {
            return false;
          }

          let mut new_left_q = vec![];
          let mut new_right_q = vec![];

          //is reverse/mirror symmetric
          for i in 0..left_q.len() {
            let l = left_q[i].clone();
            let r = right_q[i].clone();
            let r_rev = right_q[right_q.len() - 1 - i].clone();

            if l.is_some() && r_rev.is_none() || l.is_none() && r_rev.is_some() { // don't match symmetrically
              return false;
            }
            
            if l.is_some() && r_rev.is_some() && l.as_ref().unwrap().borrow().val != r_rev.as_ref().unwrap().borrow().val {
              return false;
            }

            // search children breadthfirst
            if l.is_some() {
              let _l = l.as_ref().unwrap();
              new_left_q.push(_l.borrow().left.clone());
              new_left_q.push(_l.borrow().right.clone());
            }

            // search children breadthfirst
            if r.is_some() {
              let _r = r.as_ref().unwrap();
              new_right_q.push(_r.borrow().left.clone());
              new_right_q.push(_r.borrow().right.clone());
            }

          }

          left_q = new_left_q;
          right_q = new_right_q;
        }


        true
    }
}

struct Solution {}

#[test]
fn test1() {
  //left side
  let left_left = Rc::new(RefCell::new(TreeNode::new(3)));

  let left_right = Rc::new(RefCell::new(TreeNode::new(4)));

  let left = Rc::new(RefCell::new(TreeNode::new(2)));

  left.borrow_mut().left = Some(left_left);
  left.borrow_mut().right = Some(left_right);



  //right side
  let right_left = Rc::new(RefCell::new(TreeNode::new(4)));

  let right_right = Rc::new(RefCell::new(TreeNode::new(3)));

  let right = Rc::new(RefCell::new(TreeNode::new(2)));

  right.borrow_mut().left = Some(right_left);
  right.borrow_mut().right = Some(right_right);

  let root = Rc::new(RefCell::new(TreeNode::new(1)));

  root.borrow_mut().left = Some(Rc::clone(&left));
  root.borrow_mut().right = Some(Rc::clone(&right));
  

  assert_eq!(Solution::is_symmetric(Some(root)), true);
}

#[test]
fn test2() {
  //left side
  let left_right = Rc::new(RefCell::new(TreeNode::new(3)));

  let left = Rc::new(RefCell::new(TreeNode::new(2)));

  left.borrow_mut().right = Some(left_right);



  //right side
  let right_right = Rc::new(RefCell::new(TreeNode::new(3)));

  let right = Rc::new(RefCell::new(TreeNode::new(2)));

  right.borrow_mut().right = Some(right_right);

  let root = Rc::new(RefCell::new(TreeNode::new(1)));

  root.borrow_mut().left = Some(Rc::clone(&left));
  root.borrow_mut().right = Some(Rc::clone(&right));
  

  assert_eq!(Solution::is_symmetric(Some(root)), false);
}