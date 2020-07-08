struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let length = nums.len() as i32;
        let mut map = HashMap::new();
        for i in 0..length {
            map.insert(nums[i as usize], i);
        }
        for i in 0..length {
            let complement = target - nums[i as usize];
            match map.get(&complement) {
                Some(&n) if n != i => {
                    return vec![i, n];
                },
                _ => (),
            }
        }
        vec![0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![5, 4, 2, 3, 6], 10), vec![1,4]);
    }
}