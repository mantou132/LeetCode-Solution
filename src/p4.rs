struct Solution {}

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len1 = nums1.len() as i32;
    let len2 = nums2.len() as i32;
    let mut min1_index = -1_i32;
    let mut max1_index = len1;
    let mut min2_index = -1_i32;
    let mut max2_index = len2;
    let mut min_value = 0;
    let mut max_value = 0;
    for _i in 0..(len1 + len2) / 2 + 1 {
      let next_min1_index = min1_index + 1;
      let next_min2_index = min2_index + 1;
      if nums1.get(next_min1_index as usize).is_none() {
        min2_index = next_min2_index;
        min_value = nums2[min2_index as usize];
      } else if nums2.get(next_min2_index as usize).is_none()
        || nums1[next_min1_index as usize] < nums2[next_min2_index as usize]
      {
        min1_index = next_min1_index;
        min_value = nums1[min1_index as usize];
      } else {
        min2_index = next_min2_index;
        min_value = nums2[min2_index as usize];
      }
      let next_max1_index = max1_index - 1;
      let next_max2_index = max2_index - 1;
      if nums1.get(next_max1_index as usize).is_none() {
        max2_index = next_max2_index;
        max_value = nums2[max2_index as usize];
      } else if nums2.get(next_max2_index as usize).is_none()
        || nums1[next_max1_index as usize] > nums2[next_max2_index as usize]
      {
        max1_index = next_max1_index;
        max_value = nums1[max1_index as usize];
      } else {
        max2_index = next_max2_index;
        max_value = nums2[max2_index as usize];
      }
    }
    if (len1 + len2) % 2 == 0 {
      (min_value + max_value) as f64 / 2.0
    } else {
      min_value as f64
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn find_median_sorted_arrays() {
    assert_eq!(
      super::Solution::find_median_sorted_arrays(vec![1, 1, 3, 3], vec![1, 1, 3, 3]),
      2.0
    );
    assert_eq!(
      super::Solution::find_median_sorted_arrays(vec![1, 2, 3], vec![1, 2]),
      2.0
    );
    assert_eq!(
      super::Solution::find_median_sorted_arrays(vec![1, 1, 1], vec![1, 1, 1]),
      1.0
    );
    assert_eq!(
      super::Solution::find_median_sorted_arrays(vec![3], vec![-2, -1]),
      -1.0
    );
    assert_eq!(
      super::Solution::find_median_sorted_arrays(vec![], vec![2]),
      2.0
    );
    assert_eq!(
      super::Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
      2.0
    );
    assert_eq!(
      super::Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]),
      2.5
    );
  }
}
