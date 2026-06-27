use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if (s.len() != t.len()) {
            return false;
        }
        
        let mut chars_in_s = HashMap::new();
        let mut chars_in_t = HashMap::new();

        for (c1, c2) in s.chars().zip(t.chars()) {
            *chars_in_s.entry(c1).or_insert(0) +=1;
            *chars_in_t.entry(c2).or_insert(0) +=1;
        }

        return chars_in_s == chars_in_t;
    }
}