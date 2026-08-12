impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s = s.into_bytes();
        let mut t = t.into_bytes();
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}
