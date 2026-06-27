impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        if (nums.len() < 1) {
           return false;

        }
        let mut arr = nums.clone();
        arr.sort();

        for i in 0..arr.len() - 1 {
            if (arr[i] == arr[i+1]) {
                return true;
            }
        }
        false
    }
}
