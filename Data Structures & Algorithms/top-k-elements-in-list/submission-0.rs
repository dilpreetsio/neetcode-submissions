impl Solution {
    pub fn top_k_frequent(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::new();
        let mut map:HashMap<i32,i32> = HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) +=1;
        }

        let mut map_vec: Vec<(&i32, &i32)> = map.iter().collect();

        map_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut i =0;
        for (key, value) in map_vec {
            ans.push(*key);
            i+=1;
            if (i ==k ) {
                break;
            }
        }
        ans
    }
}
