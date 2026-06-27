use std::collections::HashMap;


impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut count_map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in &strs {
            let mut count = [0;26];
            for c in s.bytes() {
                count[(c-'a' as u8) as usize] +=1;
            }
            count_map.entry(count).or_default().push(s.clone());
        }
        count_map.into_values().collect()
    }
}
