use std::collections::HashMap;


impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vec_of_maps: Vec<HashMap<u8, i32>> = Vec::new();
        let mut ans : Vec<Vec<String>> = Vec::new();
        let mut vec_of_strs: Vec<String> = strs.clone();
        for i in 0..vec_of_strs.len() {
            let s = vec_of_strs[i].clone();
            let mut char_map = HashMap::new();
            for c in s.chars() {
                *char_map.entry(c as u8).or_insert(0) +=1;
            }
            vec_of_maps.push(char_map);
        }

        let mut i = 0;
        while(i < vec_of_strs.len()) {
            let mut group = Vec::new();
            group.push(vec_of_strs[i].clone());

            let mut j = i+1;

            while(j < vec_of_maps.len()) {
                if (vec_of_maps[i] == vec_of_maps[j]) {
                    group.push(vec_of_strs[j].clone());
                    vec_of_maps.remove(j);
                    vec_of_strs.remove(j);
            
                } else {
                    j+=1;
                } 
            }
            ans.push(group);
            i+=1;
        }

        return ans;
    }
}
