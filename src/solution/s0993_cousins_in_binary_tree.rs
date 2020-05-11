/**
 * [993] Cousins in Binary Tree
 *
 * In a binary tree, the root node is at depth 0, and children of each depth k node are at depth k+1.
 * 
 * Two nodes of a binary tree are cousins if they have the same depth, but have different parents.
 * 
 * We are given the root of a binary tree with unique values, and the values x and y of two different nodes in the tree.
 * 
 * Return true if and only if the nodes corresponding to the values x and y are cousins.
 * 
 *  
 * 
 * Example 1:<br />
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/12/q1248-01.png" style="width: 180px; height: 160px;" />
 * 
 * 
 * Input: root = <span id="example-input-1-1">[1,2,3,4]</span>, x = <span id="example-input-1-2">4</span>, y = <span id="example-input-1-3">3</span>
 * Output: <span id="example-output-1">false</span>
 * 
 * 
 * <div>
 * Example 2:<br />
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/12/q1248-02.png" style="width: 201px; height: 160px;" />
 * 
 * 
 * Input: root = <span id="example-input-2-1">[1,2,3,null,4,null,5]</span>, x = <span id="example-input-2-2">5</span>, y = <span id="example-input-2-3">4</span>
 * Output: <span id="example-output-2">true</span>
 * 
 * 
 * <div>
 * Example 3:
 * 
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/13/q1248-03.png" style="width: 156px; height: 160px;" />
 * 
 * 
 * Input: root = <span id="example-input-3-1">[1,2,3,null,4]</span>, x = 2, y = 3
 * Output: <span id="example-output-3">false</span>
 * 
 *  
 * </div>
 * </div>
 * 
 * Note:
 * 
 * <ol>
 * 	The number of nodes in the tree will be between 2 and 100.
 * 	Each node has a unique integer value from 1 to 100.
 * </ol>
 * 
 * <div>
 * <div>
 * <div> </div>
 * </div>
 * </div>
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/cousins-in-binary-tree/
// discuss: https://leetcode.com/problems/cousins-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::{RefCell, Ref};

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        if x == y || root.is_none() {
            return false;
        }

        let x_result: Option<(i32, i32)> = Solution::find_depth_and_parent(&root, x, 0);
        let y_result: Option<(i32, i32)> = Solution::find_depth_and_parent(&root, y, 0);

        if x_result.is_none() || y_result.is_none() {
            return false;
        }

        let x_result = x_result.unwrap();
        let y_result = y_result.unwrap();

        if x_result.0 == y_result.0 && x_result.1 != y_result.1 {
            return true;
        }

        false
    }

    fn find_depth_and_parent(root: &Option<Rc<RefCell<TreeNode>>>, x: i32, depth: i32) -> Option<(i32, i32)> {
        let mut result = None;
        // TODO 太丑了
        if let Some(inner) = root {
            let val = inner.borrow().val;
            if let Some(l) = &inner.borrow().left {
                if *(&l.borrow().val) == x {
                    result = Some((depth, val));
                }
            }
            if result.is_none() {
                if let Some(r) = &inner.borrow().right {
                    if *(&r.borrow().val) == x {
                        result = Some((depth, val));
                    }
                }
            }
            if result.is_none() {
                result = Solution::find_depth_and_parent(&inner.borrow().left, x, depth + 1);
            }
            if result.is_none() {
                result = Solution::find_depth_and_parent(&inner.borrow().right, x, depth + 1);
            }
        }
        return result;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_993() {
        let root = to_tree(vec![Some(1),Some(2),Some(3), Some(4)]);
        let answer = Solution::is_cousins(root, 4, 3);
        assert!(!answer);

        let root = to_tree(vec![Some(1),Some(2),Some(3), None, Some(4), None, Some(5)]);
        let answer = Solution::is_cousins(root, 4, 5);
        assert!(answer);

        let root = to_tree(vec![Some(1),Some(2),Some(3), None, Some(4), None, Some(5)]);
        let answer = Solution::is_cousins(root, 2, 3);
        assert!(!answer);
    }
}
