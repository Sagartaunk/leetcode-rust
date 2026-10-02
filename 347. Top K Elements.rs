impl Solution {
    pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let _ = nums.sort_unstable();
        use std::collections::HashMap;
        let mut hash: HashMap<i32, i32> = HashMap::new();
        for i in nums.iter() {
            *hash.entry(*i).or_insert(0) += 1;
        }
        use std::cmp::Reverse;

        let mut top: Vec<_> = hash.iter().collect();
        top.sort_unstable_by_key(|&(_, v)| Reverse(*v));

        let top_keys: Vec<_> = top
            .into_iter()
            .take(k as usize)
            .map(|(key, _)| key.clone())
            .collect();
        top_keys
    }
}
