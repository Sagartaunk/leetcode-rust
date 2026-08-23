impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let str_bytes: Vec<Vec<u8>> = strs
            .iter()
            .map(|s| {
                let mut bytes = s.as_bytes().to_vec();
                bytes.sort();
                bytes
            })
            .collect();
        let mut pos: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();
        for (i, bytes) in str_bytes.into_iter().enumerate() {
            pos.entry(bytes).or_default().push(i);
        }
        let res: Vec<Vec<String>> = pos
            .values()
            .map(|indices| indices.iter().map(|&i| strs[i].clone()).collect())
            .collect();
        res
    }
}
