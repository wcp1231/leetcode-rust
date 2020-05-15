/**
 * [128] Longest Consecutive Sequence
 *
 * Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
 * 
 * Your algorithm should run in O(n) complexity.
 * 
 * Example:
 * 
 * 
 * Input: [100, 4, 200, 1, 3, 2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
 * 
 * 
 */

pub struct Solution {}

// problem: https://leetcode.com/problems/longest-consecutive-sequence/
// discuss: https://leetcode.com/problems/longest-consecutive-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::HashMap;
use std::rc::Rc;
use std::cmp;

const E: i32 = i32::min_value();

struct Segment {
    pub left: i32,
    pub right: i32,
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // TODO 每个元素是一个线段，每次处理时和左右两边的线段合并就行
        let empty = Rc::new(Segment { left: E, right: E });
        let mut segments: HashMap<i32, Rc<Segment>> = HashMap::new();
        let mut max = 0;

        for n in nums.into_iter() {
            let (ll, lr, ml, mr, rl, rr) = {
                let segment_left = segments.get(&(n - 1));
                let segment_middle = segments.get(&n);
                let segment_right = segments.get(&(n + 1));
                let sl = segment_left.unwrap_or(&empty);
                let sm = segment_middle.unwrap_or(&empty);
                let sr = segment_right.unwrap_or(&empty);
                (sl.left, sl.right, sm.left, sm.right, sr.left, sr.right)
            };

            let (l, r) = if lr != E && rl != E && lr == n - 1 && n + 1 == rl {
                segments.remove(&lr);
                segments.remove(&rl);
                (ll, rr)
            } else if lr != E && lr == n - 1 {
                segments.remove(&lr);
                (ll, n)
            } else if rl != E && n + 1 == rl {
                segments.remove(&rl);
                (n, rr)
            } else {
                (n, n)
            };

            if ml != E && (mr - ml + 1) > (r - l + 1) {
                continue;
            }

            let segment = Rc::new(Segment { left: l, right: r });
            segments.insert(l, segment.clone());
            segments.insert(r, segment.clone());
            max = cmp::max(r - l + 1, max);
        }

        max
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_128() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let ans = Solution::longest_consecutive(nums);
        assert_eq!(ans, 4);

        let nums = vec![0, -2, 100, 4, -1, 200, 1, 3, 2];
        let ans = Solution::longest_consecutive(nums);
        assert_eq!(ans, 7);

        let nums = vec![-7,-1,3,-9,-4,7,-3,2,4,9,4,-9,8,-7,5,-1,-7];
        let ans = Solution::longest_consecutive(nums);
        assert_eq!(ans, 4);

        let nums = vec![1,2,3,4,5,3,4,5,6,7,8];
        let ans = Solution::longest_consecutive(nums);
        assert_eq!(ans, 8);
    }
}
