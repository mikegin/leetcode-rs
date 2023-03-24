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
use std::borrow::Borrow;
use std::rc::Rc;
use std::cell::RefCell;
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    
    let mut v = vec![];

    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, _v: &mut Vec<i32>) {
      if node.is_none() {
        return; // exit
      }

      let n = node.as_ref().unwrap();

      traverse(&n.borrow_mut().left, _v);

      _v.push(n.borrow_mut().val);

      traverse(&n.borrow_mut().right, _v);
    };

    traverse(&root, &mut v);

    v
}

#[test]
fn test1() {
  let root_right_left = Rc::new(RefCell::new(TreeNode::new(3)));

  let root_right = Rc::new(RefCell::new(TreeNode::new(2)));
  root_right.borrow_mut().left = Some(Rc::clone(&root_right_left));

  let root = Rc::new(RefCell::new(TreeNode::new(1)));
  root.borrow_mut().right = Some(Rc::clone(&root_right));
  

  assert_eq!(inorder_traversal(Some(root)), vec![1, 3, 2]);
}
// Input: root = [1,null,2,3]
// Output: [1,3,2]

// Example 2:

// Input: root = []
// Output: []

// Example 3:

// Input: root = [1]
// Output: [1]
