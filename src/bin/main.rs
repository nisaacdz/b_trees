use b_trees::AVL;
fn main() {
    let nums = vec![1,5,9,1,5,9];
    let index_diff = 2;
    let value_diff = 3;
    let res = Solution::contains_nearby_almost_duplicate(nums, index_diff, value_diff);
    println!("result = {res}")
}

struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let index_diff = index_diff as usize;
        let (mut left, mut right) = (0, 1);
        let mut avl = AVL::new();
        avl.insert(nums[0]);
        while right < nums.len() {
            while right < nums.len() && right - left <= index_diff {
                if avl.min_abs_diff(nums[right]).unwrap() <= value_diff { return true }
                avl.insert(nums[right]);
                right += 1;
            }
            while right - left > index_diff {
                avl.delete(&nums[left]);
                left += 1;
            }
        }
        false
    }
}