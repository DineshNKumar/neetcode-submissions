impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
        let len = nums.len();

        if len <= 1 {
            return false;
        }

        nums.sort_unstable();

        let mut i = 0;
        let mut j = 1;

        while i < len && j < len {
            if nums[i] == nums[j] {
                return true;
            }
            
            i += 1;
            j += 1;
        }


        return false;
    }

}