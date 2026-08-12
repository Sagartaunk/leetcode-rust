impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        nums.windows(2).any(|i| i[0] == i[1])
    }
}
