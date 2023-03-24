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

struct Solution {}
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn traverse(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
          if p.is_none() && q.is_none() {
            return true;
          };
  
          if p.is_none() || q.is_none() {
            return false;
          };
  
          // let _p = p.unwrap().borrow();
          // let _q = q.unwrap().borrow();

          let _p = p.as_ref().unwrap().borrow();
          let _q = q.as_ref().unwrap().borrow();
  
          let left = traverse(&_p.left, &_q.left);
          let right = traverse(&_p.right, &_q.right);
          
          &_p.val == &_q.val && left && right
        }

        traverse(&p, &q)
    }
}

#[test]
fn test1() {
  let left = Rc::new(RefCell::new(TreeNode::new(2)));
  let right = Rc::new(RefCell::new(TreeNode::new(3)));
  let root = Rc::new(RefCell::new(TreeNode::new(1)));
  root.borrow_mut().left = Some(left);
  root.borrow_mut().right = Some(right);

  let left2 = Rc::new(RefCell::new(TreeNode::new(2)));
  let right2 = Rc::new(RefCell::new(TreeNode::new(3)));
  let root2 = Rc::new(RefCell::new(TreeNode::new(1)));
  root2.borrow_mut().left = Some(left2);
  root2.borrow_mut().right = Some(right2);


  assert_eq!(Solution::is_same_tree(Some(root), Some(root2)), true);
}