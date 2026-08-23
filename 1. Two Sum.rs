impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash = HashMap::new();
        for i in 0..nums.len() {
            let diff = target - nums[i];
            hash.insert(diff, i);
        }

        for i in 0..nums.len() {
            if let Some(&j) = hash.get(&nums[i]) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![0, 0]
    }
}
