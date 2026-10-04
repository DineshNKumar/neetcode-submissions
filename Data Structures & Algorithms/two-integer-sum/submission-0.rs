impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
    
        for (i, &n) in nums.iter().enumerate() {
            let complement = target - n;

            if let Some(j) = hash.get(&complement) {
                return vec![*j as i32, i as i32];
            }
            
            hash.insert(n, i);
        }
        
        vec![]
    }
}
