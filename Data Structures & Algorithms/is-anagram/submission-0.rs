use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut chars = HashMap::new();

        for c in s.chars() {
            match chars.get(&c) {
                Some(val)=> chars.insert(c, val+1),
                None => chars.insert(c, 1),
            };
        }

        for c in t.chars() {
            match chars.get(&c) {
                Some(val)=> {
                    if (val - 1 == 0) {
                        chars.remove(&c)
                    } else {
                        chars.insert(c, val-1)}
                    },
                None => return false,
            };
        }

        if chars.is_empty() {
            return true
        }

        return false;
    }
}