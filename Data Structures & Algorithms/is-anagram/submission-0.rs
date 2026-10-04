use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        
        let mut s_count = HashMap::new();

        for c in s.chars() {
            *s_count.entry(c).or_insert(0) += 1;
        }

   
        for c in t.chars() {
            let counter = s_count.entry(c).or_insert(0);
            *counter -= 1;
            if *counter < 0 {
                return false
            }
            
        }
        
        return true
    }
}
